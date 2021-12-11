pub fn authenticate(
  client_id: &str,
  client_secret: &str,
  instance_url: &str,
) {

  

  // let auth_url_string = format!("{}/services/oauth2/authorize", instance_url)
  // let auth_url = AuthUrl::new(auth_url_string.to_string())
  //     .expect("Invalid authorization endpoint URL");

  // let token_url_string = format!("{}/services/oauth2/token", instance_url)
  // let token_url = TokenUrl::new(token_url_string.to_string())
  //     .expect("Invalid token endpoint URL");

  // // Set up the config for the Google OAuth2 process.
  // let client = BasicClient::new(
  //     client_id,
  //     client_secret,
  //     auth_url,
  //     Some(token_url),
  // )
  // This example will be running its own server at localhost:8080.
  // See below for the server implementation.
  // .set_redirect_uri(
  //     RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
  // )
  // // Google supports OAuth 2.0 Token Revocation (RFC-7009)
  // .set_revocation_uri(
  //     RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
  //         .expect("Invalid revocation endpoint URL"),
  // );
  // let client =
  // BasicClient::new(
  //     ClientId::new(client_id.to_string()),
  //     Some(ClientSecret::new(client_secret.to_string())),
  //     AuthUrl::new("http://authorize".to_string())?,
  //     Some(TokenUrl::new("http://token".to_string())?),
  // );

  // let token_result = client
  //   .exchange_client_credentials()
  //   .add_scope(Scope::new("read".to_string()))
  //   .request(http_client)?;
}