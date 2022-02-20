
use dotenv::dotenv;


#[derive(Serialize, Deserialize, SObjectRepresentation)]
#[serde(rename_all = 'PascalCase')]
struct Account {
    id: Option<SalesforceId>,
    name: String
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
        
    let access_token = env::var("SF_ACCESS_TOKEN").ok().unwrap();
    let sf_instance_url = env::var("SF_INSTANCE_URL").ok().unwrap();
    let sf_api_version = env::var("SF_API_VERSION").ok().unwrap();
    
    let args = Args::parse();
    let conn = Connection::new(
        Box::new(AccessTokenAuth::new(
            access_token,
            Url::parse(&sf_instance_url)?,
        )),
        format!("v{}", sf_api_version)
    )?;

    Account::bulk_query_t(&conn, &args.query, false)
        .await?
        .map(move |r| {
            let mut r = r.unwrap();
            r.name = format!("{}.", r.name);
            r
        })
        .bulk_query_t(&conn)
        .await?;
    

    Ok(())
}
