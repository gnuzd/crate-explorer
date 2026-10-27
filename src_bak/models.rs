use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub crates: Vec<CrateInfo>,
    pub meta: Meta,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct CrateInfo {
    pub name: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub max_version: String,
    pub repository: Option<String>,
}
