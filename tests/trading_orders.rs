mod common;

use alpacars::trading::enums::{
    AdvancedAlgorithm, AssetClass, OrderDestination, OrderSide, OrderStatus, TimeInForce,
};
use alpacars::trading::models::{CancelOrderResponse, Order};
use alpacars::trading::requests::{
    AdvancedInstructions, GetOrdersRequest, OrderRequest, ReplaceOrderRequest,
};

use uuid::Uuid;
use wiremock::matchers::{method, path, query_param};
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

    let limit_json = ORDER_JSON
        .replace(r#""order_type": "market""#, r#""order_type": "limit""#)
        .replace(r#""type": "market""#, r#""type": "limit""#)
        .replace(r#""limit_price": null"#, r#""limit_price": "300.00""#);

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
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{}]", ORDER_JSON)))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetOrdersRequest::default();
    let orders = client.get_orders(Some(&req)).await.unwrap();

    assert_eq!(orders.len(), 1);
    assert!(matches!(orders[0], Order { .. }));
    assert_eq!(orders[0].symbol.as_deref(), Some("AAPL"));
}

/// getallorders-1: `asset_class` is a comma-separated array; `before_order_id` pages by order.
#[tokio::test]
async fn test_get_orders_asset_class_and_order_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/orders"))
        .and(query_param("asset_class", "us_option,crypto"))
        .and(query_param(
            "before_order_id",
            "61e69015-8549-4bfd-b9c3-01e75843f47d",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{}]", ORDER_JSON)))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetOrdersRequest {
        asset_class: Some(vec![AssetClass::UsOption, AssetClass::Crypto]),
        before_order_id: Some("61e69015-8549-4bfd-b9c3-01e75843f47d".parse().unwrap()),
        ..Default::default()
    };
    let orders = client.get_orders(Some(&req)).await.unwrap();
    assert_eq!(orders.len(), 1);
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
    let req = ReplaceOrderRequest {
        qty: Some("2".to_string()),
        ..Default::default()
    };
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
        .respond_with(
            ResponseTemplate::new(422)
                .set_body_string(r#"{"code": 42210000, "message": "order is not cancelable"}"#),
        )
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
        .respond_with(ResponseTemplate::new(207).set_body_string(
            r#"[
          {"id": "497f6eca-6276-4993-bfeb-53cbbbba6f08", "status": 200},
          {"id": "72249bb6-6c89-4ea7-b8cf-73f1a140812b", "status": 404,
           "body": {"code": 40410000, "message": "order not found"}}
        ]"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let responses = client.cancel_orders().await.unwrap();

    assert_eq!(responses.len(), 2);
    assert!(matches!(responses[0], CancelOrderResponse { .. }));
    assert_eq!(
        responses[0].id,
        Uuid::parse_str("497f6eca-6276-4993-bfeb-53cbbbba6f08").unwrap()
    );
    assert_eq!(responses[0].status, 200);
    assert_eq!(responses[1].status, 404);
}

#[test]
fn test_order_request_advanced_instructions_serialization() {
    let mut req = OrderRequest::market("AAPL", OrderSide::Buy, "100");
    let json = serde_json::to_value(&req).unwrap();
    assert!(json.get("advanced_instructions").is_none());

    req.advanced_instructions = Some(AdvancedInstructions {
        algorithm: Some(AdvancedAlgorithm::Dma),
        destination: Some(OrderDestination::Iex),
        display_qty: Some("50".to_string()),
    });
    let json = serde_json::to_value(&req).unwrap();
    let ai = &json["advanced_instructions"];
    assert_eq!(ai["algorithm"], "DMA");
    assert_eq!(ai["destination"], "IEX");
    assert_eq!(ai["display_qty"], "50");
}

/// postorder, request example `MultilegOptions` and response example
/// `MultilegOptionsOrderResponse`. An mleg order has no top-level `symbol` or `side`; the
/// crate used to send both, with a numeric `ratio_qty`. The response's parent order has
/// `"asset_id": ""`, which used to fail the whole parse.
#[tokio::test]
async fn test_submit_mleg_order_matches_reference() {
    use alpacars::trading::enums::{OrderClass, OrderType, PositionIntent};
    use alpacars::trading::requests::OptionLegRequest;
    use wiremock::matchers::body_json;

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2/orders"))
        .and(body_json(serde_json::json!({
            "legs": [
                {"position_intent": "buy_to_open", "ratio_qty": "3", "side": "buy", "symbol": "AAPL241213C00250000"},
                {"position_intent": "sell_to_open", "ratio_qty": "1", "side": "sell", "symbol": "AAPL241213C00260000"}
            ],
            "limit_price": "10",
            "order_class": "mleg",
            "qty": "3",
            "time_in_force": "day",
            "type": "limit"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(MLEG_ORDER_RESPONSE))
        .expect(1)
        .mount(&server)
        .await;

    let mut order = OrderRequest::mleg(
        "3",
        vec![
            OptionLegRequest {
                symbol: "AAPL241213C00250000".into(),
                ratio_qty: "3".into(),
                side: Some(OrderSide::Buy),
                position_intent: Some(PositionIntent::BuyToOpen),
            },
            OptionLegRequest {
                symbol: "AAPL241213C00260000".into(),
                ratio_qty: "1".into(),
                side: Some(OrderSide::Sell),
                position_intent: Some(PositionIntent::SellToOpen),
            },
        ],
    );
    order.order_type = OrderType::Limit;
    order.limit_price = Some("10".into());

    let client = common::trading_client(&server.uri());
    let placed = client.submit_order(&order).await.unwrap();
    assert_eq!(placed.order_class, OrderClass::Mleg);
    assert_eq!(placed.asset_id, None);
    assert_eq!(placed.legs.map(|l| l.len()), Some(2));
}

const MLEG_ORDER_RESPONSE: &str = r#"{
  "asset_class": "",
  "asset_id": "",
  "canceled_at": null,
  "client_order_id": "646b1fe6-b212-4f54-94c6-429e7bcdee04",
  "created_at": "2024-12-10T16:15:53.677230742Z",
  "expired_at": null,
  "extended_hours": false,
  "failed_at": null,
  "filled_at": "2024-12-10T16:15:53.694Z",
  "filled_avg_price": "1.28",
  "filled_qty": "1",
  "hwm": null,
  "id": "83f37e9f-6b1f-49ed-8fc6-3e6af716323f",
  "legs": [
    {
      "asset_class": "us_option",
      "asset_id": "f0ea14b2-8a49-4e9b-89d1-894c6e518a76",
      "canceled_at": null,
      "client_order_id": "cc8cc104-fe43-476c-b25c-f62650fb73f9",
      "created_at": "2024-12-10T16:15:53.677230742Z",
      "expired_at": null,
      "expires_at": "2024-12-10T21:00:00Z",
      "extended_hours": false,
      "failed_at": null,
      "filled_at": "2024-12-10T16:15:53.694Z",
      "filled_avg_price": "0.43",
      "filled_qty": "3",
      "hwm": null,
      "id": "df4ff24a-c58a-4e37-8b9f-ef32b83a11f2",
      "legs": null,
      "limit_price": null,
      "notional": null,
      "order_class": "mleg",
      "order_type": "",
      "position_intent": "buy_to_open",
      "qty": "3",
      "ratio_qty": "3",
      "replaced_at": null,
      "replaced_by": null,
      "replaces": null,
      "side": "buy",
      "source": null,
      "status": "filled",
      "stop_price": null,
      "submitted_at": "2024-12-10T16:15:53.684952901Z",
      "subtag": null,
      "symbol": "AAPL241213C00250000",
      "time_in_force": "day",
      "trail_percent": null,
      "trail_price": null,
      "type": "",
      "updated_at": "2024-12-10T16:15:53.725091158Z"
    },
    {
      "asset_class": "us_option",
      "asset_id": "f89940db-eeb1-46e6-8f9b-bb1f27a0b395",
      "canceled_at": null,
      "client_order_id": "0bd2d36d-4af2-4dfb-8418-333a5d5026fa",
      "created_at": "2024-12-10T16:15:53.677230742Z",
      "expired_at": null,
      "expires_at": "2024-12-10T21:00:00Z",
      "extended_hours": false,
      "failed_at": null,
      "filled_at": "2024-12-10T16:15:53.694Z",
      "filled_avg_price": "0.01",
      "filled_qty": "1",
      "hwm": null,
      "id": "ecd91110-c34d-4e9d-a7bf-a9c27c40f8b5",
      "legs": null,
      "limit_price": null,
      "notional": null,
      "order_class": "mleg",
      "order_type": "",
      "position_intent": "sell_to_open",
      "qty": "1",
      "ratio_qty": "1",
      "replaced_at": null,
      "replaced_by": null,
      "replaces": null,
      "side": "sell",
      "source": null,
      "status": "filled",
      "stop_price": null,
      "submitted_at": "2024-12-10T16:15:53.684952901Z",
      "subtag": null,
      "symbol": "AAPL241213C00260000",
      "time_in_force": "day",
      "trail_percent": null,
      "trail_price": null,
      "type": "",
      "updated_at": "2024-12-10T16:15:53.708983759Z"
    }
  ],
  "limit_price": "10",
  "notional": null,
  "order_class": "mleg",
  "order_type": "limit",
  "qty": "1",
  "replaced_at": null,
  "replaced_by": null,
  "replaces": null,
  "side": "",
  "source": null,
  "status": "filled",
  "stop_price": null,
  "submitted_at": "2024-12-10T16:15:53.684952901Z",
  "subtag": null,
  "symbol": "",
  "time_in_force": "day",
  "trail_percent": null,
  "trail_price": null,
  "type": "limit",
  "updated_at": "2024-12-10T16:15:53.725139688Z"
}"#;
