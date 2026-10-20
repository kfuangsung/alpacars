//! Per-account orders, positions and options exercise live under /v1/trading/accounts/{id}/...
//! (createorderforaccount, getallordersforaccount, getpositionsforaccount,
//! closepositionforaccountbysymbol, optionexercise-1, ... on docs.alpaca.markets/us/reference).
//! The crate previously called /v1/accounts/{id}/..., which does not exist.

mod common;

use alpacars::broker::requests::GetOrdersForAccountRequest;
use alpacars::trading::enums::{AssetClass, OrderSide, QueryOrderStatus};
use alpacars::trading::requests::{ClosePositionRequest, OrderRequest, ReplaceOrderRequest};
use uuid::Uuid;
use wiremock::matchers::{body_partial_json, method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";
const ORDER_ID: &str = "d1143025-89fc-4952-8936-db2409d899f3";

// The order in the PositionClosedResponse example (closeallpositionsforaccount).
const ORDER_JSON: &str = r#"{
  "asset_class": "us_equity",
  "asset_id": "a4778bc8-fad1-47b7-87fe-d5cde10d43f4",
  "canceled_at": null,
  "client_order_id": "17dbfab4-cb86-4e0a-8fa6-f0606b0a9a4e",
  "created_at": "2022-05-13T16:25:29.336330998Z",
  "expired_at": null,
  "extended_hours": false,
  "failed_at": null,
  "filled_at": null,
  "filled_avg_price": null,
  "filled_qty": "0",
  "hwm": null,
  "id": "d1143025-89fc-4952-8936-db2409d899f3",
  "legs": null,
  "limit_price": null,
  "notional": null,
  "order_class": "",
  "order_type": "market",
  "qty": "4",
  "replaced_at": null,
  "replaced_by": null,
  "replaces": null,
  "side": "sell",
  "source": null,
  "status": "accepted",
  "stop_price": null,
  "submitted_at": "2022-05-13T16:25:29.335776073Z",
  "symbol": "TSLA",
  "time_in_force": "day",
  "trail_percent": null,
  "trail_price": null,
  "type": "market",
  "updated_at": "2022-05-13T16:25:29.336330998Z"
}"#;

// The Position schema example (getpositionsforaccount).
const POSITION_JSON: &str = r#"{
  "asset_class": "us_equity",
  "asset_id": "93f58d0b-6c53-432d-b8ce-2bad264dbd94",
  "asset_marginable": false,
  "avg_entry_price": "172.08",
  "change_today": "0.0189483657034581",
  "cost_basis": "688.32",
  "current_price": "172.08",
  "exchange": "NASDAQ",
  "lastday_price": "168.88",
  "market_value": "688.32",
  "qty": "4",
  "qty_available": "4",
  "side": "long",
  "symbol": "AAPL",
  "unrealized_intraday_pl": "0",
  "unrealized_intraday_plpc": "0",
  "unrealized_pl": "0",
  "unrealized_plpc": "0"
}"#;

fn account() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

fn order_id() -> Uuid {
    ORDER_ID.parse().unwrap()
}

fn base() -> String {
    format!("/v1/trading/accounts/{ACCOUNT_ID}")
}

async fn mock(server: &MockServer, verb: &str, p: String, status: u16, body: &str) {
    Mock::given(method(verb))
        .and(path(p))
        .respond_with(ResponseTemplate::new(status).set_body_string(body))
        .expect(1)
        .mount(server)
        .await;
}

