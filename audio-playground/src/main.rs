use std::fs;
use std::fs::read_to_string;
use std::io;

use std::path::Path;
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use audiotags::Tag;
use chrono::NaiveDateTime;
use id3;
use mpeg_audio_header::{Header, ParseMode};
use tantivy::chrono::Utc;
use tantivy::{schema::*, DateTime, Index, IndexWriter, TantivyError};

use jwalk::{DirEntry, WalkDir};

mod reader;
mod schema;
mod search_query;
mod utils;

use crate::reader::{get_duration_for_path, get_track_from_path};
use crate::schema::{
    DocumentSearchRequest, DocumentSearchResponse, Faceted, FieldSchema, Filters, OrderBy,
    OrderType, SearchWatcher, TrackJson,
};
use crate::search_query::do_search;
use crate::utils::{file_ext, norm};

const JSON_DATA_FILE: &str = "./data/audio.json";

// "C:\\Users\\lukes\\Music"
// "E:\\Music";
const BASE_AUDIO_DIRECTORY: &str =
    "C:\\Users\\lukes\\Github\\rust-adventures\\audio-playground\\audio";

const INDEX_CACHE_DIRECTORY: &str =
    "C:\\Users\\lukes\\Github\\rust-adventures\\audio-playground\\.index-cache";

fn main() -> tantivy::Result<()> {
    if false {
        // Fetch audio data and save to the local JSON file
        walk(&norm(BASE_AUDIO_DIRECTORY).to_string());
    }

    if false {
        search();
    }

    if true {
        watch_search();
    }

    Ok(())
}

fn index_data(
    field_schema: &FieldSchema,
    mut index_writer: IndexWriter,
    json_file_path: &str,
) -> Result<(), TantivyError> {
    // Read JSON from file
    let json_file_path_as_path = Path::new(json_file_path);
    let json_file_str = read_to_string(json_file_path_as_path).expect("file not found");
    let data: Vec<TrackJson> = serde_json::from_str(&json_file_str).unwrap();

    println!("Total {} items", data.len());

    for item in data.iter() {
        let mut document = Document::default();
        document.add_text(field_schema.id, &item.id);
        document.add_text(field_schema.abs_path, &item.abs_path);
        document.add_text(field_schema.title, &item.name);
        document.add_text(field_schema.track, &item.track);
        document.add_text(field_schema.album, &item.album);
        document.add_text(field_schema.artist, &item.artist);
        document.add_text(field_schema.genre, &item.genre);
        document.add_u64(field_schema.year, item.year as u64);
        document.add_i64(field_schema.size, item.size);

        let date_time_value = DateTime::from_utc(
            NaiveDateTime::from_timestamp(item.created_date / 1000, 0),
            Utc,
        );
        document.add_date(field_schema.created_date, date_time_value);

        let date_time_modified_value = DateTime::from_utc(
            NaiveDateTime::from_timestamp(item.modified_date / 1000, 0),
            Utc,
        );
        document.add_date(field_schema.modified_date, date_time_modified_value);

        let date_time_indexed_value = DateTime::from_utc(
            NaiveDateTime::from_timestamp(item.indexed_date / 1000, 0),
            Utc,
        );
        document.add_date(field_schema.indexed_date, date_time_indexed_value);

        let facet_album_string = format!("/album/{}", &item.album);
        document.add_facet(field_schema.facets, Facet::from(&facet_album_string));

        let facet_artist_string = format!("/artist/{}", &item.artist);
        document.add_facet(field_schema.facets, Facet::from(&facet_artist_string));

        let facet_year_string = format!("/year/{}", &item.year);
        document.add_facet(field_schema.facets, Facet::from(&facet_year_string));

        for genre in &item.genres {
            let facet_string = format!("/genre/{}", &genre);
            document.add_facet(field_schema.facets, Facet::from(&facet_string));
        }

        if let Some(d) = get_duration_for_path(&item.abs_path) {
            document.add_f64(field_schema.duration, d);
        }

        index_writer.add_document(document)?;
    }

    index_writer.commit()?;

    Ok(())
}

fn watch_search() {
    let search_watcher = SearchWatcher::new(INDEX_CACHE_DIRECTORY);
    search_watcher.initial_index(JSON_DATA_FILE);

    println!("Enter a search term...\n");
    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                println!("🔎 searching for {:?}\n", line);

                // Query Variables (will be passed to the method from a FE interface of some sort)
                let mut text: String = "".to_string();
                if line.len() > 0 {
                    text = format!("\"{}\"", line);
                }
                let faceted = Faceted {
                    tags: vec![
                        "/genre".to_string(),
                        "/year".to_string(),
                        "/album".to_string(),
                        "/artist".to_string(),
                    ],
                };

                // Order by
                let order_by_object: OrderBy = OrderBy {
                    field: "created_date".to_string(),
                    order_type: OrderType::Desc,
                };
                let order_by: Option<OrderBy> = Some(order_by_object);

                let faced_only_flag = true;

                let request = DocumentSearchRequest {
                    text,
                    fields: vec!["body".to_string()],
                    filters: Filters {
                        year_start: None,
                        year_end: None,
                        created_date_start: None,
                        created_date_end: None,
                    },
                    faceted: Some(faceted.clone()),
                    order: order_by,
                    page_number: 0,
                    result_per_page: 10,
                    reload: false,
                };

                search_watcher.search(request);
                println!("\nSearch again? ...\n");
            }
            Err(err) => println!("IO error: {}", err),
        }
    }
}

