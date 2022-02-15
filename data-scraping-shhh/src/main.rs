use colored::*;
use std::fs;
use std::time::{Instant};

use serde_json::json;

pub mod types;

const MAX_API_HITS: i32 = 150;
const OFFSET_VALUE: i32 = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let now = Instant::now();
  let mut total_api_hits: i32 = 1;
  let mut results: Vec<types::Result> = vec![];

  let data = get_data(1).await?;

  println!("Total count of {}", &data.count.to_string().red());
  println!("should hit the API {} times.\n", &data.count / 20);

  results.extend_from_slice(&data.results);

  for num in 0..MAX_API_HITS {
    // change it to get range
    println!("Fetching with offset {}", num + OFFSET_VALUE);
    // hit the API here
    let response_data = get_data(num + OFFSET_VALUE).await?;
    results.extend_from_slice(&response_data.results);
    total_api_hits = total_api_hits + 1;

    if json!(response_data.links.next).is_null() {
      break;
    }
  }
  println!(
    "Saving a total of {} items to the json file.",
    results.len().to_string().green()
  );

  let as_string = serde_json::to_string_pretty(&results).unwrap();
  fs::write("zeengo.json", as_string).expect("Unable to write file");

  println!(
    "Hit the API {} times, and took {}ms",
    &total_api_hits.to_string().green(),
    now.elapsed().as_millis().to_string().red()
  );
  Ok(())
}

pub async fn get_data(
  page_number: i32,
) -> Result<types::ResponseObject, Box<dyn std::error::Error>> {
  println!("{}", "Hitting endpoint to fetch some data".magenta());

  let endpoint_url = format!(
    "https://zango.com.au/api/properties/?page={page_number}",
    page_number = page_number,
  );

  let client = reqwest::Client::new();
  let response = client
    .get(endpoint_url)
    .header("Content-Type", "application/json")
    .send()
    .await?;
  // eprintln!("Response: {:?} {}", response.version(), response.status());
  // eprintln!("Headers: {:#?}\n", response.headers());

  println!(
    "?page={} - {}{} \n",
    &page_number,
    "Status ".blue(),
    response.status().as_u16()
  );
  let json = response.json::<types::ResponseObject>().await?;
  let json_as_string = serde_json::to_string_pretty(&json).unwrap();
  let response_object = serde_json::from_str(&json_as_string).unwrap();

  Ok(response_object)
}
