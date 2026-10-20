mod common;

use alpacars::broker::enums::{AccountSubType, AccountType, AgreementType};
use alpacars::broker::requests::{AgreementRequest, CreateAccountRequest};
use chrono::{TimeZone, Utc};
use serde_json::json;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_RESPONSE: &str = r#"{
  "id": "0d18ae51-3c94-4511-b209-101e1666416b",
  "status": "SUBMITTED",
  "account_type": "ira"
}"#;

/// Body must equal the `existing-holder-ira` example on the Create an Account reference page —
/// a flow the previous request type could not express, because it required inline contact,
/// identity and disclosures.
#[tokio::test]
async fn test_create_account_existing_holder_ira() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/accounts"))
        .and(body_json(json!({
            "account_sub_type": "traditional",
            "account_type": "ira",
            "agreements": [{
                "agreement": "customer_agreement",
                "ip_address": "185.13.21.99",
                "signed_at": "2020-09-11T18:13:44Z"
            }],
            "enabled_assets": ["us_equity"],
            "primary_account_holder_id": "8b5b7e4c-9f2d-4f4a-9a7b-1b2c3d4e5f60"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(ACCOUNT_RESPONSE))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateAccountRequest {
        account_type: Some(AccountType::Ira),
        account_sub_type: Some(AccountSubType::Traditional),
        primary_account_holder_id: Some("8b5b7e4c-9f2d-4f4a-9a7b-1b2c3d4e5f60".parse().unwrap()),
        agreements: Some(vec![AgreementRequest {
            agreement: AgreementType::Customer,
            signed_at: Utc.with_ymd_and_hms(2020, 9, 11, 18, 13, 44).unwrap(),
            ip_address: "185.13.21.99".to_string(),
            revision: None,
        }]),
        enabled_assets: Some(vec!["us_equity".to_string()]),
        ..Default::default()
    };
    let account = client.create_account(&req).await.unwrap();

    assert_eq!(account.account_type, Some(AccountType::Ira));
}

/// Body must equal the `donor-advised-account` example: nothing but the type and entity.
#[tokio::test]
async fn test_create_account_donor_advised() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/accounts"))
        .and(body_json(json!({
            "account_type": "donor_advised",
            "entity_id": "14fbfc51-3704-4c74-8984-8326c4117aee"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(ACCOUNT_RESPONSE))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateAccountRequest {
        account_type: Some(AccountType::DonorAdvised),
        entity_id: Some("14fbfc51-3704-4c74-8984-8326c4117aee".to_string()),
        ..Default::default()
    };
    client.create_account(&req).await.unwrap();
}
