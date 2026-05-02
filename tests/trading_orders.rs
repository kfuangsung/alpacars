mod common;

use alpacars::trading::enums::{OrderSide, OrderStatus, TimeInForce};
use alpacars::trading::models::{CancelOrderResponse, Order};
use alpacars::trading::requests::{GetOrdersRequest, OrderRequest, ReplaceOrderRequest};

use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ORDER_JSON: &str = r#"{
  "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
  "client_order_id": "eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4",
  "created_at": "2021-03-16T18:38:01.942282Z",
  "updated_at": "2021-03-16T18:38:01.942282Z",
  "submitted_at": "2021-03-16T18:38:01.937734Z",
  "filled_at": null,
  "expired_at": null,
  "canceled_at": null,
  "failed_at": null,
  "replaced_at": null,
  "replaced_by": null,
  "replaces": null,
  "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
  "symbol": "AAPL",
  "asset_class": "us_equity",
  "notional": null,
  "qty": "1",
  "filled_qty": "0",
  "filled_avg_price": null,
  "order_class": "simple",
  "order_type": "market",
  "type": "market",
  "side": "buy",
  "time_in_force": "day",
  "limit_price": null,
  "stop_price": null,
  "status": "accepted",
  "extended_hours": false,
  "legs": null,
  "trail_percent": null,
  "trail_price": null,
  "hwm": null,
  "commission": "1.25"
}"#;

#[tokio::test]
async fn test_submit_market_order() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/orders"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = OrderRequest::market("AAPL", OrderSide::Buy, "1");
    let order = client.submit_order(&req).await.unwrap();

    assert_eq!(order.status, OrderStatus::Accepted);
    assert_eq!(order.symbol.as_deref(), Some("AAPL"));
}

#[tokio::test]
async fn test_submit_limit_order() {
    let server = MockServer::start().await;

    let limit_json = ORDER_JSON.replace(r#""order_type": "market""#, r#""order_type": "limit""#)
        .replace(r#""type": "market""#, r#""type": "limit""#)
        .replace(r#""limit_price": null"#, r#""limit_price": "300.00""#)
        .replace(r#""status": "accepted""#, r#""status": "accepted""#);

    Mock::given(method("POST"))
        .and(path("/v2/orders"))
        .respond_with(ResponseTemplate::new(200).set_body_string(limit_json))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = OrderRequest::limit("AAPL", OrderSide::Buy, "1", "300.00", TimeInForce::Day);
    let order = client.submit_order(&req).await.unwrap();

    assert_eq!(order.status, OrderStatus::Accepted);
}

#[tokio::test]
async fn test_get_orders() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/orders"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            format!("[{}]", ORDER_JSON),
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetOrdersRequest::default();
    let orders = client.get_orders(Some(&req)).await.unwrap();

    assert_eq!(orders.len(), 1);
    assert!(matches!(orders[0], Order { .. }));
    assert_eq!(orders[0].symbol.as_deref(), Some("AAPL"));
}

#[tokio::test]
async fn test_get_order_by_id() {
    let server = MockServer::start().await;
    let order_id = Uuid::parse_str("61e69015-8549-4bfd-b9c3-01e75843f47d").unwrap();

    Mock::given(method("GET"))
        .and(path(format!("/v2/orders/{}", order_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let order = client.get_order_by_id(&order_id, None).await.unwrap();

    assert_eq!(order.id, order_id);
    assert_eq!(order.status, OrderStatus::Accepted);
}

#[tokio::test]
async fn test_replace_order() {
    let server = MockServer::start().await;
    let order_id = Uuid::parse_str("61e69015-8549-4bfd-b9c3-01e75843f47d").unwrap();

    Mock::given(method("PATCH"))
        .and(path(format!("/v2/orders/{}", order_id)))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = ReplaceOrderRequest { qty: Some("2".to_string()), ..Default::default() };
    let order = client.replace_order_by_id(&order_id, &req).await.unwrap();

    assert_eq!(order.id, order_id);
}

#[tokio::test]
async fn test_cancel_order_by_id() {
    let server = MockServer::start().await;
    let order_id = Uuid::parse_str("61e69015-8549-4bfd-b9c3-01e75843f47d").unwrap();

    Mock::given(method("DELETE"))
        .and(path(format!("/v2/orders/{}", order_id)))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    client.cancel_order_by_id(&order_id).await.unwrap();
}

#[tokio::test]
async fn test_cancel_order_returns_api_error_on_422() {
    let server = MockServer::start().await;
    let order_id = Uuid::parse_str("61e69015-8549-4bfd-b9c3-01e75843f47d").unwrap();

    Mock::given(method("DELETE"))
        .and(path(format!("/v2/orders/{}", order_id)))
        .respond_with(ResponseTemplate::new(422).set_body_string(
            r#"{"code": 42210000, "message": "order is not cancelable"}"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let result = client.cancel_order_by_id(&order_id).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_cancel_all_orders() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v2/orders"))
        .respond_with(ResponseTemplate::new(207).set_body_string(r#"[
          {"id": "497f6eca-6276-4993-bfeb-53cbbbba6f08", "status": 200},
          {"id": "72249bb6-6c89-4ea7-b8cf-73f1a140812b", "status": 404,
           "body": {"code": 40410000, "message": "order not found"}}
        ]"#))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let responses = client.cancel_orders().await.unwrap();

    assert_eq!(responses.len(), 2);
    assert!(matches!(responses[0], CancelOrderResponse { .. }));
    assert_eq!(responses[0].id, Uuid::parse_str("497f6eca-6276-4993-bfeb-53cbbbba6f08").unwrap());
    assert_eq!(responses[0].status, 200);
    assert_eq!(responses[1].status, 404);
}
