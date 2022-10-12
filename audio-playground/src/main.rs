use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string, File};

use std::path::Path;
use std::time::SystemTime;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use audiotags::Tag;
use chrono::NaiveDateTime;
use tantivy::chrono::Utc;
use tantivy::collector::{Count, FacetCollector, FacetCounts, MultiCollector, TopDocs};
use tantivy::query::{AllQuery, BooleanQuery, Occur, Query, QueryParser};
use tantivy::{schema::*, DocAddress};
use tantivy::{DateTime, Index};

use jwalk::{DirEntry, WalkDir};

mod schema;
mod search_query;
mod utils;

use crate::schema::{FacetResult, FacetResults, FieldSchema, OrderBy, TrackJson};
use crate::utils::norm;

const JSON_DATA_FILE: &str = "./data/audio.json";

// "C:\\Users\\lukes\\Music"
// "E:\\Music";
const BASE_AUDIO_DIRECTORY: &str =
    "C:\\Users\\lukes\\Github\\rust-adventures\\audio-playground\\audio";

pub struct SearchResponse<'a, S> {
    pub query: &'a str,
    pub facets_count: FacetCounts,
    pub facets: Vec<String>,
    pub top_docs: Vec<(S, DocAddress)>,
    pub order_by: Option<OrderBy>,
    pub page_number: i32,
    pub results_per_page: i32,
}

fn facet_count(facet: &str, facets_count: &FacetCounts) -> Vec<FacetResult> {
    facets_count
        .top_k(facet, 50)
        .into_iter()
        .map(|(facet, count)| FacetResult {
            tag: facet.to_string(),
            total: count as i32,
        })
        .collect()
}

fn create_facets(facets: Vec<String>, facets_count: FacetCounts) -> HashMap<String, FacetResults> {
    facets
        .into_iter()
        .map(|facet| (&facets_count, facet))
        .map(|(facets_count, facet)| (facet_count(&facet, facets_count), facet))
        .filter(|(r, _)| !r.is_empty())
        .map(|(facet_results, facet)| (facet, FacetResults { facet_results }))
        .collect()
}

fn is_valid_facet(maybe_facet: &str) -> bool {
    Facet::from_text(maybe_facet)
        .map_err(|_| println!("Invalid facet: {maybe_facet}"))
        .is_ok()
}

fn main() -> tantivy::Result<()> {
    if true {
        // Fetch audio data and save to the local JSON file
        walk(&norm(BASE_AUDIO_DIRECTORY).to_string());
    }

    if false {
        search();
    }

    Ok(())
}

pub fn file_ext(file_name: &str) -> &str {
    if !file_name.contains(".") {
        return "";
    }
    file_name.split(".").last().unwrap_or("")
}

