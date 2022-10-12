use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::{self, genre_string_to_vec};
use audiotags::AudioTag;
use serde::{Deserialize, Serialize};
use tantivy::{
    collector::FacetCounts,
    schema::{
        Cardinality, FacetOptions, Field, IndexRecordOption, NumericOptions, Schema,
        TextFieldIndexing, TextOptions, Value, STORED, STRING,
    },
    DocAddress, Document,
};

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub schema: Schema,

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
        let mut sb = Schema::builder();

        let text_field_indexing =
            TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqs);
        let text_options = TextOptions::default().set_indexing_options(text_field_indexing);

        let num_options: NumericOptions = NumericOptions::default()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let date_options = NumericOptions::default()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let abs_path = sb.add_text_field("abs_path", STRING | STORED);
        let size = sb.add_i64_field("size", num_options.clone());
        let title = sb.add_text_field("title", text_options.clone());
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

        FieldSchema {
            schema,
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
    pub abs_path: String,
    pub created_date: i64,
    pub size: i64,
    pub modified_date: i64,
    pub album: String,
    pub artist: String,
    pub genres: Vec<String>,
    pub name: String,
    pub track: String,
    pub year: i32,
}

impl Track {
    pub fn with_document(field_schema: &FieldSchema, doc: Document) -> Self {
        println!("Track.with_document");

        let genre_string = doc
            .get_first(field_schema.genre)
            .and_then(Value::as_text)
            .unwrap_or("");
        let genres = genre_string_to_vec(genre_string);

        let abs_path = doc
            .get_first(field_schema.abs_path)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();
        let size = doc
            .get_first(field_schema.size)
            .and_then(Value::as_i64)
            .unwrap_or(0000) as i64;
        let created_date = doc
            .get_first(field_schema.created_date)
            .and_then(Value::as_i64)
            .unwrap_or(0000) as i64;
        let modified_date = doc
            .get_first(field_schema.modified_date)
            .and_then(Value::as_i64)
            .unwrap_or(0000) as i64;
        let album = doc
            .get_first(field_schema.album)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();
        let artist = doc
            .get_first(field_schema.artist)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();
        let name = doc
            .get_first(field_schema.title)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();
        let track = doc
            .get_first(field_schema.track)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();
        let year = doc
            .get_first(field_schema.year)
            .and_then(Value::as_i64)
            .unwrap_or(0000) as i32;

        Track {
            abs_path,
            size,
            created_date,
            modified_date,
            album,
            artist,
            name,
            track,
            year,
            genres,
        }
    }
}

impl TrackJson {
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
            .as_secs() as i64;

        let modified_date = meta
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let indexed_date = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let track = tag.title().unwrap_or("untitled").to_string();
        let artist = tag.artist().unwrap_or("untitled").to_string();
        let album = tag.album_title().unwrap_or("untitled").to_string();
        let genre = tag.genre().unwrap_or("").to_string();
        let duration: f64 = tag.duration().unwrap_or(0.0);
        let year: i32 = tag.year().unwrap_or(0);

        // Genre
        let genres = genre_string_to_vec(&genre);

        TrackJson {
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
    pub year: i32,
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

#[derive(Debug, Clone, PartialEq)]
pub struct FacetResult {
    pub tag: String,
    pub total: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FacetResults {
    pub facet_results: Vec<FacetResult>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResultScore {
    pub bm25: f32,
    // In the case of two equal bm25 scores, booster decides
    pub booster: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocumentResult {
    pub score: Option<ResultScore>,
    pub track: Track,
}

#[derive(Clone, Debug, PartialEq)]
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
