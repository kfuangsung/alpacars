//! Broker assets, clock and calendar, checked against the getassets, getassetbysymbolorid,
//! legacyclock-1 and legacycalendar-1 references (`/v1/assets`, `/v1/clock`, `/v1/calendar`).
//! Fixtures are built from those references' schema examples.

mod common;

use alpacars::trading::enums::{AssetClass, AssetStatus};
use alpacars::trading::requests::{GetAssetsRequest, GetCalendarRequest};
use chrono::NaiveDate;
use wiremock::matchers::{method, path, query_param};
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
  "fractionable": true,
  "borrow_status": "easy_to_borrow",
  "margin_requirement_long": "100",
  "margin_requirement_short": "30",
  "attributes": ["ptp_no_exception", "ipo"]
}"#;

#[tokio::test]
async fn test_get_all_assets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/assets"))
        .and(query_param("status", "active"))
        .and(query_param("asset_class", "us_equity"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{ASSET_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetAssetsRequest {
        status: Some(AssetStatus::Active),
        asset_class: Some(AssetClass::UsEquity),
        ..Default::default()
    };
    let assets = client.get_all_assets(Some(&filter)).await.unwrap();
    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].symbol, "AAPL");
}

#[tokio::test]
async fn test_get_asset() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/assets/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ASSET_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let asset = client.get_asset("AAPL").await.unwrap();
    assert_eq!(asset.name.as_deref(), Some("Apple Inc. Common Stock"));
}

/// legacyclock-1's example uses a `-04:00` offset; it must parse into UTC.
#[tokio::test]
async fn test_get_clock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/clock"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"is_open": true, "next_close": "2025-06-24T16:00:00-04:00",
                "next_open": "2025-06-25T09:30:00-04:00", "timestamp": "2025-06-24T14:15:22-04:00"}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let clock = client.get_clock().await.unwrap();
    assert!(clock.is_open);
    assert_eq!(clock.next_close.to_rfc3339(), "2025-06-24T20:00:00+00:00");
}

#[tokio::test]
async fn test_get_calendar() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/calendar"))
        .and(query_param("start", "2025-01-02"))
        .and(query_param("end", "2025-01-03"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[{"close": "16:00", "date": "2025-01-02", "open": "09:30", "session_close": "2000",
                 "session_open": "0400", "settlement_date": "2025-01-03"}]"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetCalendarRequest {
        start: NaiveDate::from_ymd_opt(2025, 1, 2),
        end: NaiveDate::from_ymd_opt(2025, 1, 3),
        date_type: None,
    };
    let days = client.get_calendar(Some(&filter)).await.unwrap();
    assert_eq!(days[0].date, NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
    assert_eq!(days[0].session_open.as_deref(), Some("0400"));
}
