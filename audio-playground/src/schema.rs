use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::reader::{get_duration_for_path, get_track_from_path};
use crate::utils::{self, genre_string_to_vec};
use audiotags::AudioTag;
use chrono::{DateTime, Utc};
use id3::TagLike;
use serde::{Deserialize, Serialize};
use slug::slugify;
use tantivy::{
    collector::FacetCounts,
    fastfield::FastValue,
    schema::{
        Cardinality, FacetOptions, Field, IndexRecordOption, NumericOptions, Schema,
        TextFieldIndexing, TextOptions, Value, STORED, STRING,
    },
    DocAddress, Document,
};

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub schema: Schema,

    pub id: Field,
    pub title: Field,
    pub abs_path: Field,
    pub size: Field,
    pub created_date: Field,
    pub modified_date: Field,
    pub indexed_date: Field,
    pub status: Field,
    pub facets: Field,
    pub track: Field,
    pub artist: Field,
    pub album: Field,
    pub year: Field,
    pub genre: Field,
    pub duration: Field,
}

impl FieldSchema {
    pub fn new() -> Self {
        println!("FieldSchema::new");
        let mut sb = Schema::builder();

        let text_field_indexing =
            TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqs);
        let text_options = TextOptions::default().set_indexing_options(text_field_indexing);

        let num_options: NumericOptions = NumericOptions::default()
            .set_stored()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let date_options = NumericOptions::default()
            .set_stored()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let id = sb.add_text_field("id", STRING | STORED);
        let abs_path = sb.add_text_field("abs_path", STRING | STORED);
        let size = sb.add_i64_field("size", num_options.clone());
        let title = sb.add_text_field("title", STRING | STORED);
        let track = sb.add_text_field("track", STRING | STORED);
        let artist = sb.add_text_field("artist", STRING | STORED);
        let album = sb.add_text_field("album", STRING | STORED);
        let genre = sb.add_text_field("genre", STRING | STORED);
        let duration = sb.add_f64_field("duration", num_options.clone());
        let year = sb.add_u64_field("year", num_options.clone());

        // Dates
        let created_date = sb.add_date_field("created_date", date_options.clone());
        let modified_date = sb.add_date_field("modified_date", date_options.clone());
        let indexed_date = sb.add_date_field("indexed_date", date_options);

        // Status
        let status = sb.add_u64_field("status", num_options);

        // Facets (artist, album, year and genre)
        let facets = sb.add_facet_field("facets", FacetOptions::default().set_stored());

        let schema = sb.build();

        println!("FieldSchema::new end");

        FieldSchema {
            schema,
            id,
            abs_path,
            size,
            title,
            created_date,
            modified_date,
            indexed_date,
            status,
            facets,
            track,
            artist,
            album,
            year,
            genre,
            duration,
        }
    }
}

