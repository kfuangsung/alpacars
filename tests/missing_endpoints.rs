//! Endpoints added in 0.6.2, checked against the Trading OpenAPI spec and the Broker references
//! (getaccountactivitiesbytype, optiondonotexercise / optiondonotexercise-1,
//! getorderbyclientorderidforaccount, get-v1-trading-accounts-account_id-orders-estimation).

mod common;

use alpacars::broker::requests::{
    EstimateOrderRequest, GetAccountActivitiesRequest as BrokerActivities,
};
use alpacars::trading::enums::{ActivityType, OrderSide, OrderType, TimeInForce};
use alpacars::trading::requests::{GetMarketCalendarRequest, UpdateWatchlistRequest};
use chrono::NaiveDate;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";

const WATCHLIST_JSON: &str = r#"{
  "id": "ec1a8a3a-6b3e-4f5d-9b8e-2a6f2e1c9d40",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "name": "Tech",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "assets": []
}"#;

const ORDER_JSON: &str = r#"{
  "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
  "client_order_id": "my-order-1",
  "order_class": "simple",
  "time_in_force": "day",
  "status": "new",
  "extended_hours": false,
  "symbol": "AAPL"
}"#;

fn account() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

// ── Trading ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_account_activities_by_type() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/account/activities/DIV"))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::trading_client(&server.uri());
    client
        .get_account_activities_by_type(&ActivityType::Div, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_do_not_exercise_options_position() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v2/positions/AAPL250620C00100000/do-not-exercise"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::trading_client(&server.uri());
    client
        .do_not_exercise_options_position("AAPL250620C00100000")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_watchlist_by_name_crud() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/watchlists:by_name"))
        .and(query_param("name", "Tech"))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v2/watchlists:by_name"))
        .and(query_param("name", "Tech"))
        .and(body_json(json!({"symbol": "AAPL"})))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/v2/watchlists:by_name"))
        .and(query_param("name", "Tech"))
        .and(body_json(
            json!({"name": "Tech", "symbols": ["AAPL", "MSFT"]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(WATCHLIST_JSON))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v2/watchlists:by_name"))
        .and(query_param("name", "Tech"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    assert_eq!(
        client.get_watchlist_by_name("Tech").await.unwrap().name,
        "Tech"
    );
    client
        .add_asset_to_watchlist_by_name("Tech", "AAPL")
        .await
        .unwrap();
    let update = UpdateWatchlistRequest {
        name: Some("Tech".to_string()),
        symbols: Some(vec!["AAPL".to_string(), "MSFT".to_string()]),
    };
    client
        .update_watchlist_by_name("Tech", &update)
        .await
        .unwrap();
    client.delete_watchlist_by_name("Tech").await.unwrap();
}

/// `GET /v3/calendar/{market}`; the day values are the `calendar_day` property examples.
#[tokio::test]
async fn test_get_market_calendar() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/calendar/XNYS"))
        .and(query_param("start", "2025-01-02"))
        .and(query_param("timezone", "America/New_York"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "market": {"acronym": "NYSE", "name": "New York Stock Exchange", "timezone": "America/New_York", "mic": "XNYS"},
              "calendar": [{
                "date": "2025-01-02",
                "pre_start": "2025-01-02T04:00:00-05:00",
                "pre_end": "2025-01-02T09:30:00-05:00",
                "core_start": "2025-01-02T09:30:00-05:00",
                "core_end": "2025-01-02T16:00:00-05:00",
                "post_start": "2025-01-02T16:00:00-05:00",
                "post_end": "2025-01-02T20:00:00-05:00",
                "settlement_date": "2025-01-03"
              }]
            }"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let filter = GetMarketCalendarRequest {
        start: NaiveDate::from_ymd_opt(2025, 1, 2),
        end: None,
        timezone: Some("America/New_York".to_string()),
    };
    let cal = client
        .get_market_calendar("XNYS", Some(&filter))
        .await
        .unwrap();
    assert_eq!(cal.market.acronym, "NYSE");
    let day = &cal.calendar[0];
    assert_eq!(day.core_start.to_rfc3339(), "2025-01-02T14:30:00+00:00");
    assert_eq!(day.lunch_start, None);
}

// ── Broker ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_broker_get_account_activities_by_type() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/accounts/activities/FILL"))
        .and(query_param("account_id", ACCOUNT_ID))
        .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::broker_client(&server.uri());
    let filter = BrokerActivities {
        account_id: Some(account()),
        ..Default::default()
    };
    client
        .get_account_activities_by_type(&ActivityType::Fill, Some(&filter))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_broker_do_not_exercise() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!(
            "/v1/trading/accounts/{ACCOUNT_ID}/positions/AAPL250620C00100000/do-not-exercise"
        )))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::broker_client(&server.uri());
    client
        .do_not_exercise_options_position_for_account(&account(), "AAPL250620C00100000")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_broker_get_order_by_client_order_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/trading/accounts/{ACCOUNT_ID}/orders:by_client_order_id"
        )))
        .and(query_param("client_order_id", "my-order-1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::broker_client(&server.uri());
    let order = client
        .get_order_for_account_by_client_order_id(&account(), "my-order-1")
        .await
        .unwrap();
    assert_eq!(order.client_order_id, "my-order-1");
}

/// Body must equal the estimation reference's "Example 1".
#[tokio::test]
async fn test_broker_estimate_order() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!(
            "/v1/trading/accounts/{ACCOUNT_ID}/orders/estimation"
        )))
        .and(body_json(json!({
            "notional": "40", "side": "buy", "swap_fee_bps": 45,
            "symbol": "AAPL", "time_in_force": "day", "type": "market"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(ORDER_JSON))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::broker_client(&server.uri());
    let req = EstimateOrderRequest {
        symbol: "AAPL".to_string(),
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        time_in_force: TimeInForce::Day,
        notional: "40".to_string(),
        swap_fee_bps: Some(45),
    };
    client
        .estimate_order_for_account(&account(), &req)
        .await
        .unwrap();
}