#[tokio::test]
async fn test_submit_order_for_account() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/orders", base())))
        .and(body_partial_json(serde_json::json!({
            "symbol": "TSLA", "qty": "4", "side": "sell", "type": "market", "time_in_force": "day"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let order = OrderRequest::market("TSLA", OrderSide::Sell, "4");
    let resp = client
        .submit_order_for_account(&account(), &order)
        .await
        .unwrap();
    assert_eq!(resp.id, order_id());
}

#[tokio::test]
async fn test_get_orders_for_account() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/orders", base())))
        .and(query_param("status", "open"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{ORDER_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetOrdersForAccountRequest {
        status: Some(QueryOrderStatus::Open),
        ..Default::default()
    };
    let orders = client
        .get_orders_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(orders.len(), 1);
}

/// getallordersforaccount: asset_class (comma-separated array), qty_above / qty_below, subtag.
#[tokio::test]
async fn test_get_orders_for_account_filters() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/orders", base())))
        .and(query_param("asset_class", "us_equity,us_option"))
        .and(query_param("qty_above", "1"))
        .and(query_param("qty_below", "100"))
        .and(query_param("subtag", "desk-7"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{ORDER_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetOrdersForAccountRequest {
        asset_class: Some(vec![AssetClass::UsEquity, AssetClass::UsOption]),
        qty_above: Some("1".to_string()),
        qty_below: Some("100".to_string()),
        subtag: Some("desk-7".to_string()),
        ..Default::default()
    };
    let orders = client
        .get_orders_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(orders.len(), 1);
}

#[tokio::test]
async fn test_get_order_for_account_by_id() {
    let server = MockServer::start().await;
    mock(
        &server,
        "GET",
        format!("{}/orders/{ORDER_ID}", base()),
        200,
        ORDER_JSON,
    )
    .await;

    let client = common::broker_client(&server.uri());
    let order = client
        .get_order_for_account_by_id(&account(), &order_id())
        .await
        .unwrap();
    assert_eq!(order.symbol.as_deref(), Some("TSLA"));
}

#[tokio::test]
async fn test_replace_order_for_account_by_id() {
    let server = MockServer::start().await;
    mock(
        &server,
        "PATCH",
        format!("{}/orders/{ORDER_ID}", base()),
        200,
        ORDER_JSON,
    )
    .await;

    let client = common::broker_client(&server.uri());
    let data = ReplaceOrderRequest {
        qty: Some("2".to_string()),
        ..Default::default()
    };
    client
        .replace_order_for_account_by_id(&account(), &order_id(), &data)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_cancel_orders_for_account() {
    let server = MockServer::start().await;
    // The 207 example on deleteallordersforaccount.
    mock(
        &server,
        "DELETE",
        format!("{}/orders", base()),
        207,
        r#"[{"id": "872994c5-0620-40ad-8ae0-f224b9948df5", "status": 200}]"#,
    )
    .await;

    let client = common::broker_client(&server.uri());
    let results = client.cancel_orders_for_account(&account()).await.unwrap();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_cancel_order_for_account_by_id() {
    let server = MockServer::start().await;
    mock(
        &server,
        "DELETE",
        format!("{}/orders/{ORDER_ID}", base()),
        204,
        "",
    )
    .await;

    let client = common::broker_client(&server.uri());
    client
        .cancel_order_for_account_by_id(&account(), &order_id())
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_all_positions_for_account() {
    let server = MockServer::start().await;
    mock(
        &server,
        "GET",
        format!("{}/positions", base()),
        200,
        &format!("[{POSITION_JSON}]"),
    )
    .await;

    let client = common::broker_client(&server.uri());
    let positions = client
        .get_all_positions_for_account(&account())
        .await
        .unwrap();
    assert_eq!(positions[0].symbol, "AAPL");
}

#[tokio::test]
async fn test_get_open_position_for_account() {
    let server = MockServer::start().await;
    mock(
        &server,
        "GET",
        format!("{}/positions/AAPL", base()),
        200,
        POSITION_JSON,
    )
    .await;

    let client = common::broker_client(&server.uri());
    let position = client
        .get_open_position_for_account(&account(), "AAPL")
        .await
        .unwrap();
    assert_eq!(position.qty, "4");
}

#[tokio::test]
async fn test_close_all_positions_for_account() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/positions", base())))
        .and(query_param("cancel_orders", "true"))
        .respond_with(ResponseTemplate::new(207).set_body_string(format!(
            r#"[{{"body": {ORDER_JSON}, "status": 200, "symbol": "TSLA"}}]"#
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let results = client
        .close_all_positions_for_account(&account(), Some(true))
        .await
        .unwrap();
    assert_eq!(results[0].symbol.as_deref(), Some("TSLA"));
    assert_eq!(results[0].status, Some(200));
}

/// `qty` / `percentage` are query parameters (closepositionforaccountbysymbol). Sent as a body
/// they are ignored and the whole position is closed.
#[tokio::test]
async fn test_close_position_for_account_sends_qty_as_query() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/positions/TSLA", base())))
        .and(query_param("percentage", "50"))
        .and(query_param_is_missing("qty"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let opts = ClosePositionRequest {
        qty: None,
        percentage: Some("50".to_string()),
    };
    client
        .close_position_for_account(&account(), "TSLA", Some(&opts))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_exercise_options_position_for_account() {
    let server = MockServer::start().await;
    mock(
        &server,
        "POST",
        format!("{}/positions/AAPL250620C00100000/exercise", base()),
        200,
        r#"{"qty_exercised": "1", "qty_remaining": "0"}"#,
    )
    .await;

    let client = common::broker_client(&server.uri());
    client
        .exercise_options_position_for_account_by_id(&account(), "AAPL250620C00100000")
        .await
        .unwrap();
}
