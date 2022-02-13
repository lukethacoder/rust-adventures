use std::env;
use std::collections::HashMap;

use colored::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use http::{HeaderMap, HeaderValue, header::{}};

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorObject {
    message: String,
    #[serde(rename = "errorCode")]
    error_code: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let access_token = env::var("SF_ACCESS_TOKEN").ok().unwrap();
  let sf_endpoint = env::var("SF_ENDPOINT").ok().unwrap();
  let sf_api_version = env::var("SF_API_VERSION").ok().unwrap();
  println!("Using access_token: {:?} ", access_token);

  let endpoint_url = format!(
    "{endpoint}/services/data/v{version}/sobjects/",
    endpoint = sf_endpoint,
    version = sf_api_version
  );
  println!("via url {}", endpoint_url);

  let authorization_header = format!("Bearer {}", access_token);
  println!("with authorization_header {}", authorization_header);

  let client = reqwest::Client::new();

  let response = client.get(endpoint_url)
    .header("Authorization", authorization_header)
    .header("Content-Type", "application/json")
    .send()
    .await?;
  // eprintln!("Response: {:?} {}", response.version(), response.status());
  // eprintln!("Headers: {:#?}\n", response.headers());
  println!("{}{}", "Status ".blue(), response.status().as_u16());

  if response.status().as_u16() == 401 {
    println!("{}", "http error here ".red());
    let error_response = response.json::<Vec<ErrorObject>>().await?;
    println!("error_response {:?}", error_response[0]);

  } else {
    println!("{}", "success here ".green());
    let json = response.json::<HashMap<String, String>>().await?;
    println!("Successful Response: \n {:#?}", json);
  }
  Ok(())
}