mod common;

use alpacars::broker::requests::GetAccountActivitiesRequest as BrokerActivitiesRequest;
use alpacars::trading::enums::{ActivityCategory, ActivityType};
use alpacars::trading::models::AccountActivity;
use alpacars::trading::requests::GetAccountActivitiesRequest;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Both examples come from the TradingActivities / NonTradeActivities schemas on
// docs.alpaca.markets/us/reference/getaccountactivities-2, not from the crate's types.
const FILL_JSON: &str = r#"{
  "activity_type": "FILL",
  "cum_qty": "2",
  "id": "20220202135509981::2d7be4ff-d1f3-43e9-856a-0f5cf5c5088e",
  "leaves_qty": "0",
  "order_id": "b5abe576-6a8a-49f3-a353-46b72c1ccae9",
  "order_status": "filled",
  "price": "174.78",
  "qty": "2",
  "side": "buy",
  "symbol": "AAPL",
  "transaction_time": "2022-02-02T18:55:09.981482Z",
  "type": "fill"
}"#;

const DIV_JSON: &str = r#"{
  "activity_sub_type": "SDIV",
  "activity_type": "DIV",
  "created_at": "2021-05-10T14:01:04.650275Z",
  "currency": "USD",
  "date": "2019-08-01",
  "id": "20190801011955195::5f596936-6f23-4cef-bdf1-3806aae57dbf",
  "net_amount": "1.02",
  "per_share_amount": "0.51",
  "qty": "2",
  "status": "executed",
  "symbol": "T"
}"#;

const PAGE_TOKEN: &str = "20220202135509981::2d7be4ff-d1f3-43e9-856a-0f5cf5c5088e";
const ORDER_ID: &str = "b5abe576-6a8a-49f3-a353-46b72c1ccae9";
const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";

#[test]
fn test_activity_type_wire_values_are_uppercase() {
    let cases = [
        (ActivityType::Fill, "FILL"),
        (ActivityType::Jnlc, "JNLC"),
        (ActivityType::Divcgl, "DIVCGL"),
        (ActivityType::Oct, "OCT"),
        (ActivityType::Reo, "REO"),
        (ActivityType::Cgd, "CGD"),
        (ActivityType::Fopt, "FOPT"),
    ];
    for (variant, wire) in cases {
        assert_eq!(serde_json::to_value(&variant).unwrap(), json!(wire));
        let parsed: ActivityType = serde_json::from_value(json!(wire)).unwrap();
        assert_eq!(parsed, variant);
    }
}

#[test]
fn test_account_activity_picks_trade_or_non_trade() {
    match serde_json::from_str::<AccountActivity>(FILL_JSON).unwrap() {
        AccountActivity::Trade(t) => {
            assert_eq!(t.activity_type, ActivityType::Fill);
            assert_eq!(t.price.as_deref(), Some("174.78"));
        }
        other => panic!("FILL should be a trade activity, got {other:?}"),
    }
    match serde_json::from_str::<AccountActivity>(DIV_JSON).unwrap() {
        AccountActivity::NonTrade(n) => {
            assert_eq!(n.activity_type, ActivityType::Div);
            assert_eq!(n.net_amount.as_deref(), Some("1.02"));
        }
        other => panic!("DIV should be a non-trade activity, got {other:?}"),
    }
}

#[tokio::test]
async fn test_get_account_activities_query() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/account/activities"))
        .and(query_param("activity_types", "FILL,DIV"))
        .and(query_param("order_id", ORDER_ID))
        .and(query_param("page_token", PAGE_TOKEN))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!("[{FILL_JSON},{DIV_JSON}]")),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let filter = GetAccountActivitiesRequest {
        activity_types: Some(vec![ActivityType::Fill, ActivityType::Div]),
        order_id: Some(ORDER_ID.parse::<Uuid>().unwrap()),
        page_token: Some(PAGE_TOKEN.to_string()),
        ..Default::default()
    };
    let activities = client.get_account_activities(Some(&filter)).await.unwrap();

    assert_eq!(activities.len(), 2);
}

#[tokio::test]
async fn test_broker_get_account_activities_path_and_query() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/accounts/activities"))
        .and(query_param("account_id", ACCOUNT_ID))
        .and(query_param("activity_types", "FILL"))
        .and(query_param("category", "trade_activity"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{FILL_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = BrokerActivitiesRequest {
        account_id: Some(ACCOUNT_ID.parse().unwrap()),
        activity_types: Some(vec![ActivityType::Fill]),
        category: Some(ActivityCategory::TradeActivity),
        ..Default::default()
    };
    let activities = client.get_account_activities(Some(&filter)).await.unwrap();

    assert_eq!(activities.len(), 1);
}
