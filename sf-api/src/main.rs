use std::env;
use dotenv::dotenv;

pub mod client;
pub mod json_methods;
pub mod sobjects;
pub mod communities;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let access_token = env::var("SF_ACCESS_TOKEN").ok().unwrap();
  let sf_endpoint = env::var("SF_ENDPOINT").ok().unwrap();
  let sf_api_version = env::var("SF_API_VERSION").ok().unwrap();
  println!("Using access_token: {:?} ", access_token);

  let client = client::SalesforceClient{
    access_token: access_token,
    sf_endpoint: sf_endpoint,
    sf_api_version: sf_api_version
  };

  json_methods::sobject_to_json(&client).await?;
  json_methods::communities_to_json(&client).await?;

  Ok(())
}