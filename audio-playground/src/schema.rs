use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils;
use audiotags::AudioTag;
use serde::{Deserialize, Serialize};
use tantivy::schema::{
    Cardinality, FacetOptions, Field, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing,
    TextOptions, STORED, STRING,
};

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub schema: Schema,

    pub uuid: Field,
    pub title: Field,
    pub created: Field,
    pub modified: Field,
    pub status: Field,
    pub facets: Field,
    pub track: Field,
    pub artist: Field,
    pub album: Field,
    pub year: Field,
    pub created_date: Field,
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

        let uuid = sb.add_text_field("uuid", STRING | STORED);
        let title = sb.add_text_field("title", text_options.clone());
        let track = sb.add_text_field("track", STRING | STORED);
        let artist = sb.add_text_field("artist", STRING | STORED);
        let album = sb.add_text_field("album", STRING | STORED);
        let year = sb.add_u64_field("year", num_options.clone());

        let created_date = sb.add_date_field("created_date", num_options.clone());

        // Date fields needs to be searched in order, order_by_u64_field seems to work in TopDocs.
        let created = sb.add_date_field("created", date_options.clone());
        let modified = sb.add_date_field("modified", date_options);

        // Status
        let status = sb.add_u64_field("status", num_options);

        // Facets (artist, album, year and genre)
        let facets = sb.add_facet_field("facets", FacetOptions::default().set_stored());

        let schema = sb.build();

        FieldSchema {
            schema,
            uuid,
            title,
            created,
            modified,
            status,
            facets,
            track,
            artist,
            album,
            year,
            created_date,
        }
    }
}

impl Default for FieldSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl TrackJson {
    pub fn new(path: String, meta: Metadata, tag: Box<dyn AudioTag>) -> Self {
        let abs_path = utils::norm(&path.clone());
        let is_dir = false;

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

        let mod_at = meta
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let track = tag.title().unwrap().to_string();
        let artist = tag.artist().unwrap().to_string();
        let album = tag.album_title().unwrap().to_string();
        let year = tag.year().unwrap().to_string();
        let genres = [].to_vec(); // tag.genre().unwrap().to_string()

        TrackJson {
            abs_path,
            created_date,
            size,
            mod_at,
            is_dir,
            album,
            artist,
            genres,
            name,
            track,
            year,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackJson {
    pub abs_path: String,
    pub created_date: i64,
    pub size: i64,
    pub mod_at: i64,
    pub is_dir: bool,
    pub album: String,
    pub artist: String,
    pub genres: Vec<String>,
    pub name: String,
    pub track: String,
    pub year: String,
}

#[derive(Clone, PartialEq)]
pub enum OrderType {
    Desc = 0,
    Asc = 1,
}

#[derive(Clone, PartialEq)]
pub struct Filter {
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct Faceted {
    pub tags: Vec<String>,
}

#[derive(Clone, PartialEq)]
pub struct OrderBy {
    pub field: String,
    pub order_type: OrderType,
}

#[derive(Clone, PartialEq)]
pub struct DocumentSearchRequest {
    pub id: String,
    // pub body: String,
    pub fields: Vec<String>,
    pub filter: Option<Filter>,
    // pub order: Option<OrderBy>,
    // pub faceted: Option<Faceted>,
    // pub page_number: i32,
    // pub result_per_page: i32,
    // pub reload: bool
}

#[derive(Clone, PartialEq)]
pub struct PokemonSearchRequest {
    pub id: String,
    // pub body: String,
    pub types: Vec<String>,
    pub genres: Filter,
    // pub order: Option<OrderBy>,
    // pub faceted: Option<Faceted>,
    // pub page_number: i32,
    // pub result_per_page: i32,
    // pub reload: bool
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

#[derive(Clone, PartialEq)]
pub struct DocumentSearchResponse {
    pub total: i32,
    // pub results: Vec<FieldSchema>,
    pub facets: ::std::collections::HashMap<String, FacetResults>,
    pub page_number: i32,
    pub result_per_page: i32,
    pub query: String,
    /// Is there a next page
    pub next_page: bool,
    // pub bm25: bool,
}
