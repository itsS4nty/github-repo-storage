use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
    Collection 
};
use std::error::Error;
use crate::models::CombinedRepositoryInfo;
use std::env;

async fn get_client() -> Result<Client, Box<dyn Error>> {
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let options =
        ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare()).await?;
    let client = Client::with_options(options)?;

    Ok(client)
}

async fn delete_repos(repositories_coll: Collection<CombinedRepositoryInfo>) -> Result<(), Box<dyn Error>> {
    repositories_coll.drop(None).await?;

    Ok(())
}

pub async fn insert_repos(repos: Vec<CombinedRepositoryInfo>) -> Result<(), Box<dyn Error>> {
    let client = get_client().await?;
    let repositories_coll = client.database("s4nty-web").collection("repositories");

    delete_repos(repositories_coll.clone()).await?;
    
    repositories_coll.insert_many(repos, None).await?;

    Ok(())
}