//! `POST /v1/agreements`. Request bodies are asserted against the named examples on
//! docs.alpaca.markets/us/reference/createagreement.

mod common;

use alpacars::broker::enums::{PartyAgreementType, PerAccountAgreementType, SignedAgreementType};
use alpacars::broker::requests::{
    CreateAccountAgreementRequest, CreateAgreementRequest, CreatePartyAgreementRequest,
};
use alpacars::error::AlpacaError;
use chrono::{TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn uuid(s: &str) -> Uuid {
    s.parse().unwrap()
}

#[tokio::test]
async fn test_create_party_agreement() {
    let server = MockServer::start().await;

    // perPartyNaturalPerson
    let body = json!({
        "agreement": "margin_agreement",
        "ip_address": "192.168.1.100",
        "party_id": "f47ac10b-58cc-4372-a567-0e02b2c3d479",
        "revision": "19.2024.01",
        "signed_at": "2024-01-15T10:30:00Z"
    });
    Mock::given(method("POST"))
        .and(path("/v1/agreements"))
        .and(body_json(&body))
        .respond_with(ResponseTemplate::new(201).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateAgreementRequest::Party(CreatePartyAgreementRequest {
        party_id: uuid("f47ac10b-58cc-4372-a567-0e02b2c3d479"),
        agreement: PartyAgreementType::MarginAgreement,
        signed_at: Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap(),
        ip_address: "192.168.1.100".to_string(),
        revision: Some("19.2024.01".to_string()),
        signer_party_id: None,
    });
    let signed = client.create_agreement(&req).await.unwrap();

    assert_eq!(signed.agreement, SignedAgreementType::MarginAgreement);
    assert_eq!(
        signed.party_id,
        uuid("f47ac10b-58cc-4372-a567-0e02b2c3d479")
    );
    assert_eq!(signed.account_id, None);
    assert_eq!(signed.revision.as_deref(), Some("19.2024.01"));
}

#[tokio::test]
async fn test_create_account_agreement_for_legal_entity() {
    let server = MockServer::start().await;

    // perAccountLegalEntity
    let body = json!({
        "account_id": "d7e8f9a0-b1c2-3d4e-5f67-890123456789",
        "agreement": "options_agreement",
        "ip_address": "10.0.0.50",
        "party_id": "550e8400-e29b-41d4-a716-446655440000",
        "signed_at": "2024-08-18T14:05:00Z",
        "signer_party_id": "550e8400-e29b-41d4-a716-446655440003"
    });
    Mock::given(method("POST"))
        .and(path("/v1/agreements"))
        .and(body_json(&body))
        .respond_with(ResponseTemplate::new(201).set_body_json(&body))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateAgreementRequest::Account(CreateAccountAgreementRequest {
        party_id: uuid("550e8400-e29b-41d4-a716-446655440000"),
        account_id: uuid("d7e8f9a0-b1c2-3d4e-5f67-890123456789"),
        agreement: PerAccountAgreementType::OptionsAgreement,
        signed_at: Utc.with_ymd_and_hms(2024, 8, 18, 14, 5, 0).unwrap(),
        ip_address: "10.0.0.50".to_string(),
        revision: None,
        signer_party_id: Some(uuid("550e8400-e29b-41d4-a716-446655440003")),
    });
    let signed = client.create_agreement(&req).await.unwrap();

    assert_eq!(signed.agreement, SignedAgreementType::OptionsAgreement);
    assert_eq!(
        signed.account_id,
        Some(uuid("d7e8f9a0-b1c2-3d4e-5f67-890123456789"))
    );
}

/// Errors use `{error, error_description}` rather than Alpaca's usual `{code, message}`;
/// the raw body still reaches the caller through `AlpacaError::Api`.
#[tokio::test]
async fn test_create_agreement_party_not_found() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/agreements"))
        .respond_with(ResponseTemplate::new(400).set_body_string(
            r#"{"error": "party_not_found", "error_description": "party does not exist"}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateAgreementRequest::Party(CreatePartyAgreementRequest {
        party_id: uuid("e4d3c2b1-a098-7654-3210-fedcba987654"),
        agreement: PartyAgreementType::CertificationsAndResolutionsStatement,
        signed_at: Utc.with_ymd_and_hms(2024, 8, 18, 14, 0, 0).unwrap(),
        ip_address: "10.0.0.50".to_string(),
        revision: None,
        signer_party_id: None,
    });

    match client.create_agreement(&req).await {
        Err(AlpacaError::Api {
            status_code,
            message,
            ..
        }) => {
            assert_eq!(status_code, 400);
            assert!(message.contains("party_not_found"), "{message}");
        }
        other => panic!("expected an API error, got {other:?}"),
    }
}
