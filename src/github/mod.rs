use crate::client::build_headers;
use reqwest::Client;
use serde_json::{Value, Map};
use std::error::Error;

pub async fn get_user_repos(client: &Client, token: &str, user_login: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let url = format!("https://api.github.com/user/repos?sort=updated&per_page=100");
    let headers = build_headers(token);

    let res = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let repos: Vec<Value> = serde_json::from_str(&res)?;
    Ok(repos.into_iter().filter(|repo| repo["owner"]["login"] == user_login).collect())
}

pub async fn get_repo_branch(client: &Client, token: &str, owner: &str, repo: &str, branch: &str) -> Result<bool, Box<dyn Error>> {
    let branch_url = format!("https://api.github.com/repos/{}/{}/branches/{}", owner, repo, branch);
    let headers = build_headers(token);

    let res = client
        .get(&branch_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let data: Value = serde_json::from_str(&res)?;

    Ok(!data["name"].is_null())
}

pub async fn get_commits(client: &Client, token: &str, owner: &str, repo: &str) -> Result<i32, Box<dyn Error>> {
    let commits_url = format!("https://api.github.com/repos/{}/{}/commits", owner, repo);
    let headers = build_headers(token);
    
    let res = client
        .get(&commits_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let json_res: Value = serde_json::from_str(&res)?;
    
    let commits_count = json_res.as_array().ok_or("Failed to parse as array")?.len() as i32;
    
    Ok(commits_count)
}

pub async fn get_languages(client: &Client, token: &str, owner: &str, repo: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let languages_url = format!("https://api.github.com/repos/{}/{}/languages", owner, repo);
    let headers = build_headers(token);

    let res = client
        .get(&languages_url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let languages: Map<String, Value> = serde_json::from_str(&res)?;
    
    let language_names = languages.keys()
        .map(|k| k.to_lowercase())
        .collect::<Vec<String>>();
    
    Ok(language_names)
}