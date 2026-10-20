//! `RestClient` behavior shared by every endpoint. The JSON verbs and `get_bytes` all go
//! through one send-and-retry path; these tests pin its retry and error handling.

mod common;

use alpacars::error::AlpacaError;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const CLOCK_JSON: &str = r#"{
  "timestamp": "2026-10-20T10:00:00Z",
  "is_open": true,
  "next_open": "2026-10-21T13:30:00Z",
  "next_close": "2026-10-20T20:00:00Z"
}"#;

/// A 429 is retried (after DEFAULT_RETRY_WAIT_SECS) and the retried request succeeds.
#[tokio::test]
async fn test_429_is_retried() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/clock"))
        .respond_with(ResponseTemplate::new(429))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/clock"))
        .respond_with(ResponseTemplate::new(200).set_body_string(CLOCK_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let clock = client.get_clock().await.unwrap();
    assert!(clock.is_open);
}

/// A non-retryable error is returned at once as `AlpacaError::Api`, with Alpaca's code/message.
#[tokio::test]
async fn test_non_retryable_error_is_not_retried() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/clock"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_string(r#"{"code": 40310000, "message": "forbidden"}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    match client.get_clock().await {
        Err(AlpacaError::Api {
            status_code,
            code,
            message,
        }) => {
            assert_eq!(status_code, 403);
            assert_eq!(code, 40310000);
            assert_eq!(message, "forbidden");
        }
        other => panic!("expected an API error, got {other:?}"),
    }
}
