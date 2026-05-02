use crate::common::client::{base_url, RestClient};
use crate::data::models::{News, NewsSet};
use crate::error::AlpacaError;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct NewsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_contentless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct NewsResponse {
    news: Option<Vec<News>>,
    next_page_token: Option<String>,
}

/// Async client for the Alpaca News API.
#[derive(Clone)]
pub struct NewsClient {
    client: RestClient,
}

impl NewsClient {
    pub fn new(api_key: Option<&str>, secret_key: Option<&str>) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                api_key.map(str::to_string),
                secret_key.map(str::to_string),
                None,
                base_url::DATA.to_string(),
                "v1beta1".to_string(),
                false,
            )?,
        })
    }

    /// Fetch news articles (auto-paginated).
    pub async fn get_news(&self, req: &NewsRequest) -> Result<NewsSet, AlpacaError> {
        let mut params = req.clone();
        let mut all_news: NewsSet = Vec::new();

        loop {
            let resp: NewsResponse = self.client.get("/news", Some(&params)).await?;
            if let Some(articles) = resp.news {
                all_news.extend(articles);
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(all_news)
    }
}
