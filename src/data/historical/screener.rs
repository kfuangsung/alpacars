use crate::common::client::{base_url, RestClient};
use crate::data::enums::{MarketType, MostActivesBy};
use crate::data::models::{MostActives, Movers};
use crate::error::AlpacaError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct MostActivesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<MostActivesBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct MarketMoversRequest {
    /// Selects the endpoint (`/screener/{market_type}/movers`); defaults to `Stocks`. It is a
    /// path segment, not a query parameter.
    #[serde(skip)]
    pub market_type: Option<MarketType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<u32>,
}

/// Async client for the Alpaca Stock Screener API.
#[derive(Clone)]
pub struct ScreenerClient {
    client: RestClient,
}

impl ScreenerClient {
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

    /// Create a client pointed at a custom base URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(
        api_key: &str,
        secret_key: &str,
        base_url: &str,
    ) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                base_url.to_string(),
                "v1beta1".to_string(),
                false,
            )?,
        })
    }

    pub async fn get_most_actives(
        &self,
        req: &MostActivesRequest,
    ) -> Result<MostActives, AlpacaError> {
        self.client
            .get("/screener/stocks/most-actives", Some(req))
            .await
    }

    pub async fn get_market_movers(
        &self,
        req: &MarketMoversRequest,
    ) -> Result<Movers, AlpacaError> {
        let market = match req.market_type {
            Some(MarketType::Crypto) => "crypto",
            _ => "stocks",
        };
        self.client
            .get(&format!("/screener/{}/movers", market), Some(req))
            .await
    }
}
