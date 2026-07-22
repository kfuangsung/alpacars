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
    #[serde(skip_serializing_if = "Option::is_none")]
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
        self.client.get("/screener/stocks/movers", Some(req)).await
    }
}