impl Default for FieldSchema {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Track {
    pub id: String,
    pub abs_path: String,
    pub created_date: i64,
    pub modified_date: i64,
    pub indexed_date: i64,
    pub size: i64,
    pub album: String,
    pub artist: String,
    pub genres: Vec<String>,
    pub name: String,
    pub track: String,
    pub year: u64,
    pub duration: f64,
    pub exists: bool,
}

impl Track {
    pub fn with_document(field_schema: &FieldSchema, doc: Document) -> Self {
        println!("with_document doc {:?} ", doc);

        let abs_path = doc
            .get_first(field_schema.abs_path)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();

        let duration = get_duration_for_path(&abs_path).unwrap_or(0.0);

        let track_json_option = get_track_from_path(&abs_path);

        if (track_json_option.is_none()) {
            return Track {
                id: "".to_string(),
                abs_path: "".to_string(),
                size: 0,
                created_date: 0,
                modified_date: 0,
                indexed_date: 0,
                album: "".to_string(),
                artist: "".to_string(),
                name: "".to_string(),
                track: "".to_string(),
                year: 0,
                genres: vec![],
                duration: 0.0,
                exists: false,
            };
        }
        let track_json = track_json_option.unwrap();

        let id = doc
            .get_first(field_schema.id)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();

        let genres = track_json.genres;
        let size = track_json.size;
        let album = track_json.album;
        let artist = track_json.artist;
        let name = track_json.name;
        let track = track_json.track;
        let year = track_json.year;

        let created_date = track_json.created_date;
        let modified_date = track_json.modified_date;

        let now_date_time: &DateTime<Utc> = &chrono::DateTime::<Utc>::MIN_UTC;

        // Dates
        let indexed_date: i64 = doc
            .get_first(field_schema.indexed_date)
            .and_then(Value::as_date)
            .unwrap_or(now_date_time)
            .timestamp();

        Track {
            id,
            abs_path,
            size,
            created_date,
            modified_date,
            indexed_date,
            album,
            artist,
            name,
            track,
            year,
            genres,
            duration,
            exists: true,
        }

        // Track {
        //     id: "".to_string(),
        //     abs_path: "".to_string(),
        //     size: 0,
        //     created_date: 0,
        //     modified_date: 0,
        //     indexed_date: 0,
        //     album: "".to_string(),
        //     artist: "".to_string(),
        //     name: "".to_string(),
        //     track: "".to_string(),
        //     year: 0,
        //     genres: vec![],
        // }
    }
}

impl TrackJson {
    pub fn new_wav(path: String, meta: Metadata, tag: id3::Tag) -> Self {
        let abs_path = utils::norm(&path.clone());

        #[cfg(windows)]
        let size = meta.file_size() as i64;
        #[cfg(unix)]
        let size = meta.size() as i64;

        let name = utils::path2name(path.clone());

        let created_date = meta
            .created()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let modified_date = meta
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let indexed_date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // create a unique id to check for existing index items
        let id = slugify(format!("{}-{}", &created_date, name.clone()));

        let track = tag.title().unwrap_or("untitled").to_string();
        let artist = tag.artist().unwrap_or("untitled").to_string();
        let album = tag.album().unwrap_or("untitled").to_string();
        let genre = tag.genre().unwrap_or("").to_string();
        let year: u64 = tag.year().unwrap_or(0) as u64;

        // NOTE: we're not using tag.duration() as this queries for the ID3 value which is usually null
        // instead we will query for the duration at indexing run time
        let duration: f64 = 0.0;

        // Genre
        let genres = genre_string_to_vec(&genre);

        TrackJson {
            id,
            abs_path,
            created_date,
            modified_date,
            indexed_date,
            size,
            album,
            artist,
            genre,
            genres,
            name,
            track,
            duration,
            year,
        }
    }
    pub fn new(path: String, meta: Metadata, tag: Box<dyn AudioTag>) -> Self {
        let abs_path = utils::norm(&path.clone());

        #[cfg(windows)]
        let size = meta.file_size() as i64;
        #[cfg(unix)]
        let size = meta.size() as i64;

        let name = utils::path2name(path.clone());

        let created_date = meta
            .created()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let modified_date = meta
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let indexed_date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // create a unique id to check for existing index items
        let id = slugify(format!("{}-{}", &created_date, name.clone()));

        let track = tag.title().unwrap_or("untitled").to_string();
        let artist = tag.artist().unwrap_or("untitled").to_string();
        let album = tag.album_title().unwrap_or("untitled").to_string();
        let genre = tag.genre().unwrap_or("").to_string();
        let year: u64 = tag.year().unwrap_or(0) as u64;

        // NOTE: we're not using tag.duration() as this queries for the ID3 value which is usually null
        // instead we will query for the duration at indexing run time
        let duration: f64 = 0.0;

        // Genre
        let genres = genre_string_to_vec(&genre);

        TrackJson {
            id,
            abs_path,
            created_date,
            modified_date,
            indexed_date,
            size,
            album,
            artist,
            genre,
            genres,
            name,
            track,
            duration,
            year,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TrackJson {
    pub id: String,
    pub abs_path: String,
    pub created_date: i64,
    pub modified_date: i64,
    pub indexed_date: i64,
    pub size: i64,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub genres: Vec<String>,
    pub name: String,
    pub track: String,
    pub duration: f64,
    pub year: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderType {
    Desc = 0,
    Asc = 1,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Filters {
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub created_date_start: Option<i32>,
    pub created_date_end: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Faceted {
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrderBy {
    pub field: String,
    pub order_type: OrderType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentSearchRequest {
    pub text: String,
    pub fields: Vec<String>,
    pub filters: Filters,
    pub order: Option<OrderBy>,
    pub faceted: Option<Faceted>,
    pub page_number: i32,
    pub result_per_page: i32,
    pub reload: bool,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FacetResult {
    pub tag: String,
    pub total: i32,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct FacetResults {
    pub facet_results: Vec<FacetResult>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct ResultScore {
    pub bm25: f32,
    // In the case of two equal bm25 scores, booster decides
    pub booster: f32,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct DocumentResult {
    pub score: Option<ResultScore>,
    pub track: Track,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct DocumentSearchResponse {
    pub total: i32,
    pub results: Vec<DocumentResult>,
    pub facets: ::std::collections::HashMap<String, FacetResults>,
    pub page_number: i32,
    pub result_per_page: i32,
    pub query: String,
    /// Is there a next page
    pub next_page: bool,
    pub bm25: bool,
}

pub struct SearchResponse<'a, S> {
    pub query: &'a str,
    pub facets_count: FacetCounts,
    pub facets: Vec<String>,
    pub top_docs: Vec<(S, DocAddress)>,
    pub order_by: Option<OrderBy>,
    pub page_number: i32,
    pub results_per_page: i32,
}
