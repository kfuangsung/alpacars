//! ACATS (Beta) endpoints under /v1beta1/acats. Fixtures are the schema / response examples
//! on the docs.alpaca.markets/us/reference ACATS pages.

mod common;

use alpacars::broker::enums::{
    AcatsAccountType, AcatsAssetClass, AcatsAssetStatus, AcatsContraBrokerType, AcatsDtccStatus,
    AcatsDtccTransferType, AcatsPosition, AcatsRejectionReason, AcatsSortBy, AcatsSortDirection,
    AcatsTransferDirection, AcatsTransferStatus,
};
use alpacars::broker::requests::{
    AcatsCashBalance, AcatsContraAccountDetails, AcatsEquity, AcatsInitiateTransferRequest,
    AcatsSecurities, AcatsSecurityId, AcatsTransferDetails, ListAcatsTransfersForAccountRequest,
    ListAcatsTransfersRequest,
};
use chrono::{TimeZone, Utc};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "9ac16d7d-1f6e-445b-8e51-807e77ba2902";
const ACATS_ID: &str = "2c827687-ddd3-4ad9-bee8-0a3cdad35301";

// AcatsTransfer schema example (listacatstransfers).
const TRANSFER_JSON: &str = r#"{
  "acats_id": "2c827687-ddd3-4ad9-bee8-0a3cdad35301",
  "account_id": "9ac16d7d-1f6e-445b-8e51-807e77ba2902",
  "account_name": "John Doe",
  "account_number": "12345678",
  "account_type": "SINGLE",
  "contra_account_number": "A00000123",
  "contra_broker_name": "Alpaca Clearing",
  "contra_broker_number": "00001234",
  "contra_broker_type": "BANK",
  "created_at": "2026-04-21T12:41:03.505862Z",
  "direction": "INCOMING",
  "ip_address": "203.0.113.42",
  "rejection_details": {"rejection_message": "SS# Tax ID Mismatch", "rejection_reason": "SS_TAX_ID_MISMATCH"},
  "status": {"dtcc_status": "REJECT", "transfer_status": "REJECTED"},
  "total_transfer_value": "24223.62",
  "transfer_identifier": "03251237654321",
  "transfer_type": "FUL"
}"#;

fn uuid(s: &str) -> Uuid {
    s.parse().unwrap()
}

fn contra_account() -> AcatsContraAccountDetails {
    AcatsContraAccountDetails {
        account_number: "A00000123".to_string(),
        broker_number: "00001234".to_string(),
        customer_name: "John Doe".to_string(),
        customer_tax_id: "123-45-6789".to_string(),
        broker_name: None,
    }
}

