mod common;

use alpacars::trading::enums::LocateStatus;
use alpacars::trading::requests::{CreateLocateRequest, GetLocateQuotesRequest, GetLocatesRequest};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const LOCATE_JSON: &str = r#"{
  "id": "0f4361a4-90c6-4a3f-b3f1-6d5e78c8a1b2",
  "symbol": "GME",
  "status": "active",
  "requested_qty": 100,
  "located_qty": 100,
  "located_price": "0.05",
  "total_fee": "5.00",
  "limit_price": "0.10",
  "all_or_none": true,
  "rejection_reason": null,
  "expires_at": "2026-07-17T20:00:00Z",
  "created_at": "2026-07-17T10:00:00Z"
}"#;

#[tokio::test]
async fn test_create_locate() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/locates"))
        .respond_with(ResponseTemplate::new(200).set_body_string(LOCATE_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = CreateLocateRequest {
        symbol: "GME".to_string(),
        qty: 100,
        limit_price: Some("0.10".to_string()),
        all_or_none: Some(true),
    };
    let locate = client.create_locate(&req).await.unwrap();

    assert_eq!(locate.symbol, "GME");
    assert_eq!(locate.status, LocateStatus::Active);
    assert_eq!(locate.requested_qty, 100);
    assert_eq!(locate.located_qty, Some(100));
    assert_eq!(locate.total_fee.as_deref(), Some("5.00"));
}

#[tokio::test]
async fn test_get_locates() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/locates"))
        .and(query_param("status", "active"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{"locates": [{}], "next_page_token": null}}"#,
            LOCATE_JSON
        )))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetLocatesRequest {
        status: Some(LocateStatus::Active),
        ..Default::default()
    };
    let resp = client.get_locates(Some(&req)).await.unwrap();

    assert_eq!(resp.locates.len(), 1);
    assert_eq!(resp.locates[0].symbol, "GME");
    assert!(resp.next_page_token.is_none());
}

#[tokio::test]
async fn test_get_locate_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/locates/0f4361a4-90c6-4a3f-b3f1-6d5e78c8a1b2"))
        .respond_with(ResponseTemplate::new(200).set_body_string(LOCATE_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let locate = client
        .get_locate("0f4361a4-90c6-4a3f-b3f1-6d5e78c8a1b2")
        .await
        .unwrap();

    assert_eq!(locate.id, "0f4361a4-90c6-4a3f-b3f1-6d5e78c8a1b2");
    assert_eq!(locate.all_or_none, true);
}

#[tokio::test]
async fn test_get_locate_quotes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/locates/quotes"))
        .and(query_param("symbols", "GME,AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "quotes": [
                {"symbol": "GME", "available_qty": 5000, "price": "0.05", "quoted_at": "2026-07-17T10:00:00Z"}
              ],
              "errors": [
                {"symbol": "AAPL", "code": "easy_to_borrow", "message": "AAPL is easy to borrow; no locate required"}
              ]
            }"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetLocateQuotesRequest {
        symbols: "GME,AAPL".to_string(),
    };
    let resp = client.get_locate_quotes(&req).await.unwrap();

    assert_eq!(resp.quotes.len(), 1);
    assert_eq!(resp.quotes[0].available_qty, 5000);
    let errors = resp.errors.unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "easy_to_borrow");
}
