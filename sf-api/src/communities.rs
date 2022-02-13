use serde::{Deserialize, Serialize};
use colored::*;

use crate::client;

pub async fn get_communities_data(
  client: &client::SalesforceClient,
) -> Result<ResponseObject, Box<dyn std::error::Error>> {
  println!("Using access_token: {:?} ", client.access_token);

  let endpoint_url = format!(
    "{endpoint}/services/data/v{version}/connect/communities",
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
    communities: Vec<Community>,
    total: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Community {
    #[serde(rename = "allowChatterAccessWithoutLogin")]
    allow_chatter_access_without_login: bool,
    #[serde(rename = "allowMembersToFlag")]
    allow_members_to_flag: bool,
    #[serde(rename = "builderBasedSnaEnabled")]
    builder_based_sna_enabled: bool,
    description: Option<serde_json::Value>,
    #[serde(rename = "guestMemberVisibilityEnabled")]
    guest_member_visibility_enabled: bool,
    id: String,
    #[serde(rename = "invitationsEnabled")]
    invitations_enabled: bool,
    #[serde(rename = "knowledgeableEnabled")]
    knowledgeable_enabled: bool,
    #[serde(rename = "loginUrl")]
    login_url: String,
    #[serde(rename = "memberVisibilityEnabled")]
    member_visibility_enabled: bool,
    name: String,
    #[serde(rename = "nicknameDisplayEnabled")]
    nickname_display_enabled: bool,
    #[serde(rename = "privateMessagesEnabled")]
    private_messages_enabled: bool,
    #[serde(rename = "reputationEnabled")]
    reputation_enabled: bool,
    #[serde(rename = "sendWelcomeEmail")]
    send_welcome_email: bool,
    #[serde(rename = "siteAsContainerEnabled")]
    site_as_container_enabled: bool,
    #[serde(rename = "siteUrl")]
    site_url: String,
    status: String,
    #[serde(rename = "templateName")]
    template_name: String,
    url: String,
    #[serde(rename = "urlPathPrefix")]
    url_path_prefix: Option<String>,
}
