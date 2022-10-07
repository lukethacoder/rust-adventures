use tantivy::chrono::{DateTime, NaiveDateTime, Utc};
use tantivy::schema::{
    Cardinality, FacetOptions, Field, IndexRecordOption, NumericOptions, Schema, TextFieldIndexing,
    TextOptions, STORED, STRING, TEXT,
};

#[derive(Debug, Clone)]
pub struct FieldSchema {
    pub schema: Schema,

    pub uuid: Field,
    pub title: Field,
    pub created: Field,
    pub modified: Field,
    pub status: Field,
    pub artist: Field,
    pub album: Field,
    pub genres: Field,
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

        let facet_options = FacetOptions::default().set_stored();

        let uuid = sb.add_text_field("uuid", STRING | STORED);
        let title = sb.add_text_field("title", text_options.clone());
        let artist = sb.add_text_field("artist", STRING | STORED);
        let album = sb.add_text_field("album", STRING | STORED);

        // Date fields needs to be searched in order, order_by_u64_field seems to work in TopDocs.
        let created = sb.add_date_field("created", date_options.clone());
        let modified = sb.add_date_field("modified", date_options);

        // Status
        let status = sb.add_u64_field("status", num_options);

        // Facets
        let genres = sb.add_facet_field("genres", facet_options);

        let schema = sb.build();

        FieldSchema {
            schema,
            uuid,
            title,
            created,
            modified,
            status,
            artist,
            album,
            genres,
        }
    }
}

impl Default for FieldSchema {
    fn default() -> Self {
        Self::new()
    }
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
