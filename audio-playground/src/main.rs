use std::collections::HashSet;
use std::fs::{read_to_string};
use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery};
use tantivy::schema::*;
use tantivy::{doc, DocId, Index, Score, SegmentReader};

use serde::{Deserialize, Serialize};

const JSON_DATA_FILE: &str = "../_data/pokemon-small.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct PokemonImage {
  pub back_default: String,
  pub front_default: String,
  pub official: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pokemon {
  pub id: i64,
  pub name: String,
  pub height: i64,
  pub weight: i64,
  pub types: Vec<String>,
  pub images: PokemonImage,
}

pub fn add_fields_to_schema(schema_builder: &mut SchemaBuilder) {
  schema_builder.add_text_field("title", TEXT | STORED);
  schema_builder.add_facet_field("types", STORED); // FacetOptions::default()
}
  
fn main() -> tantivy::Result<()> {
  // Read JSON from file
  let json_file_path = Path::new(JSON_DATA_FILE);
  let json_file_str = read_to_string(json_file_path).expect("file not found");
  let data:  Vec<Pokemon> = serde_json::from_str(&json_file_str).unwrap();
  
  let mut schema_builder = Schema::builder();
  add_fields_to_schema(&mut schema_builder);
  let schema = schema_builder.build();

  let title_field = schema.get_field("title").unwrap();
  let types_field = schema.get_field("types").unwrap();

  let index = Index::create_in_ram(schema);

  let mut index_writer = index.writer(30_000_000)?;
  
  println!("Total {} items", data.len());

  for item in data.iter() {
    let mut document = Document::default();
    document.add_text(title_field, &item.name);

    for pokemon_type in &item.types {
      let facet_string = format!("/{}", &pokemon_type);
      document.add_facet(types_field, Facet::from(&facet_string));
    }

    index_writer.add_document(document)?;
  }
  index_writer.commit()?;

  let reader = index.reader()?;
  let searcher = reader.searcher();


  // String Search 
  // let query_parser = QueryParser::for_index(&index, vec![title]);
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

  // Facet Search
  {
    // Types to search by 
    let facets = vec![
      Facet::from("/flying"),
      Facet::from("/fire")
    ];


    let query = BooleanQuery::new_multiterms_query(
      facets
        .iter()
        .map(|key| {
          Term::from_facet(types_field, key)
        })
        .collect()
    );

    let top_docs_by_custom_score = TopDocs::with_limit(4).tweak_score(
      move |segment_reader: &SegmentReader| {
        let types_reader = segment_reader.facet_reader(types_field).unwrap();
        let facet_dict = types_reader.facet_dict();

        let query_ords: HashSet<u64> = facets
          .iter()
          .filter_map(|key| facet_dict.term_ord(key.encoded_str()).unwrap())
          .collect();

        let mut facet_ords_buffer: Vec<u64> = Vec::with_capacity(20);

        move |doc: DocId, original_score: Score| {
          types_reader.facet_ords(doc, &mut facet_ords_buffer);
          let missing_types = facet_ords_buffer
            .iter()
            .filter(|ord| !query_ords.contains(ord))
            .count();
          let tweak = 1.0 / (4_f32).powi(missing_types as i32);

          original_score * tweak
        }
      }
    );
    
    let top_docs = searcher.search(&query, &top_docs_by_custom_score).ok().unwrap();
    println!("found {} items", &top_docs.len());

    for (_score, doc_address) in top_docs {
      let the_doc = searcher.doc(doc_address).ok().unwrap();
      let response_name = the_doc.get_first(title_field).unwrap().as_text().unwrap().to_owned();
      println!("\nFound pokemon {:?} with types ", &response_name);

      let response_types = the_doc.get_all(types_field).collect::<Vec<_>>();
      for type_value in response_types {
        let as_facet = type_value.as_facet().unwrap();
        println!("  {:?}", &as_facet.to_path_string().replace("/", ""));
      }
    }
  }

  Ok(())
}