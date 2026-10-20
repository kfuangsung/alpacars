//! Per-account watchlists live under /v1/trading/accounts/{id}/watchlists (getallwatchlistsforaccount,
//! createwatchlistforaccount, getwatchlistforaccountbyid, replacewatchlistforaccountbyid,
//! deletewatchlistfromaccountbyid). The crate previously called /v1/accounts/{id}/watchlists.
//! Fixtures follow the reference Watchlist / WatchlistWithoutAsset schemas.

mod common;

use alpacars::broker::requests::{CreateWatchlistRequest, UpdateWatchlistRequest};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";
const WATCHLIST_ID: &str = "ec1a8a3a-6b3e-4f5d-9b8e-2a6f2e1c9d40";

const WATCHLIST_JSON: &str = r#"{
  "id": "ec1a8a3a-6b3e-4f5d-9b8e-2a6f2e1c9d40",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "name": "Tech",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "assets": [{
    "id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
    "class": "us_equity",
    "exchange": "NASDAQ",
    "symbol": "AAPL",
    "name": "Apple Inc.",
    "status": "active",
    "tradable": true,
    "marginable": true,
    "shortable": true,
    "fractionable": true
  }]
}"#;

fn account() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

fn watchlist() -> Uuid {
    WATCHLIST_ID.parse().unwrap()
}

fn base() -> String {
    format!("/v1/trading/accounts/{ACCOUNT_ID}/watchlists")
}

#[tokio::test]
async fn test_get_watchlists_for_account() {
    let server = MockServer::start().await;
    // List items are WatchlistWithoutAsset: no `assets` key.
    Mock::given(method("GET"))
        .and(path(base()))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"[{{"id": "{WATCHLIST_ID}", "account_id": "{ACCOUNT_ID}", "name": "Tech",
                 "created_at": "2026-10-20T10:00:00Z", "updated_at": "2026-10-20T10:00:00Z"}}]"#
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let lists = client.get_watchlists_for_account(&account()).await.unwrap();
    assert_eq!(lists[0].id, watchlist());
    assert!(lists[0].assets.is_none());
}

#[tokio::test]
async fn test_get_watchlist_for_account_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/{WATCHLIST_ID}", base())))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let list = client
        .get_watchlist_for_account_by_id(&account(), &watchlist())
        .await
        .unwrap();
    assert_eq!(list.assets.map(|a| a.len()), Some(1));
}

#[tokio::test]
async fn test_create_watchlist_for_account() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(base()))
        .and(body_json(json!({"name": "Tech", "symbols": ["AAPL"]})))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateWatchlistRequest {
        name: "Tech".to_string(),
        symbols: vec!["AAPL".to_string()],
    };
    let list = client
        .create_watchlist_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(list.name, "Tech");
}

#[tokio::test]
async fn test_create_watchlist_for_account_sends_empty_symbols() {
    // createwatchlistforaccount requires `symbols`; an empty list must still be sent.
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(base()))
        .and(body_json(json!({"name": "Tech", "symbols": []})))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateWatchlistRequest {
        name: "Tech".to_string(),
        symbols: vec![],
    };
    client
        .create_watchlist_for_account(&account(), &req)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_update_watchlist_for_account_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path(format!("{}/{WATCHLIST_ID}", base())))
        .and(body_json(
            json!({"name": "Tech", "symbols": ["AAPL", "MSFT"]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = UpdateWatchlistRequest {
        name: "Tech".to_string(),
        symbols: vec!["AAPL".to_string(), "MSFT".to_string()],
    };
    client
        .update_watchlist_for_account_by_id(&account(), &watchlist(), &req)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_delete_watchlist_from_account_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/{WATCHLIST_ID}", base())))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .delete_watchlist_from_account_by_id(&account(), &watchlist())
        .await
        .unwrap();
}
