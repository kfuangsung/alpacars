#![allow(dead_code)]

use alpacars::trading::client::TradingClient;
use alpacars::data::historical::stock::StockHistoricalDataClient;

pub fn trading_client(base_url: &str) -> TradingClient {
    TradingClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}

pub fn stock_client(base_url: &str) -> StockHistoricalDataClient {
    StockHistoricalDataClient::new_with_url("test-key", "test-secret", base_url).unwrap()
}
