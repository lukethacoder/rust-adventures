use rustforce::{Client, Error};
use rustforce::response::{QueryResponse, ErrorResponse};
use serde::Deserialize;
use std::env;
use colored::*;
use dotenv::dotenv;

mod schema;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    #[serde(rename = "attributes")]
    attributes: Attribute,
    id: String,
    name: String,
}
#[derive(Deserialize, Debug)]
struct Attribute {
    url: String,
    #[serde(rename = "type")]
    sobject_type: String,
}

#[async_std::main]
async fn main() {
  dotenv().ok();
  println!("Running main...\n");
  let _response = main_async().await;
  print!("Success??");
}

// fn get_username_from_alias(alias: &str) {
  
// }
// fn open_sf_org(username: &str) {

// }

async fn main_async() -> Result<(), Error> {
  println!("main_async");
    
  let client_id = env::var("SFDC_CLIENT_ID").unwrap();
  println!("client_id: {} ", client_id);
  let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
  println!("client_secret: {} ", client_secret);
  let refresh_token = env::var("SFDC_REFESH_TOKEN").unwrap();
  println!("refresh_token: {} ", refresh_token);
  let access_token = env::var("SFDC_ACCESS_TOKEN").unwrap();
  println!("access_token: {} ", access_token);
  // let username = env::var("SFDC_USERNAME").unwrap();
  // let password = env::var("SFDC_PASSWORD").unwrap();

  let mut client = sf_client::Client::new(client_id, client_secret);
  println!("{}", "created client token here".red());
  client.set_access_token(&access_token);
  println!("{}", "set access token".red());
  println!("{:?}", client.base_path());
  // client.refresh(&refresh_token).await?;
  // println!("{}", "refreshed token here".green());
  // client.login_with_credential(username, password).await?;
  let sosl_search = client.search("FIND {luke}").await?;
  println!("{:?}", sosl_search);


  // let res: QueryResponse<Account> = client.query(&"SELECT Id, Name FROM Account WHERE Name != null LIMIT 20".to_string()).await?;
  // println!("{:?}", res);

  Ok(())
}