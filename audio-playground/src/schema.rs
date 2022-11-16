use std::fs;
use std::fs::read_to_string;
use std::fs::Metadata;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::reader::{get_duration_for_path, get_track_from_path};
use crate::search_query::{convert_bm25_order, convert_int_order, create_query, do_search};
use crate::utils::{
    self, adapt_text, file_ext, genre_string_to_vec, get_order_field, is_valid_facet, norm,
    ALLOWED_FILE_TYPES,
};
use audiotags::AudioTag;
use chrono::{DateTime, NaiveDateTime, Utc};
use id3::TagLike;
use jwalk::DirEntry;
use jwalk::WalkDir;
use serde::{Deserialize, Serialize};
use slug::slugify;
use tantivy::collector::Count;
use tantivy::collector::FacetCollector;
use tantivy::collector::MultiCollector;
use tantivy::collector::TopDocs;
use tantivy::query::AllQuery;
use tantivy::query::Query;
use tantivy::query::QueryParser;
use tantivy::time::PrimitiveDateTime;
use tantivy::{
    collector::FacetCounts,
    schema::{
        Cardinality, Facet, FacetOptions, Field, IndexRecordOption, NumericOptions, Schema,
        TextFieldIndexing, TextOptions, Value, FAST, STORED, STRING,
    },
    DocAddress, Document, Index, IndexReader, IndexWriter, TantivyError,
};

pub struct SearchWatcher {
    pub field_schema: FieldSchema,
    pub index: Index,
    pub reader: IndexReader,
    pub writer: Arc<Mutex<IndexWriter>>,
}

const JSON_DATA_FILE: &str = "./data/audio.json";

const BASE_AUDIO_DIRECTORY: &str =
    "C:\\Users\\lukes\\Github\\rust-adventures\\audio-playground\\audio";

