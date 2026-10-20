mod common;

use alpacars::broker::enums::{AccountSubType, AccountType, AgreementType};
use alpacars::broker::requests::{
    AgreementRequest, CreateAccountRequest, ListAccountsRequest, UpdateAccountRequest,
};
use chrono::{TimeZone, Utc};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
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

// The AccountExtended schema example (getaccount reference).
const ACCOUNT_EXTENDED_JSON: &str = r#"{
  "account_number": "601842165",
  "account_type": "trading",
  "agreements": [
    {
      "agreement": "margin_agreement",
      "ip_address": "127.0.0.1",
      "revision": "16.2021.05",
      "signed_at": "2022-01-21T21:25:26.579487214Z"
    },
    {
      "agreement": "customer_agreement",
      "ip_address": "127.0.0.1",
      "revision": "22.2024.08",
      "signed_at": "2022-01-21T21:25:26.579487214Z"
    }
  ],
  "beneficiaries": [
    {
      "date_of_birth": "1970-01-01",
      "family_name": "Smith",
      "given_name": "John",
      "middle_name": "P",
      "relationship": "spouse",
      "share_pct": "100",
      "tax_id": "xxx-xx-xxxx",
      "tax_id_type": "USA_SSN",
      "type": "primary"
    }
  ],
  "contact": {
    "city": "San Mateo",
    "country": "USA",
    "email_address": "strange_elbakyan_97324509@example.com",
    "phone_number": "614-555-0697",
    "postal_code": "94401",
    "state": "CA",
    "street_address": [
      "20 N San Mateo Dr"
    ]
  },
  "created_at": "2022-01-21T21:25:26.583576Z",
  "crypto_status": "INACTIVE",
  "currency": "USD",
  "disclosures": {
    "immediate_family_exposed": false,
    "is_affiliated_exchange_or_finra": false,
    "is_control_person": false,
    "is_discretionary": false,
    "is_politically_exposed": false
  },
  "documents": [
    {
      "content": "https://example.com/documents/identity-verification.jpg",
      "created_at": "2022-01-21T21:25:28.184231Z",
      "document_sub_type": "passport",
      "document_type": "identity_verification",
      "id": "d5af1585-6c60-494d-9ea5-c5df62704229"
    }
  ],
  "id": "3dcb795c-3ccc-402a-abb9-07e26a1b1326",
  "identity": {
    "country_of_birth": "USA",
    "country_of_citizenship": "USA",
    "country_of_tax_residence": "USA",
    "date_of_birth": "1970-01-01",
    "family_name": "Elbakyan",
    "funding_source": [
      "employment_income"
    ],
    "given_name": "Strange",
    "tax_id_type": "USA_SSN"
  },
  "last_equity": "40645.13",
  "primary_account_holder_id": null,
  "status": "ACTIVE",
  "trusted_contact": {
    "email_address": "strange_elbakyan_97324509@example.com",
    "family_name": "Doe",
    "given_name": "Jane"
  }
}"#;

// The TradeAccount schema example (gettradingaccount reference).
const TRADE_ACCOUNT_JSON: &str = r#"{
  "account_blocked": false,
  "account_number": "927584925",
  "accrued_fees": "0",
  "admin_configurations": {},
  "balance_asof": "2021-04-01",
  "buying_power": "103556.8572572922",
  "cash": "24861.91",
  "cash_transferable": "24861.91",
  "cash_withdrawable": "17861.91",
  "clearing_broker": "VELOX",
  "created_at": "2021-03-01T13:28:49.270232Z",
  "crypto_status": "ACTIVE",
  "currency": "USD",
  "effective_buying_power": "103556.8572572922",
  "equity": "28059.3882330664",
  "id": "c8f1ef5d-edc0-4f23-9ee4-378f19cb92a4",
  "initial_margin": "1598.7391165332",
  "intraday_adjustments": "0",
  "last_buying_power": "104433.9158860662",
  "last_cash": "23861.91",
  "last_equity": "26977.323677655",
  "last_initial_margin": "1557.7068388275",
  "last_long_market_value": "3115.413677655",
  "last_maintenance_margin": "934.6241032965",
  "last_options_buying_power": "25419.62",
  "last_regt_buying_power": "50839.233677655",
  "last_short_market_value": "0",
  "long_market_value": "3197.4782330664",
  "maintenance_margin": "959.24346991992",
  "multiplier": "2",
  "non_marginable_buying_power": "24861.91",
  "options_approved_level": 0,
  "options_buying_power": "26460.65",
  "options_trading_level": 0,
  "pending_reg_taf_fees": "0",
  "pending_transfer_out": "0",
  "portfolio_value": "28059.3882330664",
  "position_market_value": "3197.4782330664",
  "previous_close": "2021-04-01T19:00:00-04:00",
  "regt_buying_power": "52921.2982330664",
  "short_market_value": "0",
  "shorting_enabled": true,
  "sma": "26758.0590204615",
  "status": "ACTIVE",
  "trade_suspended_by_user": false,
  "trading_blocked": false,
  "transfers_blocked": false
}"#;

const EXAMPLE_ACCOUNT_ID: &str = "3dcb795c-3ccc-402a-abb9-07e26a1b1326";

#[tokio::test]
async fn test_list_accounts() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/accounts"))
        .and(query_param("query", "elbakyan"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!("[{ACCOUNT_EXTENDED_JSON}]")),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = ListAccountsRequest {
        query: Some("elbakyan".to_string()),
        ..Default::default()
    };
    let accounts = client.list_accounts(Some(&filter)).await.unwrap();
    assert_eq!(accounts[0].account_number.as_deref(), Some("601842165"));
}

#[tokio::test]
async fn test_get_account_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/accounts/{EXAMPLE_ACCOUNT_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_string(ACCOUNT_EXTENDED_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let account = client
        .get_account_by_id(&EXAMPLE_ACCOUNT_ID.parse().unwrap())
        .await
        .unwrap();
    assert_eq!(account.account_type, Some(AccountType::Trading));
    assert!(account.contact.is_some());
}

#[tokio::test]
async fn test_update_account() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path(format!("/v1/accounts/{EXAMPLE_ACCOUNT_ID}")))
        .and(body_json(
            json!({"enabled_assets": ["us_equity", "crypto"]}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(ACCOUNT_EXTENDED_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = UpdateAccountRequest {
        enabled_assets: Some(vec!["us_equity".to_string(), "crypto".to_string()]),
        ..Default::default()
    };
    client
        .update_account(&EXAMPLE_ACCOUNT_ID.parse().unwrap(), &req)
        .await
        .unwrap();
}

/// The close action is `POST /v1/accounts/{id}/actions/close` (204), not `POST .../{id}:close`.
#[tokio::test]
async fn test_close_account() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!(
            "/v1/accounts/{EXAMPLE_ACCOUNT_ID}/actions/close"
        )))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .close_account(&EXAMPLE_ACCOUNT_ID.parse().unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_trading_account() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/v1/trading/accounts/c8f1ef5d-edc0-4f23-9ee4-378f19cb92a4/account",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_string(TRADE_ACCOUNT_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let id = "c8f1ef5d-edc0-4f23-9ee4-378f19cb92a4".parse().unwrap();
    let account = client.get_trading_account(&id).await.unwrap();
    assert_eq!(account.account_number.as_deref(), Some("927584925"));
    assert_eq!(account.buying_power.as_deref(), Some("103556.8572572922"));
}
