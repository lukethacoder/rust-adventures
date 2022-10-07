use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::{AllQuery, BooleanQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::*;
use tantivy::{doc, DocId, Index, Score, SegmentReader};

use serde::{Deserialize, Serialize};

mod schema;
mod search_query;

use crate::schema::{DocumentSearchRequest, FieldSchema, Filter};
use crate::search_query::create_query;

const JSON_DATA_FILE: &str = "./data/audio.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackJson {
    pub abs_path: String,
    pub created_at: i64,
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
        document.add_text(field_schema.album, &item.album);
        document.add_text(field_schema.artist, &item.artist);

        for genre in &item.genres {
            let facet_string = format!("/{}", &genre);
            document.add_facet(field_schema.genres, Facet::from(&facet_string));
        }

        index_writer.add_document(document)?;
    }
    index_writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    // String Search

    // create_query()
    let query_parser = QueryParser::for_index(&index, vec![field_schema.title]);
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

    let text = "50 cent";

    let mut queries = vec![];
    let main_q = if text.is_empty() {
        Box::new(AllQuery)
    } else {
        query_parser.parse_query(text).unwrap()
    };

    queries.push((Occur::Must, main_q));
    // Fields
    // search.fields.iter().for_each(|value| {
    //     let facet_key: String = format!("/{}", value);
    //     let facet = Facet::from(facet_key.as_str());
    //     let facet_term = Term::from_facet(schema.field, &facet);
    //     let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
    //     queries.push((Occur::Must, Box::new(facet_term_query)));
    // });

    let tags = ["/jazz".to_string(), "/metalcore".to_string()];
    // let facet_string: Filter;
    // facet_string.tags = tags;

    // Add filter
    tags.iter()
        // .flat_map(|f| f.iter())
        .for_each(|value| {
            let facet = Facet::from(value.as_str());
            let facet_term = Term::from_facet(field_schema.genres, &facet);
            let facet_term_query = TermQuery::new(facet_term, IndexRecordOption::Basic);
            queries.push((Occur::Should, Box::new(facet_term_query)));
        });

    let query = BooleanQuery::new(queries);

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(10))
        .ok()
        .unwrap();

    println!("found {} items", &top_docs.len());
    for (_score, doc_address) in top_docs {
        let the_doc = searcher.doc(doc_address).ok().unwrap();

        let response_name = the_doc.get_first(field_schema.title);
        println!("name {:?}", &response_name);

        let response_album = the_doc.get_first(field_schema.album);
        println!("album {:?}", &response_album);

        let response_artist = the_doc.get_first(field_schema.artist);
        println!("artist {:?}", &response_artist);

        let response_types = the_doc.get_all(field_schema.genres).collect::<Vec<_>>();
        for type_value in response_types {
            let as_facet = type_value.as_facet().unwrap();
            println!("  {:?}", &as_facet.to_path_string().replace("/", ""));
        }
    }

    // let offset = 0;
    // let order_field = title_field;

    // let mut multicollector = MultiCollector::new();
    // let facet_handler = multicollector.add_collector(facet_collector);
    // let topdocs_collector = TopDocs::with_limit(extra_result)
    //     .and_offset(offset)
    //     .order_by_u64_field(order_field);
    // let topdocs_handler = multicollector.add_collector(topdocs_collector);
    // let mut multi_fruit = searcher.search(&query, &multicollector).unwrap();
    // let facets_count = facet_handler.extract(&mut multi_fruit);
    // let top_docs = topdocs_handler.extract(&mut multi_fruit);
    // self.convert_int_order(
    //     SearchResponse {
    //         facets_count,
    //         facets,
    //         top_docs,
    //         query: &text,
    //         order_by: request.order.clone(),
    //         page_number: request.page_number,
    //         results_per_page: results as i32,
    //     },
    //     &searcher,
    // )

    // let top_docs = searcher
    //     .search(&query, &top_docs_by_custom_score)
    //     .ok()
    //     .unwrap();
    // println!("found {} items", &top_docs.len());
    // for (_score, doc_address) in top_docs {
    //     let the_doc = searcher.doc(doc_address).ok().unwrap();
    //     let response_name = the_doc
    //         .get_first(field_schema.title)
    //         .unwrap()
    //         .as_text()
    //         .unwrap()
    //         .to_owned();
    //     println!("\nFound pokemon {:?} with types ", &response_name);
    //     let response_types = the_doc.get_all(types_field).collect::<Vec<_>>();
    //     for type_value in response_types {
    //         let as_facet = type_value.as_facet().unwrap();
    //         println!("  {:?}", &as_facet.to_path_string().replace("/", ""));
    //     }
    // }

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
