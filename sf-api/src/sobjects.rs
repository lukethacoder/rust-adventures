use serde::{Deserialize, Serialize};
use colored::*;

use crate::client;

pub async fn get_sobject_data(
  client: &client::SalesforceClient,
) -> Result<ResponseObject, Box<dyn std::error::Error>> {
  println!("Using access_token: {:?} ", client.access_token);

  let endpoint_url = format!(
    "{endpoint}/services/data/v{version}/sobjects/",
    endpoint = client.sf_endpoint,
    version = client.sf_api_version
  );
  println!("via url {}", endpoint_url);

  let authorization_header = format!("Bearer {}", client.access_token);
  println!("with authorization_header {}", authorization_header);

  let client = reqwest::Client::new();

  let response = client
    .get(endpoint_url)
    .header("Authorization", authorization_header)
    .header("Content-Type", "application/json")
    .send()
    .await?;
  // eprintln!("Response: {:?} {}", response.version(), response.status());
  // eprintln!("Headers: {:#?}\n", response.headers());
  println!("{}{}", "Status ".blue(), response.status().as_u16());

  println!("{}", "success here ".green());
  let json = response.json::<ResponseObject>().await?;
  let json_as_string = serde_json::to_string(&json).unwrap();
  let response_object = serde_json::from_str(&json_as_string).unwrap();
  println!("Successful Response: \n {:#?}", response_object);
  
  Ok(response_object)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseObject {
  encoding: String,
  #[serde(rename = "maxBatchSize")]
  max_batch_size: i64,
  sobjects: Vec<SObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SObject {
  activateable: bool,
  #[serde(rename = "associateEntityType")]
  associate_entity_type: Option<AssociateEntityType>,
  #[serde(rename = "associateParentEntity")]
  associate_parent_entity: Option<String>,
  createable: bool,
  custom: bool,
  #[serde(rename = "customSetting")]
  custom_setting: bool,
  #[serde(rename = "deepCloneable")]
  deep_cloneable: bool,
  deletable: bool,
  #[serde(rename = "deprecatedAndHidden")]
  deprecated_and_hidden: bool,
  #[serde(rename = "feedEnabled")]
  feed_enabled: bool,
  #[serde(rename = "hasSubtypes")]
  has_subtypes: bool,
  #[serde(rename = "isInterface")]
  is_interface: bool,
  #[serde(rename = "isSubtype")]
  is_subtype: bool,
  #[serde(rename = "keyPrefix")]
  key_prefix: Option<String>,
  label: String,
  #[serde(rename = "labelPlural")]
  label_plural: String,
  layoutable: bool,
  mergeable: bool,
  #[serde(rename = "mruEnabled")]
  mru_enabled: bool,
  name: String,
  queryable: bool,
  replicateable: bool,
  retrieveable: bool,
  searchable: bool,
  triggerable: bool,
  undeletable: bool,
  updateable: bool,
  urls: Urls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Urls {
  #[serde(rename = "rowTemplate")]
  row_template: String,
  describe: String,
  sobject: String,
  #[serde(rename = "eventSchema")]
  event_schema: Option<String>,
  layouts: Option<String>,
  #[serde(rename = "compactLayouts")]
  compact_layouts: Option<String>,
  #[serde(rename = "approvalLayouts")]
  approval_layouts: Option<String>,
  listviews: Option<String>,
  #[serde(rename = "quickActions")]
  quick_actions: Option<String>,
  #[serde(rename = "caseArticleSuggestions")]
  case_article_suggestions: Option<String>,
  #[serde(rename = "caseRowArticleSuggestions")]
  case_row_article_suggestions: Option<String>,
  #[serde(rename = "eventSeriesUpdates")]
  event_series_updates: Option<String>,
  push: Option<String>,
  #[serde(rename = "namedLayouts")]
  named_layouts: Option<String>,
  #[serde(rename = "passwordUtilities")]
  password_utilities: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AssociateEntityType {
  ChangeEvent,
  Comment,
  DataCategorySelection,
  Feed,
  History,
  Share,
  TeamMember,
  TeamRole,
  TeamTemplate,
  TeamTemplateMember,
  TeamTemplateRecord,
  VersionHistory,
  ViewStat,
  VoteStat,
}