impl SearchWatcher {
    pub fn new(index_cache_directory: &str) -> Self {
        let field_schema = FieldSchema::new();

        let index_path: &Path = Path::new(index_cache_directory);
        let index;
        if index_path.exists() {
            index = Index::open_in_dir(&index_path).ok().unwrap();
        } else {
            fs::create_dir(index_path).ok();
            index = Index::create_in_dir(&index_path, field_schema.schema.clone())
                .ok()
                .unwrap();
        }

        let writer = Arc::new(Mutex::new(
            index.writer_with_num_threads(2, 140_000_000).unwrap(),
        ));

        let reader = index
            .reader_builder()
            .reload_policy(tantivy::ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        SearchWatcher {
            field_schema,
            index,
            reader,
            writer,
        }
    }
    pub fn search(&self, request: DocumentSearchRequest) -> tantivy::Result<()> {
        let faced_only_flag = true;
        let response: DocumentSearchResponse = self.do_search(&request, faced_only_flag);

        println!("Total {} items", response.total);
        for item in response.results {
            println!("🎵 {} - ({})", item.track.name, item.track.id);
        }
        // let response_json = serde_json::to_string(&response)?;

        Ok(())
    }
    pub fn do_search(
        &self,
        request: &DocumentSearchRequest,
        facet_only_flag: bool,
    ) -> DocumentSearchResponse {
        let query_parser = {
            let query_parser = QueryParser::for_index(
                &self.index,
                vec![
                    self.field_schema.title,
                    self.field_schema.artist,
                    self.field_schema.album,
                    self.field_schema.track,
                ],
            );
            query_parser
        };
        let text = adapt_text(&query_parser, &request.text);

        let query = if !request.text.is_empty() {
            create_query(&query_parser, request, &self.field_schema, &text)
        } else {
            Box::new(AllQuery) as Box<dyn Query>
        };

        // Offset to search from
        let results = request.result_per_page as usize;

        let offset = results * request.page_number as usize;

        let extra_result = results + 1;
        let order_field = get_order_field(&self.field_schema, &request.order);
        let facets = request
            .faceted
            .as_ref()
            .map(|v| {
                v.tags
                    .iter()
                    .filter(|s| is_valid_facet(*s))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        let mut facet_collector = FacetCollector::for_field(self.field_schema.facets);
        for facet in &facets {
            match Facet::from_text(facet) {
                Ok(facet) => facet_collector.add_facet(facet),
                Err(_) => println!("Invalid facet: {}", facet),
            }
        }

        let searcher = self.reader.searcher();

        // TODO: use request.filters to filter by year range and date ranges

        match order_field {
            _ if !facet_only_flag => {
                // Just a facet search
                let facets_count = searcher.search(&query, &facet_collector).unwrap();
                convert_bm25_order(
                    self.field_schema.clone(),
                    SearchResponse {
                        facets,
                        query: &text,
                        top_docs: vec![],
                        facets_count,
                        order_by: request.order.clone(),
                        page_number: request.page_number,
                        results_per_page: results as i32,
                    },
                    &searcher,
                )
            }
            Some(order_field) => {
                let mut multicollector = MultiCollector::new();
                let facet_handler = multicollector.add_collector(facet_collector);
                let count_handler = multicollector.add_collector(Count);

                let topdocs_collector = TopDocs::with_limit(extra_result)
                    .and_offset(offset)
                    .order_by_u64_field(order_field);
                let topdocs_handler = multicollector.add_collector(topdocs_collector);
                let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
                let facets_count = facet_handler.extract(&mut multi_fruit);
                let top_docs = topdocs_handler.extract(&mut multi_fruit);

                let count = count_handler.extract(&mut multi_fruit);

                convert_int_order(
                    self.field_schema.clone(),
                    SearchResponse {
                        facets_count,
                        facets,
                        top_docs,
                        query: &text,
                        order_by: request.order.clone(),
                        page_number: request.page_number,
                        results_per_page: results as i32,
                    },
                    &searcher,
                )
            }
            None => {
                let mut multicollector = MultiCollector::new();
                let facet_handler = multicollector.add_collector(facet_collector);
                let topdocs_collector = TopDocs::with_limit(extra_result).and_offset(offset);
                let topdocs_handler = multicollector.add_collector(topdocs_collector);
                let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
                let facets_count = facet_handler.extract(&mut multi_fruit);
                let top_docs = topdocs_handler.extract(&mut multi_fruit);

                convert_bm25_order(
                    self.field_schema.clone(),
                    SearchResponse {
                        facets_count,
                        facets,
                        top_docs,
                        query: &text,
                        order_by: request.order.clone(),
                        page_number: request.page_number,
                        results_per_page: results as i32,
                    },
                    &searcher,
                )
            }
        }
    }

    pub fn is_existing_by_path(&self, track_path: &str) -> bool {
        let query_parser = {
            let query_parser =
                QueryParser::for_index(&self.index, vec![self.field_schema.abs_path]);
            query_parser
        };

        let searcher = self.reader.searcher();

        let path_query = format!("\"{}\"", &track_path);
        let query = query_parser.parse_query(path_query.as_str()).unwrap();
        let count = searcher.search(&query, &Count).unwrap();

        count > 0
    }

    pub fn add(&self, item: &TrackJson) {
        // quick duplicate check
        if self.is_existing_by_path(&item.abs_path) {
            return;
        }

        let mut document = Document::default();
        document.add_text(self.field_schema.id, &item.id);
        document.add_text(self.field_schema.abs_path, &item.abs_path);
        document.add_text(self.field_schema.title, &item.name);
        document.add_text(self.field_schema.track, &item.track);
        document.add_text(self.field_schema.album, &item.album);
        document.add_text(self.field_schema.artist, &item.artist);
        document.add_text(self.field_schema.genre, &item.genre);
        document.add_u64(self.field_schema.year, item.year as u64);
        document.add_i64(self.field_schema.size, item.size);

        let date_time_value: tantivy::DateTime =
            tantivy::DateTime::from_unix_timestamp(item.created_date / 1000);
        document.add_date(self.field_schema.created_date, date_time_value);

        let date_time_modified_value: tantivy::DateTime =
            tantivy::DateTime::from_unix_timestamp(item.modified_date / 1000);
        document.add_date(self.field_schema.modified_date, date_time_modified_value);

        let date_time_indexed_value: tantivy::DateTime =
            tantivy::DateTime::from_unix_timestamp(item.indexed_date / 1000);
        document.add_date(self.field_schema.indexed_date, date_time_indexed_value);

        let facet_album_string = format!("/album/{}", &item.album);
        document.add_facet(self.field_schema.facets, Facet::from(&facet_album_string));

        let facet_artist_string = format!("/artist/{}", &item.artist);
        document.add_facet(self.field_schema.facets, Facet::from(&facet_artist_string));

        let facet_year_string = format!("/year/{}", &item.year);
        document.add_facet(self.field_schema.facets, Facet::from(&facet_year_string));

        for genre in &item.genres {
            let facet_string = format!("/genre/{}", &genre);
            document.add_facet(self.field_schema.facets, Facet::from(&facet_string));
        }

        if let Some(d) = get_duration_for_path(&item.abs_path) {
            document.add_f64(self.field_schema.duration, d);
        }

        self.writer.lock().unwrap().add_document(document).unwrap();
    }
    pub fn initial_index_from_json(&self, json_file_path: &str) {
        // Read JSON from file
        let json_file_path_as_path = Path::new(json_file_path);
        let json_file_str = read_to_string(json_file_path_as_path).expect("file not found");
        let data: Vec<TrackJson> = serde_json::from_str(&json_file_str).unwrap();

        println!("Indexing {} items", data.len());
        for item in data.iter() {
            self.add(item);
        }
        println!("Total {} items indexed", data.len());

        self.writer.lock().unwrap().commit().unwrap();
    }
    pub fn index_since_last_opened(&self) {
        let path = &norm(BASE_AUDIO_DIRECTORY).to_string();
        let start = SystemTime::now();

        // TODO: pull from locally stored config (LAST_UPDATED)
        let last_opened: u128 = 1665410457180;
        let now: u128 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        println!("compare now {} to last opened {} ", &now, &last_opened);

        let mut paths: Vec<String> = vec![];
        let mut cnt = 0;

        let mut generic = WalkDir::new(&path);
        generic = generic.process_read_dir(move |_depth, _path, _read_dir_state, children| {
            children.iter_mut().for_each(|dir_entry_result| {
                if let Ok(dir_entry) = dir_entry_result {
                    let modified = dir_entry
                        .metadata()
                        .unwrap()
                        .modified()
                        .unwrap()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    // check if this file should be indexed (or at least checked) given the last indexed date
                    if last_opened < modified {
                        println!("✅ new file - please index");
                        norm(dir_entry.path().to_str().unwrap_or(""));
                    } else {
                        println!("⏭️ should have already indexed this file");
                    }
                }
            });
        });
        println!("paths {:?} ", &paths);

        for entry in generic {
            cnt += 1;
            if entry.is_err() {
                continue;
            }

            let en: DirEntry<((), ())> = entry.unwrap();
            let buf = en.path();
            let file_type = en.file_type();
            let is_dir = file_type.is_dir();

            let path_string = buf.to_str().unwrap().to_string();
            let name = en.file_name().to_str().unwrap();
            let ext = file_ext(name);

            if !is_dir & ALLOWED_FILE_TYPES.contains(&ext) {
                if let Some(track) = get_track_from_path(&path_string) {
                    self.add(&track)
                }
                // else {
                //     tracks_failed.push(path_string)
                // }
            }
        }

        // println!("Failed to index {} tracks", &tracks_failed.len());

        let end = SystemTime::now();
        println!(
            "cost {}ms, total {} files",
            end.duration_since(start).unwrap().as_millis(),
            cnt
        );

        self.writer.lock().unwrap().commit().unwrap();

        // TODO: on success, set the locally stored config for LAST_UPDATED
    }
}

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
        let mut sb = Schema::builder();

        let text_field_indexing =
            TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_stored()
            .set_indexing_options(text_field_indexing)
            .set_fast();

        let num_options: NumericOptions = NumericOptions::default()
            .set_stored()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let date_options = NumericOptions::default()
            .set_stored()
            .set_indexed()
            .set_fast(Cardinality::SingleValue);

        let id = sb.add_bytes_field("id", STORED | FAST);
        let abs_path = sb.add_text_field("abs_path", STRING | STORED);
        let size = sb.add_i64_field("size", num_options.clone());
        let title = sb.add_text_field("title", text_options.clone());
        let track = sb.add_text_field("track", text_options.clone());
        let artist = sb.add_text_field("artist", text_options.clone());
        let album = sb.add_text_field("album", text_options.clone());
        let genre = sb.add_text_field("genre", text_options.clone());
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
        let abs_path = doc
            .get_first(field_schema.abs_path)
            .and_then(Value::as_text)
            .unwrap_or("")
            .to_string();

        let duration = get_duration_for_path(&abs_path).unwrap_or(0.0);

        let track_json_option = get_track_from_path(&abs_path);

        if track_json_option.is_none() {
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

        let now_date_time: tantivy::DateTime =
            tantivy::DateTime::from_primitive(PrimitiveDateTime::MIN);

        // Dates
        let indexed_date: i64 = doc
            .get_first(field_schema.indexed_date)
            .and_then(Value::as_date)
            .unwrap_or(now_date_time)
            .into_unix_timestamp();

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
