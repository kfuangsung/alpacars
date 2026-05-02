mod common;

use alpacars::trading::enums::{AssetClass, AssetExchange, AssetStatus};
use alpacars::trading::models::Asset;
use alpacars::trading::requests::GetAssetsRequest;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ASSET_JSON: &str = r#"{
  "id": "904837e3-3b76-47ec-b432-046db621571b",
  "class": "us_equity",
  "exchange": "NASDAQ",
  "symbol": "AAPL",
  "name": "Apple Inc. Common Stock",
  "status": "active",
  "tradable": true,
  "marginable": true,
  "shortable": true,
  "easy_to_borrow": true,
  "fractionable": true
}"#;

#[tokio::test]
async fn test_get_all_assets() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            format!("[{}]", ASSET_JSON),
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetAssetsRequest { status: Some(AssetStatus::Active), ..Default::default() };
    let assets = client.get_all_assets(Some(&req)).await.unwrap();

    assert_eq!(assets.len(), 1);
    assert!(matches!(assets[0], Asset { .. }));
    assert_eq!(assets[0].symbol, "AAPL");
    assert_eq!(assets[0].status, AssetStatus::Active);
}

#[tokio::test]
async fn test_get_all_assets_with_filters() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/assets"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            format!("[{}]", ASSET_JSON),
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetAssetsRequest {
        status: Some(AssetStatus::Active),
        asset_class: Some(AssetClass::UsEquity),
        exchange: Some(AssetExchange::Nasdaq),
        ..Default::default()
    };
    let assets = client.get_all_assets(Some(&req)).await.unwrap();

    assert_eq!(assets.len(), 1);
    let asset = &assets[0];
    assert_eq!(asset.asset_class, AssetClass::UsEquity);
    assert_eq!(asset.exchange, AssetExchange::Nasdaq);
}

#[tokio::test]
async fn test_get_asset_by_symbol() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/assets/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ASSET_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let asset = client.get_asset("AAPL").await.unwrap();

    assert!(matches!(asset, Asset { .. }));
    assert_eq!(asset.symbol, "AAPL");
    assert_eq!(asset.status, AssetStatus::Active);
    assert!(asset.tradable);
}
