use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct RepositoryInfo {
    pub name: String,
    pub languages: Vec<String>,
    pub html_url: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub commits: i32,
    pub updated_at: Option<String>,
    pub _type: Option<String>,
    // TODO: add framework/library property
}
