use std::fs;

use crate::client;
use crate::sobjects;
use crate::communities;

pub async fn sobject_to_json(client: &client::SalesforceClient) -> Result<(), Box<dyn std::error::Error>> {
  let sobject_data = sobjects::get_sobject_data(&client)
    .await?;
  let as_string = serde_json::to_string_pretty(&sobject_data).unwrap();
  fs::write("sobject.json", as_string).expect("Unable to write file");

  Ok(())
}

pub async fn communities_to_json(client: &client::SalesforceClient) -> Result<(), Box<dyn std::error::Error>> {
  let sobject_data = communities::get_communities_data(&client)
    .await?;
  let as_string = serde_json::to_string_pretty(&sobject_data).unwrap();
  fs::write("communities.json", as_string).expect("Unable to write file");

  Ok(())
}