fn search() -> tantivy::Result<()> {
    let start = SystemTime::now();

    let field_schema: FieldSchema = FieldSchema::new();

    let index_path: &Path = Path::new(INDEX_CACHE_DIRECTORY);
    let index: Index;
    if index_path.exists() {
        index = Index::open_in_dir(&index_path).ok().unwrap();
    } else {
        fs::create_dir(index_path).ok();
        index = Index::create_in_dir(&index_path, field_schema.schema.clone())
            .ok()
            .unwrap();
    }

    // let index = Index::create_in_ram(field_schema.schema.clone());
    let index_writer: IndexWriter = index.writer(30_000_000)?;

    index_data(&field_schema, index_writer, JSON_DATA_FILE)?;
    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    // Query Variables (will be passed to the method from a FE interface of some sort)
    let text = "Eminem".to_string();
    // let facet_strings_for_search = vec!["/genre/7".to_string(), "/year/2020".to_string()];
    // let facet_strings = vec![
    //     "/genre".to_string(),
    //     "/year".to_string(),
    //     "/album".to_string(),
    //     "/artist".to_string(),
    // ];

    let faceted = Faceted {
        tags: vec![
            "/genre".to_string(),
            "/year".to_string(),
            "/album".to_string(),
            "/artist".to_string(),
        ],
    };
    // let year_start: Option<i32> = Some(2004);
    // let year_end: Option<i32> = Some(2006);

    // Order by
    let order_by_object: OrderBy = OrderBy {
        field: "created_date".to_string(),
        order_type: OrderType::Desc,
    };
    let order_by: Option<OrderBy> = Some(order_by_object);

    // let created_date_start: Option<i32> = Some(1665060000);
    // let created_date_end: Option<i32> = Some(1665066700);

    // let limit = 10;
    // let offset = 0;
    let faced_only_flag = true;

    let request = DocumentSearchRequest {
        text,
        fields: vec!["body".to_string()],
        filters: Filters {
            year_start: Some(1900),
            year_end: Some(2050),
            created_date_start: None,
            created_date_end: None,
        },
        faceted: Some(faceted.clone()), // Some(faceted.clone()),
        order: order_by,
        page_number: 0,
        result_per_page: 10,
        reload: false,
    };

    println!("request {:?} ", &request);
    let response: DocumentSearchResponse =
        do_search(index, reader, field_schema, &request, faced_only_flag);

    let response_json = serde_json::to_string(&response)?;
    println!("{}", response_json);

    let end = SystemTime::now();
    println!(
        "cost {}ms ({}secs) to index and run the search",
        end.duration_since(start).unwrap().as_millis(),
        end.duration_since(start).unwrap().as_secs(),
    );

    Ok(())
}

fn walk(path: &String) {
    let start = SystemTime::now();
    println!("start travel {}", path);
    let mut cnt = 0;

    let mut generic = WalkDir::new(&path);
    generic = generic.process_read_dir(move |_depth, _path, _read_dir_state, children| {
        children.iter_mut().for_each(|dir_entry_result| {
            if let Ok(dir_entry) = dir_entry_result {
                norm(dir_entry.path().to_str().unwrap_or(""));
            }
        });
    });

    let mut all_tracks: Vec<TrackJson> = Vec::new();
    let mut tracks_failed: Vec<String> = Vec::new();

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

        let allowed_types = ["mp3", "m4a", "mp4", "flac", "wav"];

        if !is_dir & allowed_types.contains(&ext) {
            if let Some(t) = get_track_from_path(&path_string) {
                all_tracks.push(t);
            } else {
                tracks_failed.push(path_string)
            }
        }
    }

    if !tracks_failed.is_empty() {
        println!("Failed to index {} file(s) ", &tracks_failed.len());
        if !tracks_failed.is_empty() {
            for track_failed in tracks_failed {
                println!("  track failed {} ", track_failed);
            }
        }
    }
    let end = SystemTime::now();

    // save all_tracks to json file
    let as_string = serde_json::to_string_pretty(&all_tracks).unwrap();
    fs::write("./data/audio.json", as_string).expect("Unable to write file");

    println!(
        "cost {}ms, total {} files",
        end.duration_since(start).unwrap().as_millis(),
        cnt
    );
}
