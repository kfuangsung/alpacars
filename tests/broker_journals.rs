mod common;

use alpacars::broker::enums::{JournalEntryType, JournalStatus};
use alpacars::broker::requests::{
    BatchJournalEntryRequest, CreateBatchJournalRequest, CreateJournalRequest,
    CreateReverseBatchJournalRequest, GetJournalsRequest, ReverseBatchJournalEntry,
};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, body_partial_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const FROM_ACCOUNT: &str = "8f8c8cee-2591-4f83-be12-82c659b5e748";
const TO_ACCOUNT: &str = "399f85f1-cbbd-4eaa-a934-70027fb5c1de";

// Taken from the JNLC / JNLS schema examples on docs.alpaca.markets/us/reference/getalljournals,
// not from the crate's own types.
const JNLC_JSON: &str = r#"{
  "currency": "USD",
  "description": "",
  "entry_type": "JNLC",
  "from_account": "8f8c8cee-2591-4f83-be12-82c659b5e748",
  "id": "56f106e5-25a4-4eee-96fa-25bb05dc86bc",
  "net_amount": "1000",
  "price": null,
  "qty": null,
  "settle_date": null,
  "status": "pending",
  "symbol": null,
  "system_date": null,
  "to_account": "399f85f1-cbbd-4eaa-a934-70027fb5c1de"
}"#;

const JNLS_JSON: &str = r#"{
  "currency": "USD",
  "description": "this is a test journal",
  "entry_type": "JNLS",
  "from_account": "8f4f3d11-4483-4199-840f-6c5fe0b7ca24",
  "id": "f45f67e8-d1fc-4136-aa4f-cf4460aecdfc",
  "net_amount": null,
  "price": "128.23",
  "qty": "0.5",
  "settle_date": "2020-12-24",
  "status": "executed",
  "symbol": "AAPL",
  "system_date": "2020-12-24",
  "to_account": "a7a7e666-6f2a-433c-8c33-17b66b8941fa"
}"#;

fn uuid(s: &str) -> Uuid {
    s.parse().unwrap()
}

