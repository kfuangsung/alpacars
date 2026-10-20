//! Broker funding: ACH relationships, recipient banks and transfers, checked against the
//! getaccountachrelationships / createachrelationshipforaccount / deleteachrelationshipfromaccount,
//! getrecipientbanks / createrecipientbank / deleterecipientbank and gettransfersforaccount /
//! createtransferforaccount / deletetransfer references. Fixtures follow their schemas.

mod common;

use alpacars::broker::enums::{
    ACHRelationshipStatus, BankAccountType, BankCodeType, BankStatus, BankStatusFilter,
    FeePaymentMethod, TransferDirection, TransferStatus, TransferType,
};
use alpacars::broker::models::BankExtraFields;
use alpacars::broker::requests::{
    CreateACHRelationshipRequest, CreateBankRequest, CreateTransferRequest,
    GetACHRelationshipsRequest, GetBanksRequest, GetTransfersRequest, TransferIra,
    TransferIraTaxWithholding,
};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";
const ACH_ID: &str = "61e69015-8549-4bfd-b9c3-01e75843f47d";
const BANK_ID: &str = "0c6b2c3a-47c4-4b8e-9f1c-0d3e5a7b9c11";
const TRANSFER_ID: &str = "be3c368a-4c7c-4384-808e-f02c9f5a8afe";

const ACH_JSON: &str = r#"{
  "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "status": "QUEUED",
  "account_owner_name": "John Doe",
  "bank_account_type": "CHECKING",
  "bank_account_number": "32131231abc",
  "bank_routing_number": "121000358",
  "nickname": "Bank of America Checking"
}"#;

const BANK_JSON: &str = r#"{
  "id": "0c6b2c3a-47c4-4b8e-9f1c-0d3e5a7b9c11",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "name": "Bank of America",
  "bank_code": "026009593",
  "bank_code_type": "ABA",
  "account_number": "123456789",
  "status": "QUEUED"
}"#;

const WIRE_BANK_JSON: &str = r#"{
  "id": "0c6b2c3a-47c4-4b8e-9f1c-0d3e5a7b9c11",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "name": "Deutsche Bank",
  "bank_code": "DEUTDEFF",
  "bank_code_type": "BIC",
  "account_number": "DE89370400440532013000",
  "country": "DE",
  "city": "Frankfurt",
  "extra_fields": {"intermediary_bank1_bic": "CHASUS33"},
  "status": "SENT_TO_CLEARING"
}"#;

const WIRE_TRANSFER_JSON: &str = r#"{
  "id": "be3c368a-4c7c-4384-808e-f02c9f5a8afe",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "bank_id": "0c6b2c3a-47c4-4b8e-9f1c-0d3e5a7b9c11",
  "type": "wire",
  "status": "QUEUED",
  "amount": "1000",
  "requested_amount": "1000",
  "fee": "25",
  "fee_payment_method": "invoice",
  "instant_amount": "0",
  "direction": "OUTGOING",
  "additional_information": "Ref 42",
  "ira": {
    "distribution_reason": "normal",
    "tax_year": "2024",
    "fed_withholding_pct": "10.25",
    "fed_withholding_amount": "102.5",
    "state_withholding_pct": "9.75",
    "state_withholding_amount": "97.5"
  },
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z"
}"#;

const TRANSFER_JSON: &str = r#"{
  "id": "be3c368a-4c7c-4384-808e-f02c9f5a8afe",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "relationship_id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
  "type": "ach",
  "status": "QUEUED",
  "amount": "500",
  "direction": "INCOMING",
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "reason": null,
  "requested_amount": null,
  "fee": null,
  "fee_payment_method": null,
  "additional_information": null
}"#;

fn account() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

fn base() -> String {
    format!("/v1/accounts/{ACCOUNT_ID}")
}

