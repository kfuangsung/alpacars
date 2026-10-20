//! Broker funding: ACH relationships, recipient banks and transfers, checked against the
//! getaccountachrelationships / createachrelationshipforaccount / deleteachrelationshipfromaccount,
//! getrecipientbanks / createrecipientbank / deleterecipientbank and gettransfersforaccount /
//! createtransferforaccount / deletetransfer references. Fixtures follow their schemas.

mod common;

use alpacars::broker::enums::{
    ACHRelationshipStatus, BankAccountType, TransferDirection, TransferStatus, TransferType,
};
use alpacars::broker::requests::{
    CreateACHRelationshipRequest, CreateBankRequest, CreateTransferRequest, GetTransfersRequest,
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
            "bank_routing_number": "121000358"
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
        plaid_processor_token: None,
        processor_token: None,
    };
    let ach = client
        .create_ach_relationship_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(ach.status, ACHRelationshipStatus::Queued);
}

#[tokio::test]
async fn test_get_and_delete_ach_relationships() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/ach_relationships", base())))
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
    let list = client
        .get_ach_relationships_for_account(&account())
        .await
        .unwrap();
    assert_eq!(list[0].bank_account_type, Some(BankAccountType::Checking));
    client
        .delete_ach_relationship_for_account(&account(), &ACH_ID.parse().unwrap())
        .await
        .unwrap();
}

/// Recipient banks live at /recipient_banks (the crate called /banks). Unset optional fields
/// are omitted rather than sent as `null`.
#[tokio::test]
async fn test_create_bank() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("{}/recipient_banks", base())))
        .and(body_json(json!({
            "name": "Bank of America",
            "routing_number": "026009593",
            "account_number": "123456789",
            "bank_code": "026009593",
            "bank_code_type": "ABA"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(BANK_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateBankRequest {
        name: "Bank of America".to_string(),
        routing_number: "026009593".to_string(),
        account_number: "123456789".to_string(),
        bank_code: Some("026009593".to_string()),
        bank_code_type: Some("ABA".to_string()),
        country: None,
        state_province: None,
        postal_code: None,
        city: None,
        street_address: None,
    };
    let bank = client.create_bank_for_account(&account(), &req).await.unwrap();
    assert_eq!(bank.name.as_deref(), Some("Bank of America"));
}

#[tokio::test]
async fn test_get_and_delete_banks() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("{}/recipient_banks", base())))
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
    let banks = client.get_banks_for_account(&account()).await.unwrap();
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
        timing: None,
        additional_information: None,
    };
    let transfer = client
        .create_transfer_for_account(&account(), &req)
        .await
        .unwrap();
    assert_eq!(transfer.r#type, Some(TransferType::Ach));
    assert_eq!(transfer.status, Some(TransferStatus::Queued));
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
