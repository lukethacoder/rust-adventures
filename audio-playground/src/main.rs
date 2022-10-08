use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::fs::read_to_string;
use std::path::Path;

use chrono::NaiveDateTime;
use tantivy::chrono::Utc;
use tantivy::collector::{Count, FacetCollector, FacetCounts, MultiCollector, TopDocs};
use tantivy::query::{AllQuery, BooleanQuery, Occur, Query, QueryParser, RangeQuery, TermQuery};
use tantivy::{doc, DateTime, DocId, Index, IndexReader, Score, SegmentReader};
use tantivy::{schema::*, DocAddress};

use serde::{Deserialize, Serialize};

mod schema;
mod search_query;

use crate::schema::{
    DocumentSearchRequest, FacetResult, FacetResults, FieldSchema, Filter, OrderBy,
};
use crate::search_query::create_query;

const JSON_DATA_FILE: &str = "./data/audio.json";

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
        document.add_u64(field_schema.year, item.year.parse::<u64>().unwrap_or(1900));

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

    // String Search

    // create_query()
    let mut query_parser = QueryParser::for_index(
        &index,
        vec![field_schema.title, field_schema.artist, field_schema.album],
    );
    query_parser.set_field_boost(field_schema.title, 2.0);
    // // let ext_query_parser = QueryParser::for_index(&index, vec![year]);
    // let query = query_parser.parse_query("geodude")?;
    // let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).ok().unwrap();
    // println!("found {} items", &top_docs.len());

    // for (_score, doc_address) in top_docs {
    //   println!("doc_address {:?}", &doc_address);
    //   let retrieved_doc = searcher.doc(doc_address)?;
    //   println!("response {:?}", &retrieved_doc);
    // }

    // let string_to_search = "charizard";
    // let query_parser = QueryParser::for_index(&index, vec![title]);
    // // let ext_query_parser = QueryParser::for_index(&index, vec![year]);
    // let query = query_parser.parse_query(string_to_search)?;

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

    // Facet Search
    // {
    //   // Types to search by
    //   let facets = vec![
    //     Facet::from("/flying"),
    //     Facet::from("/fire")
    //   ];

    //   let query = BooleanQuery::new_multiterms_query(
    //     facets
    //       .iter()
    //       .map(|key| {
    //         Term::from_facet(types_field, key)
    //       })
    //       .collect()
    //   );

    //   let top_docs_by_custom_score = TopDocs::with_limit(4).tweak_score(
    //     move |segment_reader: &SegmentReader| {
    //       let types_reader = segment_reader.facet_reader(types_field).unwrap();
    //       let facet_dict = types_reader.facet_dict();

    //       let query_ords: HashSet<u64> = facets
    //         .iter()
    //         .filter_map(|key| facet_dict.term_ord(key.encoded_str()).unwrap())
    //         .collect();

    //       let mut facet_ords_buffer: Vec<u64> = Vec::with_capacity(20);

    //       move |doc: DocId, original_score: Score| {
    //         types_reader.facet_ords(doc, &mut facet_ords_buffer);
    //         let missing_types = facet_ords_buffer
    //           .iter()
    //           .filter(|ord| !query_ords.contains(ord))
    //           .count();
    //         let tweak = 1.0 / (4_f32).powi(missing_types as i32);

    //         original_score * tweak
    //       }
    //     }
    //   );

    //   let top_docs = searcher.search(&query, &top_docs_by_custom_score).ok().unwrap();
    //   println!("found {} items", &top_docs.len());

    //   for (_score, doc_address) in top_docs {
    //     let the_doc = searcher.doc(doc_address).ok().unwrap();
    //     let response_name = the_doc.get_first(title_field).unwrap().as_text().unwrap().to_owned();
    //     println!("\nFound pokemon {:?} with types ", &response_name);

    //     let response_types = the_doc.get_all(types_field).collect::<Vec<_>>();
    //     for type_value in response_types {
    //       let as_facet = type_value.as_facet().unwrap();
    //       println!("  {:?}", &as_facet.to_path_string().replace("/", ""));
    //     }
    //   }
    // }

    Ok(())
}

// fn search_text_in_track() {}
// fn search_text_in_genre() {}
// fn search_text_in_artist() {}
// fn search_text_in_album() {}

// fn search_range_year(year_from: Option<String>, year_to: Option<String>) {
//     println!("Searching from {:?} to {:?}", year_from, year_to);
// }
