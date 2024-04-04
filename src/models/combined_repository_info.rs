use std::collections::HashMap;
use super::RepositoryInfo;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CombinedRepositoryInfo {
    pub name: String,
    pub components: HashMap<String, RepositoryInfo>,
}
