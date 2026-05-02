mod common;

use alpacars::trading::models::{ClosePositionResponse, Order, Position};
use alpacars::trading::requests::ClosePositionRequest;
use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const POSITION_JSON: &str = r#"{
    "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
    "symbol": "AAPL",
    "exchange": "NASDAQ",
    "asset_class": "us_equity",
    "avg_entry_price": "100.0",
    "qty": "5",
    "side": "long",
    "market_value": "600.0",
    "cost_basis": "500.0",
    "unrealized_pl": "100.0",
    "unrealized_plpc": "0.20",
    "unrealized_intraday_pl": "10.0",
    "unrealized_intraday_plpc": "0.0084",
    "current_price": "120.0",
    "lastday_price": "119.0",
    "change_today": "0.0084"
}"#;

const ORDER_JSON: &str = r#"{
  "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
  "client_order_id": "eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4",
  "created_at": "2021-03-16T18:38:01.942282Z",
  "updated_at": "2021-03-16T18:38:01.942282Z",
  "submitted_at": "2021-03-16T18:38:01.937734Z",
  "filled_at": null, "expired_at": null, "canceled_at": null,
  "failed_at": null, "replaced_at": null, "replaced_by": null, "replaces": null,
  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
  "symbol": "AAPL",
  "asset_class": "us_equity",
  "notional": null, "qty": "1", "filled_qty": "0", "filled_avg_price": null,
  "order_class": "simple", "order_type": "market", "type": "market",
  "side": "buy", "time_in_force": "day",
  "limit_price": null, "stop_price": null, "status": "accepted",
  "extended_hours": false, "legs": null,
  "trail_percent": null, "trail_price": null, "hwm": null
}"#;

#[tokio::test]
async fn test_get_all_positions() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/positions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            format!("[{}]", POSITION_JSON),
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let positions = client.get_all_positions().await.unwrap();

    assert_eq!(positions.len(), 1);
    assert!(matches!(positions[0], Position { .. }));
    assert_eq!(positions[0].symbol, "AAPL");
    assert_eq!(positions[0].qty, "5");
}

#[tokio::test]
async fn test_get_open_position_by_symbol() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/positions/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(POSITION_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let pos = client.get_open_position("AAPL").await.unwrap();

    assert_eq!(pos.symbol, "AAPL");
    assert_eq!(pos.avg_entry_price, "100.0");
}

#[tokio::test]
async fn test_get_open_position_by_id() {
    let server = MockServer::start().await;
    let asset_id = Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/v2/positions/{}", asset_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(POSITION_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let pos = client.get_open_position(&asset_id.to_string()).await.unwrap();

    assert_eq!(pos.asset_id, asset_id);
}

#[tokio::test]
async fn test_close_all_positions() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/positions"))
        .respond_with(ResponseTemplate::new(207).set_body_string(r#"[
            {
                "symbol": "AAPL",
                "status": 200,
                "body": {
                    "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
                    "client_order_id": "eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4",
                    "created_at": "2021-03-16T18:38:01.942282Z",
                    "updated_at": "2021-03-16T18:38:01.942282Z",
                    "submitted_at": "2021-03-16T18:38:01.937734Z",
                    "filled_at": null, "expired_at": null, "canceled_at": null,
                    "failed_at": null, "replaced_at": null,
                    "replaced_by": null, "replaces": null,
                    "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
                    "symbol": "AAPL", "asset_class": "us_equity",
                    "notional": null, "qty": "5", "filled_qty": "0",
                    "filled_avg_price": null, "order_class": "simple",
                    "order_type": "market", "type": "market", "side": "sell",
                    "time_in_force": "day", "limit_price": null, "stop_price": null,
                    "status": "pending_new", "extended_hours": false, "legs": null,
                    "trail_percent": null, "trail_price": null, "hwm": null
                }
            }
        ]"#))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let results = client.close_all_positions(Some(true)).await.unwrap();

    assert_eq!(results.len(), 1);
    assert!(matches!(results[0], ClosePositionResponse { .. }));
    assert_eq!(results[0].symbol.as_deref(), Some("AAPL"));
    assert_eq!(results[0].status, Some(200));
}

#[tokio::test]
async fn test_close_position_by_symbol() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/positions/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let order = client.close_position("AAPL", None).await.unwrap();

    assert!(matches!(order, Order { .. }));
    assert_eq!(order.symbol.as_deref(), Some("AAPL"));
}

#[tokio::test]
async fn test_close_position_with_qty() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/positions/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let opts = ClosePositionRequest { qty: Some("3".to_string()), percentage: None };
    let order = client.close_position("AAPL", Some(&opts)).await.unwrap();

    assert!(matches!(order, Order { .. }));
}