#[tokio::test]
async fn test_create_journal_sends_jnlc_entry_type() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/journals"))
        .and(body_partial_json(json!({"entry_type": "JNLC"})))
        .respond_with(ResponseTemplate::new(200).set_body_string(JNLC_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateJournalRequest {
        from_account: uuid(FROM_ACCOUNT),
        to_account: uuid(TO_ACCOUNT),
        entry_type: JournalEntryType::Cash,
        amount: Some("1000".to_string()),
        symbol: None,
        qty: None,
        description: None,
        transmitter_name: None,
        transmitter_account_number: None,
        transmitter_address: None,
        transmitter_financial_institution: None,
        transmitter_timestamp: None,
    };
    let journal = client.create_journal(&req).await.unwrap();

    assert_eq!(journal.entry_type, Some(JournalEntryType::Cash));
    assert_eq!(journal.status, Some(JournalStatus::Pending));
    assert_eq!(journal.net_amount.as_deref(), Some("1000"));
}

#[tokio::test]
async fn test_get_journals_parses_jnlc_and_jnls() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/journals"))
        .and(query_param("entry_type", "JNLS"))
        .and(query_param("status", "executed"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!("[{JNLC_JSON},{JNLS_JSON}]")),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetJournalsRequest {
        entry_type: Some(JournalEntryType::Security),
        status: Some(JournalStatus::Executed),
        ..Default::default()
    };
    let journals = client.get_journals(Some(&filter)).await.unwrap();

    assert_eq!(journals.len(), 2);
    assert_eq!(journals[0].entry_type, Some(JournalEntryType::Cash));
    assert_eq!(journals[1].entry_type, Some(JournalEntryType::Security));
    assert_eq!(journals[1].status, Some(JournalStatus::Executed));
    assert_eq!(journals[1].symbol.as_deref(), Some("AAPL"));
}

#[tokio::test]
async fn test_journal_status_wire_values() {
    // Every JournalStatus variant must round-trip through the lowercase value Alpaca uses.
    let cases = [
        (JournalStatus::Pending, "pending"),
        (JournalStatus::ActivityCreated, "activity_created"),
        (JournalStatus::Executed, "executed"),
        (JournalStatus::Rejected, "rejected"),
        (JournalStatus::Canceled, "canceled"),
        (JournalStatus::Refused, "refused"),
        (JournalStatus::Corrected, "correct"),
    ];
    for (status, wire) in cases {
        assert_eq!(serde_json::to_value(&status).unwrap(), json!(wire));
        let parsed: JournalStatus = serde_json::from_value(json!(wire)).unwrap();
        assert_eq!(parsed, status);
    }
}

#[tokio::test]
async fn test_create_batch_journal_path() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/journals/batch"))
        // Shape per the BatchJournalRequest schema (createbatchjournal reference).
        .and(body_json(json!({
            "entry_type": "JNLC",
            "from_account": FROM_ACCOUNT,
            "correspondent": "ABCD",
            "entries": [{"to_account": TO_ACCOUNT, "amount": "1000", "currency": "USD"}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{JNLC_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateBatchJournalRequest {
        from_account: uuid(FROM_ACCOUNT),
        entry_type: JournalEntryType::Cash,
        entries: vec![BatchJournalEntryRequest {
            to_account: uuid(TO_ACCOUNT),
            amount: Some("1000".to_string()),
            currency: Some("USD".to_string()),
            symbol: None,
            qty: None,
            description: None,
            transmitter_name: None,
            transmitter_account_number: None,
            transmitter_address: None,
            transmitter_financial_institution: None,
            transmitter_timestamp: None,
        }],
        correspondent: Some("ABCD".to_string()),
    };
    let journals = client.create_batch_journal(&req).await.unwrap();

    assert_eq!(journals.len(), 1);
    assert_eq!(journals[0].to_account, Some(uuid(TO_ACCOUNT)));
}

#[tokio::test]
async fn test_create_reverse_batch_journal_path() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/journals/reverse_batch"))
        // Many-to-one: one top-level to_account, each entry carries its from_account
        // (post-v1-journals-reverse_batch reference).
        .and(body_json(json!({
            "entry_type": "JNLC",
            "to_account": TO_ACCOUNT,
            "entries": [{"from_account": FROM_ACCOUNT, "amount": "1000"}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{JNLC_JSON}]")))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateReverseBatchJournalRequest {
        entry_type: JournalEntryType::Cash,
        to_account: uuid(TO_ACCOUNT),
        entries: vec![ReverseBatchJournalEntry {
            from_account: uuid(FROM_ACCOUNT),
            amount: "1000".to_string(),
            description: None,
        }],
    };
    let journals = client.create_reverse_batch_journal(&req).await.unwrap();

    assert_eq!(journals.len(), 1);
}

#[tokio::test]
async fn test_get_journal_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/journals/56f106e5-25a4-4eee-96fa-25bb05dc86bc"))
        .respond_with(ResponseTemplate::new(200).set_body_string(JNLC_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let id: Uuid = "56f106e5-25a4-4eee-96fa-25bb05dc86bc".parse().unwrap();
    let journal = client.get_journal_by_id(&id).await.unwrap();
    assert_eq!(journal.id, id);
    assert_eq!(journal.entry_type, Some(JournalEntryType::Cash));
    assert_eq!(journal.status, Some(JournalStatus::Pending));
}

/// deletejournalbyid answers 204 No Content.
#[tokio::test]
async fn test_cancel_journal_by_id() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v1/journals/56f106e5-25a4-4eee-96fa-25bb05dc86bc"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .cancel_journal_by_id(&"56f106e5-25a4-4eee-96fa-25bb05dc86bc".parse().unwrap())
        .await
        .unwrap();
}
