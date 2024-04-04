mod client;
mod models;
mod utils;
mod database;

use client::create_client;
use models::RepositoryInfo;
use std::env;
use dotenv::dotenv;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let github_user = env::var("GITHUB_USER").expect("GITHUB_USER not set");
    let repo_branch = env::var("REPO_BRANCH").expect("REPO_BRANCH not set");
    
    let client = create_client()?;

    let repos = utils::get_user_repos(&client, &token, &github_user).await?;
    let mut repos_to_add = Vec::new();
    
    for repo in repos {
        if let Some(name) = repo["name"].as_str() {
            if utils::get_repo_branch(&client, &token, &github_user, name, &repo_branch).await? {
                let repo_info = RepositoryInfo {
                    name: name.to_string(),
                    languages: utils::get_languages(&client, &token, &github_user, name).await?,
                    html_url: repo["html_url"].as_str().unwrap_or_default().to_string(),
                    description: repo["description"].as_str().map(|s| s.to_string()),
                    homepage: repo["homepage"].as_str().map(|s| s.to_string()),
                    commits: utils::get_commits(&client, &token, &github_user, name).await?,
                    updated_at: repo["updated_at"].as_str().map(|s| s.to_string()),
                    _type: Some("Test".to_string()),
                };
                repos_to_add.push(repo_info);
            }
        }
    }

    let combined_repos = utils::combine_repos(repos_to_add);
    database::insert_repos(combined_repos).await?;

    println!("Repositories inserted.");

    Ok(())
}