/// `transfer_details` must equal the AcatsInitiateTransferDetails schema example.
#[tokio::test]
async fn test_initiate_partial_transfer() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/v1beta1/acats/{ACCOUNT_ID}")))
        .and(body_json(json!({
            "contra_account_details": {
                "account_number": "A00000123",
                "broker_number": "00001234",
                "customer_name": "John Doe",
                "customer_tax_id": "123-45-6789"
            },
            "transfer_details": {
                "cash": [{"amount": "10000", "currency": "USD"}],
                "securities": {"equities": [
                    {"cusip": "037833100", "position": "LONG", "qty": "2"},
                    {"cusip": "78462F103", "position": "SHORT"}
                ]},
                "type": "PARTIAL"
            },
            "ip_address": "203.0.113.42",
            "signed_at": "2026-04-21T12:00:00Z"
        })))
        .respond_with(
            ResponseTemplate::new(202).set_body_string(format!(r#"{{"acats_id": "{ACATS_ID}"}}"#)),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = AcatsInitiateTransferRequest {
        contra_account_details: contra_account(),
        transfer_details: AcatsTransferDetails::Partial {
            cash: Some(vec![AcatsCashBalance {
                currency: "USD".to_string(),
                amount: Some("10000".to_string()),
                transfer_all: None,
            }]),
            securities: Some(AcatsSecurities {
                equities: vec![
                    AcatsEquity {
                        security: AcatsSecurityId::Cusip("037833100".to_string()),
                        position: AcatsPosition::Long,
                        qty: Some("2".to_string()),
                        transfer_all: None,
                    },
                    AcatsEquity {
                        security: AcatsSecurityId::Cusip("78462F103".to_string()),
                        position: AcatsPosition::Short,
                        qty: None,
                        transfer_all: None,
                    },
                ],
            }),
        },
        ip_address: "203.0.113.42".to_string(),
        signed_at: Utc.with_ymd_and_hms(2026, 4, 21, 12, 0, 0).unwrap(),
        validate_account_number: None,
    };
    let resp = client
        .initiate_acats_transfer(&uuid(ACCOUNT_ID), &req)
        .await
        .unwrap();

    assert_eq!(resp.acats_id, uuid(ACATS_ID));
}

/// A FULL transfer carries no cash or securities keys at all (the API rejects them with 400),
/// and an equity identified by symbol sends `symbol`, not `cusip`.
#[test]
fn test_transfer_details_shapes() {
    assert_eq!(
        serde_json::to_value(AcatsTransferDetails::Full).unwrap(),
        json!({"type": "FULL"})
    );
    let by_symbol = AcatsEquity {
        security: AcatsSecurityId::Symbol("AAPL".to_string()),
        position: AcatsPosition::Long,
        qty: Some("2".to_string()),
        transfer_all: None,
    };
    // The AcatsEquity schema example.
    assert_eq!(
        serde_json::to_value(by_symbol).unwrap(),
        json!({"position": "LONG", "qty": "2", "symbol": "AAPL"})
    );
}

#[tokio::test]
async fn test_list_acats_transfers_firm_wide() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/acats"))
        .and(query_param("transfer_status", "REJECTED"))
        .and(query_param("transfer_direction", "INCOMING"))
        .and(query_param("sort_by", "created_at"))
        .and(query_param("sort_direction", "DESC"))
        .and(query_param("page_token", "eyJjcmVhdGVkX2F0Ijoi"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{"transfers": [{TRANSFER_JSON}], "page_token_next": "next-page", "pending_transfers": 3}}"#
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = ListAcatsTransfersRequest {
        transfer_status: Some(AcatsTransferStatus::Rejected),
        transfer_direction: Some(AcatsTransferDirection::Incoming),
        sort_by: Some(AcatsSortBy::CreatedAt),
        sort_direction: Some(AcatsSortDirection::Desc),
        page_token: Some("eyJjcmVhdGVkX2F0Ijoi".to_string()),
        ..Default::default()
    };
    let page = client.list_acats_transfers(Some(&filter)).await.unwrap();

    assert_eq!(page.page_token_next.as_deref(), Some("next-page"));
    assert_eq!(page.pending_transfers, Some(3));
    let t = &page.transfers[0];
    assert_eq!(t.acats_id, uuid(ACATS_ID));
    assert_eq!(t.account_type, Some(AcatsAccountType::Single));
    assert_eq!(t.contra_broker_type, Some(AcatsContraBrokerType::Bank));
    assert_eq!(t.transfer_type, AcatsDtccTransferType::Ful);
    assert_eq!(t.direction, AcatsTransferDirection::Incoming);
    assert_eq!(t.status.dtcc_status, Some(AcatsDtccStatus::Reject));
    assert_eq!(
        t.status.transfer_status,
        Some(AcatsTransferStatus::Rejected)
    );
    let rejection = t.rejection_details.as_ref().unwrap();
    assert_eq!(
        rejection.rejection_reason,
        Some(AcatsRejectionReason::SsTaxIdMismatch)
    );
}

#[tokio::test]
async fn test_list_acats_transfers_for_account() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v1beta1/acats/{ACCOUNT_ID}")))
        .and(query_param("limit", "25"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(format!(r#"{{"transfers": [{TRANSFER_JSON}]}}"#)),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = ListAcatsTransfersForAccountRequest {
        limit: Some(25),
        page_token: None,
    };
    let page = client
        .list_acats_transfers_for_account(&uuid(ACCOUNT_ID), Some(&filter))
        .await
        .unwrap();

    assert_eq!(page.transfers.len(), 1);
    assert_eq!(page.page_token_next, None);
}

#[tokio::test]
async fn test_get_acats_transfer() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v1beta1/acats/{ACCOUNT_ID}/{ACATS_ID}")))
        .respond_with(ResponseTemplate::new(200).set_body_string(TRANSFER_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let t = client
        .get_acats_transfer(&uuid(ACCOUNT_ID), &uuid(ACATS_ID))
        .await
        .unwrap();

    assert_eq!(t.total_transfer_value.as_deref(), Some("24223.62"));
}

#[tokio::test]
async fn test_list_acats_transfer_assets() {
    let server = MockServer::start().await;

    // AcatsAsset schema example; the response is a bare array.
    Mock::given(method("GET"))
        .and(path(format!(
            "/v1beta1/acats/{ACCOUNT_ID}/{ACATS_ID}/assets"
        )))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[{
              "amount": "24223.62",
              "asset_class": "EQUITY",
              "asset_description": "APPLE INC",
              "currency_code": "USD",
              "cusip": "037833100",
              "position": "LONG",
              "qty": "123",
              "status": "SETTLED",
              "symbol": "AAPL"
            }]"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let assets = client
        .list_acats_transfer_assets(&uuid(ACCOUNT_ID), &uuid(ACATS_ID))
        .await
        .unwrap();

    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].asset_class, AcatsAssetClass::Equity);
    assert_eq!(assets[0].status, AcatsAssetStatus::Settled);
    assert_eq!(assets[0].position, AcatsPosition::Long);
    assert_eq!(assets[0].cusip.as_deref(), Some("037833100"));
}

#[tokio::test]
async fn test_list_acats_contra_brokers() {
    let server = MockServer::start().await;

    // The 200 response example on listacatscontrabrokers.
    Mock::given(method("GET"))
        .and(path("/v1beta1/acats/contrabrokers"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"brokers": [
              {"name": "Example Brokerage Inc.", "number": "0001", "type": "BROKER"},
              {"name": "Example Bank, N.A.", "number": "0002", "type": "BANK"}
            ]}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let brokers = client.list_acats_contra_brokers().await.unwrap();

    assert_eq!(brokers.len(), 2);
    assert_eq!(brokers[0].number, "0001");
    assert_eq!(brokers[0].broker_type, AcatsContraBrokerType::Broker);
    assert_eq!(brokers[1].broker_type, AcatsContraBrokerType::Bank);
}
