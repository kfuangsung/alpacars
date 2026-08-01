mod common;

use alpacars::trading::enums::{
    TokenizationIssuer, TokenizationNetwork, TokenizationRequestStatus, TokenizationRequestType,
};
use alpacars::trading::requests::{GetTokenizationRequestsRequest, TokenizationMintRequest};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TOKENIZATION_REQUEST_JSON: &str = r#"{
  "tokenization_request_id": "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
  "status": "completed",
  "type": "mint",
  "underlying_symbol": "AAPL",
  "token_symbol": "AAPLx",
  "qty": "10.5",
  "created_at": "2026-07-16T10:00:00Z",
  "updated_at": "2026-07-16T10:05:00Z",
  "issuer": "xstocks",
  "network": "solana",
  "client_request_id": "ap-request-2026-07-16-001",
  "client_account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "client_external_account_id": "issuer-acct-42",
  "issuer_request_id": "iss-req-778",
  "fees": "0.25",
  "tx_hash": "5xY7...abc",
  "account": null,
  "issuer_account": null
}"#;

#[tokio::test]
async fn test_mint_tokenized_asset() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v2/tokenization/mint"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "tokenization_request_id": "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
              "status": "pending",
              "underlying_symbol": "AAPL",
              "token_symbol": "AAPLx",
              "qty": "10.5",
              "created_at": "2026-07-16T10:00:00Z",
              "issuer": "xstocks",
              "network": "solana"
            }"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = TokenizationMintRequest {
        underlying_symbol: "AAPL".to_string(),
        qty: "10.5".to_string(),
        issuer: TokenizationIssuer::Xstocks,
        network: TokenizationNetwork::Solana,
        wallet_address: "9xQe...wallet".to_string(),
        client_request_id: Some("ap-request-2026-07-16-001".to_string()),
    };
    let resp = client.mint_tokenized_asset(&req).await.unwrap();

    assert_eq!(resp.token_symbol, "AAPLx");
    assert_eq!(resp.status, TokenizationRequestStatus::Pending);
    assert_eq!(resp.issuer, TokenizationIssuer::Xstocks);
    assert_eq!(resp.network, TokenizationNetwork::Solana);
}

#[tokio::test]
async fn test_get_tokenization_requests_with_filters() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/tokenization/requests"))
        .and(query_param("type", "mint"))
        .and(query_param("status", "completed"))
        .and(query_param("network", "solana"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!("[{}]", TOKENIZATION_REQUEST_JSON)),
        )
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetTokenizationRequestsRequest {
        request_type: Some(TokenizationRequestType::Mint),
        status: Some(TokenizationRequestStatus::Completed),
        network: Some(TokenizationNetwork::Solana),
        ..Default::default()
    };
    let requests = client.get_tokenization_requests(Some(&req)).await.unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].underlying_symbol, "AAPL");
    assert_eq!(requests[0].request_type, TokenizationRequestType::Mint);
    assert_eq!(requests[0].fees.as_deref(), Some("0.25"));
}

#[tokio::test]
async fn test_get_tokenization_request_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/v2/tokenization/requests/6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOKENIZATION_REQUEST_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = client
        .get_tokenization_request("6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11")
        .await
        .unwrap();

    assert_eq!(
        req.tokenization_request_id,
        "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11"
    );
    assert_eq!(req.status, TokenizationRequestStatus::Completed);
    assert_eq!(
        req.client_external_account_id.as_deref(),
        Some("issuer-acct-42")
    );
    // Deprecated fields (sunset 2026-10-15) may be omitted entirely.
    assert!(req.account.is_none());
}

#[tokio::test]
async fn test_get_tokenization_request_by_client_request_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/tokenization/requests:by_client_request_id"))
        .and(query_param(
            "client_request_id",
            "ap-request-2026-07-16-001",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(TOKENIZATION_REQUEST_JSON))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = client
        .get_tokenization_request_by_client_request_id("ap-request-2026-07-16-001")
        .await
        .unwrap();

    assert_eq!(
        req.client_request_id.as_deref(),
        Some("ap-request-2026-07-16-001")
    );
}
