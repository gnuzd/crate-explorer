use anyhow::{Context, Ok, Result};
use reqwest::Client;

use crate::models::{CrateInfo, SearchResponse};

const BASE: &str = "https://crates.io/api/v1";

#[derive(Debug)]
pub struct CratesClient {
    http: Client,
}

impl CratesClient {
    pub fn new() -> Result<Self> {
        let http = Client::builder()
            .user_agent("crate-explorer/0.1 (ban@example.com)")
            .build()
            .context("Can't build http client")?;
        Ok(Self { http })
    }

    pub async fn search(&self, query: &str) -> Result<Vec<CrateInfo>> {
        self.fetch("q", query).await
    }

    pub async fn by_category(&self, category: &str) -> Result<Vec<CrateInfo>> {
        self.fetch("category", category).await
    }

    pub async fn by_keyword(&self, keyword: &str) -> Result<Vec<CrateInfo>> {
        self.fetch("keyword", keyword).await
    }

    async fn fetch(&self, param: &str, value: &str) -> Result<Vec<CrateInfo>> {
        let url = format!("{}/crates", BASE);
        let resp = self
            .http
            .get(&url)
            .query(&[(param, value), ("per_page", "20")])
            .send()
            .await
            .context("Send request fail")?
            .error_for_status()
            .context("crates.io response error HTTP")?
            .json::<SearchResponse>()
            .await
            .context("Parse error")?;

        Ok(resp.crates)
    }
}
