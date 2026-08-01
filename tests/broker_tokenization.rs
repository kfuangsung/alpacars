mod common;

use alpacars::broker::models::{GetTokenizationRequestsRequest, TokenizationMintRequest};
use alpacars::trading::enums::{
    TokenizationIssuer, TokenizationNetwork, TokenizationRequestStatus, TokenizationRequestType,
};
use uuid::Uuid;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";

const TOKENIZATION_REQUEST_JSON: &str = r#"{
  "tokenization_request_id": "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
  "status": "completed",
  "type": "mint",
  "underlying_symbol": "TSLA",
  "token_symbol": "TSLAx",
  "qty": "3",
  "created_at": "2026-07-16T10:00:00Z",
  "updated_at": null,
  "issuer": "st0x",
  "network": "base",
  "client_request_id": "ap-request-2026-07-16-002",
  "client_account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "client_external_account_id": "issuer-acct-99",
  "issuer_request_id": "iss-req-31337",
  "fees": null,
  "tx_hash": null,
  "account": null,
  "issuer_account": null
}"#;

fn account_id() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

#[tokio::test]
async fn test_mint_tokenized_asset_for_account() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!(
            "/v1/accounts/{}/tokenization/mint",
            ACCOUNT_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "tokenization_request_id": "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
              "status": "pending",
              "underlying_symbol": "TSLA",
              "token_symbol": "TSLAx",
              "qty": "3",
              "created_at": "2026-07-16T10:00:00Z",
              "issuer": "st0x",
              "network": "base"
            }"#,
        ))
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = TokenizationMintRequest {
        underlying_symbol: "TSLA".to_string(),
        qty: "3".to_string(),
        issuer: TokenizationIssuer::St0x,
        network: TokenizationNetwork::Base,
        wallet_address: "0xdead...beef".to_string(),
        client_request_id: None,
    };
    let resp = client
        .mint_tokenized_asset_for_account(&account_id(), &req)
        .await
        .unwrap();

    assert_eq!(resp.token_symbol, "TSLAx");
    assert_eq!(resp.status, TokenizationRequestStatus::Pending);
    assert_eq!(resp.network, TokenizationNetwork::Base);
}

#[tokio::test]
async fn test_get_tokenization_requests_for_account() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/accounts/{}/tokenization/requests",
            ACCOUNT_ID
        )))
        .and(query_param("underlying_symbol", "TSLA"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!("[{}]", TOKENIZATION_REQUEST_JSON)),
        )
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetTokenizationRequestsRequest {
        underlying_symbol: Some("TSLA".to_string()),
        ..Default::default()
    };
    let requests = client
        .get_tokenization_requests_for_account(&account_id(), Some(&filter))
        .await
        .unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].token_symbol, "TSLAx");
    assert_eq!(requests[0].issuer, TokenizationIssuer::St0x);
    assert!(requests[0].updated_at.is_none());
}

#[tokio::test]
async fn test_get_tokenization_request_for_account_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/accounts/{}/tokenization/requests/6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
            ACCOUNT_ID
        )))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOKENIZATION_REQUEST_JSON))
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = client
        .get_tokenization_request_for_account(&account_id(), "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11")
        .await
        .unwrap();

    assert_eq!(req.request_type, TokenizationRequestType::Mint);
    assert_eq!(req.client_account_id, Some(account_id()));
}

#[tokio::test]
async fn test_get_tokenization_request_for_account_by_client_request_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/accounts/{}/tokenization/requests:by_client_request_id",
            ACCOUNT_ID
        )))
        .and(query_param(
            "client_request_id",
            "ap-request-2026-07-16-002",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOKENIZATION_REQUEST_JSON))
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = client
        .get_tokenization_request_for_account_by_client_request_id(
            &account_id(),
            "ap-request-2026-07-16-002",
        )
        .await
        .unwrap();

    assert_eq!(
        req.client_request_id.as_deref(),
        Some("ap-request-2026-07-16-002")
    );
}

/// Broker-only lookup; the Trading API has no issuer_request_id equivalent.
#[tokio::test]
async fn test_get_tokenization_request_for_account_by_issuer_request_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/accounts/{}/tokenization/requests:by_issuer_request_id",
            ACCOUNT_ID
        )))
        .and(query_param("issuer_request_id", "iss-req-31337"))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOKENIZATION_REQUEST_JSON))
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = client
        .get_tokenization_request_for_account_by_issuer_request_id(&account_id(), "iss-req-31337")
        .await
        .unwrap();

    assert_eq!(req.issuer_request_id.as_deref(), Some("iss-req-31337"));
}
