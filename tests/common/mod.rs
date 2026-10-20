#![allow(dead_code)]

use alpacars::broker::client::BrokerClient;
use alpacars::data::historical::corporate_actions::CorporateActionsClient;
use alpacars::data::historical::crypto::CryptoHistoricalDataClient;
use alpacars::data::historical::news::NewsClient;
use alpacars::data::historical::option::OptionHistoricalDataClient;
use alpacars::data::historical::screener::ScreenerClient;
use alpacars::data::historical::stock::StockHistoricalDataClient;
use alpacars::trading::client::TradingClient;

pub fn trading_client(base_url: &str) -> TradingClient {
    TradingClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn broker_client(base_url: &str) -> BrokerClient {
    BrokerClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn stock_client(base_url: &str) -> StockHistoricalDataClient {
    StockHistoricalDataClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn corporate_actions_client(base_url: &str) -> CorporateActionsClient {
    CorporateActionsClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn crypto_client(base_url: &str) -> CryptoHistoricalDataClient {
    CryptoHistoricalDataClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn news_client(base_url: &str) -> NewsClient {
    NewsClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn screener_client(base_url: &str) -> ScreenerClient {
    ScreenerClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn option_client(base_url: &str) -> OptionHistoricalDataClient {
    OptionHistoricalDataClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}