fn search() -> tantivy::Result<()> {
    // Read JSON from file
    let json_file_path = Path::new(JSON_DATA_FILE);
    let json_file_str = read_to_string(json_file_path).expect("file not found");
    let data: Vec<TrackJson> = serde_json::from_str(&json_file_str).unwrap();

    let field_schema: FieldSchema = FieldSchema::new();
    let index = Index::create_in_ram(field_schema.schema.clone());

    let mut index_writer = index.writer(30_000_000)?;

    println!("Total {} items", data.len());

    for item in data.iter() {
        let mut document = Document::default();
        document.add_text(field_schema.title, &item.name);
        document.add_text(field_schema.track, &item.track);
        document.add_text(field_schema.album, &item.album);
        document.add_text(field_schema.artist, &item.artist);
        document.add_u64(field_schema.year, item.year as u64);

        let date_time_value =
            DateTime::from_utc(NaiveDateTime::from_timestamp(item.created_date, 0), Utc);
        document.add_date(field_schema.created_date, date_time_value);

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

        index_writer.add_document(document)?;
    }
    index_writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let mut query_parser = QueryParser::for_index(
        &index,
        vec![field_schema.title, field_schema.artist, field_schema.album],
    );
    query_parser.set_field_boost(field_schema.title, 2.0);

    // Query Variables
    let text = "";
    let facet_strings_for_search =
        ["/genre/ambient".to_string(), "/year/2003".to_string()].to_vec();
    let facet_strings = [
        "/genre".to_string(),
        // "/genre/metalcore".to_string(),
        "/year".to_string(),
        "/album".to_string(),
        "/artist".to_string(),
    ]
    .to_vec();
    let year_start = 2004;
    let year_end = 2006;

    let created_date_start = 1665060000;
    let created_date_end = 1665066700;

    let limit = 10;
    let offset = 0;
    let order_field = field_schema.title;

    let mut queries: Vec<(Occur, Box<dyn Query>)> = vec![];
    let main_q = if text.is_empty() {
        Box::new(AllQuery)
    } else {
        query_parser.parse_query(text).unwrap()
    };

    queries.push((Occur::Must, main_q));

    // By Year
    // let year_range_query = Box::new(RangeQuery::new_u64(field_schema.year, year_start..year_end));
    // queries.push((Occur::Must, year_range_query));

    // By Created Date
    // let created_date_range_query = Box::new(RangeQuery::new_u64(
    //     field_schema.created_date,
    //     created_date_start..created_date_end,
    // ));
    // queries.push((Occur::Must, created_date_range_query));

    let facet_strings_for_search_valid: Vec<String> = facet_strings_for_search
        .iter()
        .filter(|s| is_valid_facet(*s))
        .cloned()
        .collect();
    // queries = build_facets(queries, &field_schema, facet_strings_for_search_valid);

    let query = BooleanQuery::new(queries);
    let mut multicollector = MultiCollector::new();

    let facets = facet_strings
        .iter()
        .filter(|s| is_valid_facet(*s))
        .cloned()
        .collect();

    println!("facets {:?} ", facets);

    let mut facet_collector = FacetCollector::for_field(field_schema.facets);
    for facet in &facets {
        match Facet::from_text(facet) {
            Ok(facet) => facet_collector.add_facet(facet),
            Err(_) => println!("Invalid facet: {}", facet),
        }
    }

    let facet_handler = multicollector.add_collector(facet_collector);

    let topdocs_collector = TopDocs::with_limit(limit).and_offset(offset);
    // .order_by_u64_field(order_field);
    let topdocs_handler = multicollector.add_collector(topdocs_collector);
    let count_handler = multicollector.add_collector(Count);

    let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();

    let facets_count = facet_handler.extract(&mut multi_fruit);
    let top_docs = topdocs_handler.extract(&mut multi_fruit);
    let count = count_handler.extract(&mut multi_fruit);
    println!("count {:?} \n", count);

    let facets_created = create_facets(facets, facets_count);
    for (facet_key, facet_top) in facets_created {
        println!("\n{}", facet_key);
        for facet_item in facet_top.facet_results {
            println!(
                "  {}: {:?} ",
                facet_item.tag.replace(&facet_key, ""),
                facet_item.total
            );
        }
    }

    let items_found = &top_docs.len();
    println!("\nfound {} items", &items_found);

    for (_score, doc_address) in top_docs {
        let the_doc = searcher.doc(doc_address).ok().unwrap();

        let response_track = the_doc.get_first(field_schema.track);
        println!("\ntrack {:?}", &response_track.unwrap());

        let response_album = the_doc.get_first(field_schema.album);
        println!("album {:?}", &response_album.unwrap());

        let response_artist = the_doc.get_first(field_schema.artist);
        println!("artist {:?}", &response_artist.unwrap());

        let response_facets = the_doc.get_all(field_schema.facets).collect::<Vec<_>>();
        for type_value in response_facets {
            let as_facet = type_value.as_facet().unwrap();
            println!("  {:?}", &as_facet.to_path_string());
        }
    }

    println!("found {} items", &items_found);

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
                let curr_path = norm(dir_entry.path().to_str().unwrap_or(""));
            }
        });
    });

    let mut all_tracks: Vec<TrackJson> = Vec::new();
    let mut tracks_failed: Vec<String> = Vec::new();

    // let (game_code, len) = loop {
    //     match loby_matcher(&mut stream) {
    //         Ok(game_code) => break game_code,
    //         // Err(e) if e.kind() == ErrorKind::Other && e.to_string() == "wrong game code" => {
    //         // or just (for ease of use)
    //         Err(e) if e.to_string() == "wrong game code" => {
    //             stream.write("Wrong code\n".as_bytes())?;
    //         }
    //         Err(e) => return Err(e),
    //     };
    // };

    for entry in generic {
        cnt += 1;
        if entry.is_err() {
            continue;
        }

        let en: DirEntry<((), ())> = entry.unwrap();
        let metadata = en.metadata().unwrap();
        let buf = en.path();
        let file_type = en.file_type();
        let is_dir = file_type.is_dir();

        let path = buf.to_str().unwrap().to_string();
        let name = en.file_name().to_str().unwrap();
        let ext = file_ext(name);

        let allowed_types = ["mp3", "m4a", "mp4", "flac"];

        if !is_dir & allowed_types.contains(&ext) {
            println!("path {}", &path);
            let tag = Tag::new().read_from_path(&path);

            if tag.is_err() {
                tracks_failed.push(path);
                continue;
            }
            all_tracks.push(TrackJson::new(norm(&path), metadata, tag.unwrap()));
        }
    }

    println!("Failed to index {} file(s) ", &tracks_failed.len());
    if !tracks_failed.is_empty() {
        for track_failed in tracks_failed {
            println!("  track failed {} ", track_failed);
        }
    }
    let end = SystemTime::now();

    // save all_tracks to json file
    let as_string = serde_json::to_string_pretty(&all_tracks).unwrap();
    fs::write("./data/audio.json", as_string).expect("Unable to write file");

    println!(
        "cost {} s, total {} files",
        end.duration_since(start).unwrap().as_secs(),
        cnt
    );
}
