//! Screener (`/v1beta1/screener/...`), checked against the mostactives-1 and movers-1 references.

mod common;

use alpacars::data::enums::{MarketType, MostActivesBy};
use alpacars::data::historical::screener::{MarketMoversRequest, MostActivesRequest};
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

const MOVERS_JSON: &str = r#"{
  "gainers": [
    {
      "change": 2.46,
      "percent_change": 145.56,
      "price": 4.15,
      "symbol": "AGRI"
    },
    {
      "change": 0.03,
      "percent_change": 85.63,
      "price": 0.0594,
      "symbol": "GRCYW"
    }
  ],
  "last_updated": "2022-03-10T17:53:30.088309839Z",
  "losers": [
    {
      "change": -0.26,
      "percent_change": -63.07,
      "price": 0.1502,
      "symbol": "MTACW"
    },
    {
      "change": -3.61,
      "percent_change": -51.21,
      "price": 3.435,
      "symbol": "TIG"
    }
  ],
  "market_type": "stocks"
}"#;

#[tokio::test]
async fn test_get_most_actives() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1beta1/screener/stocks/most-actives"))
        .and(query_param("by", "volume"))
        .and(query_param("top", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"most_actives": [{"symbol": "AAPL", "volume": 1000000, "trade_count": 5000}],
                "last_updated": "2026-10-20T15:00:00Z"}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::screener_client(&server.uri());
    let req = MostActivesRequest {
        by: Some(MostActivesBy::Volume),
        top: Some(5),
    };
    client.get_most_actives(&req).await.unwrap();
}

/// `market_type` is a path segment (`/screener/{market_type}/movers`). The crate always called
/// `/screener/stocks/movers` and sent `market_type` as an ignored query parameter, so crypto
/// movers returned stock movers.
#[tokio::test]
async fn test_get_market_movers_crypto_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1beta1/screener/crypto/movers"))
        .and(query_param_is_missing("market_type"))
        .and(query_param("top", "3"))
        .respond_with(ResponseTemplate::new(200).set_body_string(MOVERS_JSON))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::screener_client(&server.uri());
    let req = MarketMoversRequest {
        market_type: Some(MarketType::Crypto),
        top: Some(3),
    };
    client.get_market_movers(&req).await.unwrap();
}

#[tokio::test]
async fn test_get_market_movers_defaults_to_stocks() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1beta1/screener/stocks/movers"))
        .respond_with(ResponseTemplate::new(200).set_body_string(MOVERS_JSON))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::screener_client(&server.uri());
    let movers = client
        .get_market_movers(&MarketMoversRequest::default())
        .await
        .unwrap();
    assert!(!movers.gainers.is_empty());
    assert_eq!(movers.market_type, Some(MarketType::Stocks));
}
