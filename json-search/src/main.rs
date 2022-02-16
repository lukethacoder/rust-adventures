

use std::fs::{read_to_string};
use std::path::Path;
use tantivy::{doc,Index,ReloadPolicy};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tempfile::TempDir;

pub mod types;

const JSON_DATA_FILE: &str = "./data/zeengo.json";

fn main() -> tantivy::Result<()> {
  let index_path = TempDir::new()?;
  let mut schema_builder = Schema::builder();

  add_fields_to_schema(&mut schema_builder);
  
  // Read JSON from file
  let json_file_path = Path::new(JSON_DATA_FILE);
  let json_file_str = read_to_string(json_file_path).expect("file not found");
  let data:  Vec<types::Result> = serde_json::from_str(&json_file_str).unwrap();
  
  let schema = schema_builder.build();
  let index = Index::create_in_dir(&index_path, schema.clone())?;
  let mut index_writer = index.writer(50_000_000)?;

  let id = schema.get_field("id").unwrap();
  let price = schema.get_field("price").unwrap();
  let headline = schema.get_field("headline").unwrap();
  
  println!("Total {} items", data.len());
  for item in data.iter() {
    println!("Test item {} ", item.id);

    // let _price = Some(item.price).unwrap();
    index_writer.add_document(doc!(
      id => item.id,
      // price => &_price,
      headline => item.headline.to_string()
    ));
  }

  // let mut old_man_doc = Document::default();
  // old_man_doc.add_text(title, "The Old Man and the Sea");
  // old_man_doc.add_text(
  //   body,
  //   "He was an old man who fished alone in a skiff in the Gulf Stream and \
  //        he had gone eighty-four days now without taking a fish.",
  // );
  // index_writer.add_document(old_man_doc);

  // index_writer.add_document(doc!(
  // title => "Of Mice and Men",
  // body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
  //         bank and runs deep and green. The water is warm too, for it has slipped twinkling \
  //         over the yellow sands in the sunlight before reaching the narrow pool. On one \
  //         side of the river the golden foothill slopes curve up to the strong and rocky \
  //         Gabilan Mountains, but on the valley side the water is lined with trees—willows \
  //         fresh and green with every spring, carrying in their lower leaf junctures the \
  //         debris of the winter's flooding; and sycamores with mottled, white, recumbent \
  //         limbs and branches that arch over the pool"
  // ));

  // index_writer.add_document(doc!(
  // title => "Of Mice and Men",
  // body => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
  //         bank and runs deep and green. The water is warm too, for it has slipped twinkling \
  //         over the yellow sands in the sunlight before reaching the narrow pool. On one \
  //         side of the river the golden foothill slopes curve up to the strong and rocky \
  //         Gabilan Mountains, but on the valley side the water is lined with trees—willows \
  //         fresh and green with every spring, carrying in their lower leaf junctures the \
  //         debris of the winter's flooding; and sycamores with mottled, white, recumbent \
  //         limbs and branches that arch over the pool"
  // ));
  // index_writer.add_document(doc!(
  // title => "Frankenstein",
  // title => "The Modern Prometheus",
  // body => "You will rejoice to hear that no disaster has accompanied the commencement of an \
  //          enterprise which you have regarded with such evil forebodings.  I arrived here \
  //          yesterday, and my first task is to assure my dear sister of my welfare and \
  //          increasing confidence in the success of my undertaking."
  // ));

  index_writer.commit()?;

  let reader = index
    .reader_builder()
    .reload_policy(ReloadPolicy::OnCommit)
    .try_into()?;

  let searcher = reader.searcher();
  let query_parser = QueryParser::for_index(&index, vec![id, price, headline]);
  
  let query = query_parser.parse_query("outstanding views")?;
  let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

  for (_score, doc_address) in top_docs {
    let retrieved_doc = searcher.doc(doc_address)?;
    println!("{}", schema.to_json(&retrieved_doc));
  }

  Ok(())
}


pub fn add_fields_to_schema(schema_builder: &mut SchemaBuilder) {
  schema_builder.add_text_field("id", TEXT);
  schema_builder.add_i64_field("price", {});
  schema_builder.add_text_field("headline", TEXT);
}