#[tokio::test]
async fn test_create_ach_relationship() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/ach_relationships", base())))
        .and(body_json(json!({
            "account_owner_name": "John Doe",
            "bank_account_type": "CHECKING",
            "bank_account_number": "32131231abc",
            "bank_routing_number": "121000358",
            "processor_token": "processor-sandbox-123",
            "instant": true,
            "nickname": "Bank of America Checking"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(ACH_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateACHRelationshipRequest {
        account_owner_name: "John Doe".to_string(),
        bank_account_type: BankAccountType::Checking,
        bank_account_number: "32131231abc".to_string(),
        bank_routing_number: "121000358".to_string(),
        processor_token: Some("processor-sandbox-123".to_string()),
        instant: Some(true),
        nickname: Some("Bank of America Checking".to_string()),
    };
    let ach = client
        .create_ach_relationship_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(ach.status, ACHRelationshipStatus::Queued);
    assert_eq!(ach.nickname.as_deref(), Some("Bank of America Checking"));
}

#[tokio::test]
async fn test_get_and_delete_ach_relationships() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/ach_relationships", base())))
        .and(query_param("statuses", "QUEUED,APPROVED"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{ACH_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/ach_relationships/{ACH_ID}", base())))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetACHRelationshipsRequest {
        statuses: Some(vec![
            ACHRelationshipStatus::Queued,
            ACHRelationshipStatus::Approved,
        ]),
    };
    let list = client
        .get_ach_relationships_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(list[0].bank_account_type, Some(BankAccountType::Checking));
    client
        .delete_ach_relationship_for_account(&account(), &ACH_ID.parse().unwrap())
        .await
        .unwrap();
}

/// Recipient banks live at /recipient_banks (the crate called /banks). The bank is identified by
/// `bank_code` + `bank_code_type`; there is no `routing_number`. Unset optional fields are
/// omitted rather than sent as `null`.
#[tokio::test]
async fn test_create_bank() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/recipient_banks", base())))
        .and(body_json(json!({
            "name": "Bank of America",
            "bank_code": "026009593",
            "bank_code_type": "ABA",
            "account_number": "123456789"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(BANK_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateBankRequest {
        name: "Bank of America".to_string(),
        bank_code: "026009593".to_string(),
        bank_code_type: BankCodeType::Aba,
        account_number: "123456789".to_string(),
        extra_fields: None,
        country: None,
        state_province: None,
        postal_code: None,
        city: None,
        street_address: None,
    };
    let bank = client
        .create_bank_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(bank.name.as_deref(), Some("Bank of America"));
    assert_eq!(bank.bank_code.as_deref(), Some("026009593"));
    assert_eq!(bank.bank_code_type, Some(BankCodeType::Aba));
    assert_eq!(bank.status, Some(BankStatus::Queued));
}

#[tokio::test]
async fn test_create_international_bank_with_intermediary() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/recipient_banks", base())))
        .and(body_json(json!({
            "name": "Deutsche Bank",
            "bank_code": "DEUTDEFF",
            "bank_code_type": "BIC",
            "account_number": "DE89370400440532013000",
            "extra_fields": {"intermediary_bank1_bic": "CHASUS33"},
            "country": "DE",
            "city": "Frankfurt"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(WIRE_BANK_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateBankRequest {
        name: "Deutsche Bank".to_string(),
        bank_code: "DEUTDEFF".to_string(),
        bank_code_type: BankCodeType::Bic,
        account_number: "DE89370400440532013000".to_string(),
        extra_fields: Some(BankExtraFields {
            intermediary_bank1_bic: Some("CHASUS33".to_string()),
            ..Default::default()
        }),
        country: Some("DE".to_string()),
        state_province: None,
        postal_code: None,
        city: Some("Frankfurt".to_string()),
        street_address: None,
    };
    let bank = client
        .create_bank_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(bank.bank_code_type, Some(BankCodeType::Bic));
    assert_eq!(bank.status, Some(BankStatus::SentToClearing));
    assert_eq!(
        bank.extra_fields.unwrap().intermediary_bank1_bic.as_deref(),
        Some("CHASUS33")
    );
}

#[tokio::test]
async fn test_get_and_delete_banks() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/recipient_banks", base())))
        .and(query_param("status", "ACTIVE"))
        .and(query_param("bank_name", "Bank of America"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{BANK_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/recipient_banks/{BANK_ID}", base())))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetBanksRequest {
        status: Some(BankStatusFilter::Active),
        bank_name: Some("Bank of America".to_string()),
    };
    let banks = client
        .get_banks_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(banks.len(), 1);
    client
        .delete_bank_for_account(&account(), &BANK_ID.parse().unwrap())
        .await
        .unwrap();
}

/// `transfer_type` is lowercase on the wire (`ach`), and `bank_id` is omitted, not `null`.
#[tokio::test]
async fn test_create_ach_transfer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/transfers", base())))
        .and(body_json(json!({
            "transfer_type": "ach",
            "relationship_id": ACH_ID,
            "amount": "500",
            "direction": "INCOMING"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(TRANSFER_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateTransferRequest {
        transfer_type: TransferType::Ach,
        relationship_id: Some(ACH_ID.parse().unwrap()),
        bank_id: None,
        amount: "500".to_string(),
        direction: TransferDirection::Incoming,
        additional_information: None,
        fee_payment_method: None,
        ira: None,
    };
    let transfer = client
        .create_transfer_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(transfer.r#type, Some(TransferType::Ach));
    assert_eq!(transfer.status, Some(TransferStatus::Queued));
}

#[tokio::test]
async fn test_create_ira_wire_transfer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/transfers", base())))
        .and(body_json(json!({
            "transfer_type": "wire",
            "bank_id": BANK_ID,
            "amount": "1000",
            "direction": "OUTGOING",
            "additional_information": "Ref 42",
            "fee_payment_method": "invoice",
            "ira": {
                "distribution_reason": "normal",
                "tax_year": "2024",
                "tax_withholding": {"fed_pct": "10.25", "state_pct": "8.25"}
            }
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(WIRE_TRANSFER_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateTransferRequest {
        transfer_type: TransferType::Wire,
        relationship_id: None,
        bank_id: Some(BANK_ID.parse().unwrap()),
        amount: "1000".to_string(),
        direction: TransferDirection::Outgoing,
        additional_information: Some("Ref 42".to_string()),
        fee_payment_method: Some(FeePaymentMethod::Invoice),
        ira: Some(TransferIra {
            distribution_reason: Some("normal".to_string()),
            tax_year: Some("2024".to_string()),
            tax_withholding: Some(TransferIraTaxWithholding {
                fed_pct: Some("10.25".to_string()),
                state_pct: Some("8.25".to_string()),
            }),
        }),
    };
    let transfer = client
        .create_transfer_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(transfer.r#type, Some(TransferType::Wire));
    assert_eq!(transfer.fee.as_deref(), Some("25"));
    assert_eq!(transfer.requested_amount.as_deref(), Some("1000"));
    assert_eq!(transfer.instant_amount.as_deref(), Some("0"));
    assert_eq!(transfer.fee_payment_method, Some(FeePaymentMethod::Invoice));
    let ira = transfer.ira.unwrap();
    assert_eq!(ira.fed_withholding_amount.as_deref(), Some("102.5"));
    assert_eq!(ira.state_withholding_pct.as_deref(), Some("9.75"));
}

#[tokio::test]
async fn test_get_and_cancel_transfers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/transfers", base())))
        .and(query_param("direction", "INCOMING"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{TRANSFER_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("{}/transfers/{TRANSFER_ID}", base())))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetTransfersRequest {
        direction: Some(TransferDirection::Incoming),
        limit: None,
        offset: None,
    };
    let transfers = client
        .get_transfers_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(transfers[0].direction, Some(TransferDirection::Incoming));
    client
        .cancel_transfer_for_account(&account(), &TRANSFER_ID.parse().unwrap())
        .await
        .unwrap();
}

#[test]
fn test_transfer_type_wire_values() {
    // gettransfersforaccount / createtransferforaccount: TransferType enum ['ach', 'wire'].
    for (variant, wire) in [(TransferType::Ach, "ach"), (TransferType::Wire, "wire")] {
        assert_eq!(serde_json::to_value(&variant).unwrap(), json!(wire));
        let parsed: TransferType = serde_json::from_value(json!(wire)).unwrap();
        assert_eq!(parsed, variant);
    }
}
