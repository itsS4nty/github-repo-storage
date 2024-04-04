use reqwest::{header, Client};

pub fn create_client() -> Result<Client, reqwest::Error> {
    let client = Client::builder()
        .user_agent("Github Repo Storage")
        .build()?;
    Ok(client)
}

pub fn build_headers(token: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers
}
