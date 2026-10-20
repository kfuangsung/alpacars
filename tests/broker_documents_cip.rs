//! Broker documents and CIP, checked against the getdocsforaccount, uploaddoctoaccount,
//! downloaddocfromaccount, get-v1-accounts-account_id-cip and post-v1-accounts-account_id-cip
//! references.

mod common;

use alpacars::broker::enums::{DocumentType, TradeDocumentType, UploadDocumentMimeType};
use alpacars::broker::requests::UploadDocumentRequest;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";
const DOCUMENT_ID: &str = "0d18ae51-3c94-4511-b209-101e1666416b";

const CIP_JSON: &str = r#"{
  "id": "4f6bc222-0e13-4d9f-9a88-2b3c4d5e6f70",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "provider_name": ["onfido"],
  "created_at": "2026-10-20T10:00:00Z",
  "updated_at": "2026-10-20T10:00:00Z",
  "kyc": {"id": "kyc-1", "result": "clear", "status": "complete"}
}"#;

fn account() -> Uuid {
    ACCOUNT_ID.parse().unwrap()
}

fn document() -> Uuid {
    DOCUMENT_ID.parse().unwrap()
}

/// The AccountDocument example uses the keys `type` / `sub_type` and lowercase type values;
/// the crate read `document_type` / `document_sub_type`, so both came back `None`.
#[tokio::test]
async fn test_get_trade_documents_for_account() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/accounts/{ACCOUNT_ID}/documents")))
        .and(query_param("start", "2019-01-01"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"[{"date": "2019-09-30", "id": "0d18ae51-3c94-4511-b209-101e1666416b",
                 "sub_type": "1099-Comp", "type": "account_statement"}]"#,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = alpacars::broker::requests::GetTradeDocumentsRequest {
        start: Some(chrono::NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()),
        end: None,
    };
    let docs = client
        .get_trade_documents_for_account(&account(), Some(&filter))
        .await
        .unwrap();
    assert_eq!(docs[0].id, document());
    assert_eq!(docs[0].document_type, Some(TradeDocumentType::AccountStatement));
    assert_eq!(docs[0].document_sub_type.as_deref(), Some("1099-Comp"));
}

/// Body must equal the OwnerDocumentUploadRequest example.
#[tokio::test]
async fn test_upload_documents_to_account() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/accounts/{ACCOUNT_ID}/documents/upload")))
        .and(body_json(json!([{
            "content": "/9j/Cg==",
            "document_sub_type": "passport",
            "document_type": "identity_verification",
            "mime_type": "image/jpeg"
        }])))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let docs = [UploadDocumentRequest {
        document_type: DocumentType::IdentityVerification,
        document_sub_type: Some("passport".to_string()),
        content: "/9j/Cg==".to_string(),
        mime_type: UploadDocumentMimeType::Jpeg,
    }];
    client
        .upload_documents_to_account(&account(), &docs)
        .await
        .unwrap();
}

/// The download endpoint answers 301 to a pre-signed link on another host. The redirect is
/// followed, and the Broker API's Basic-auth `Authorization` header must not reach that host.
#[tokio::test]
async fn test_download_document_follows_redirect_without_credentials() {
    let file_host = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/presigned/statement.pdf"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&file_host)
        .await;
    Mock::given(method("GET"))
        .and(path("/presigned/statement.pdf"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(b"%PDF-1.7 test".to_vec()))
        .expect(1)
        .mount(&file_host)
        .await;

    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!(
            "/v1/accounts/{ACCOUNT_ID}/documents/{DOCUMENT_ID}/download"
        )))
        .and(header_exists("authorization"))
        .respond_with(
            ResponseTemplate::new(301)
                .insert_header("Location", format!("{}/presigned/statement.pdf", file_host.uri())),
        )
        .expect(1)
        .mount(&api)
        .await;

    let client = common::broker_client(&api.uri());
    let bytes = client
        .download_trade_document_for_account(&account(), &document())
        .await
        .unwrap();
    assert_eq!(bytes, b"%PDF-1.7 test");
}

#[tokio::test]
async fn test_get_cip_data() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/accounts/{ACCOUNT_ID}/cip")))
        .respond_with(ResponseTemplate::new(200).set_body_string(CIP_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let cip = client
        .get_cip_data_for_account_by_id(&account())
        .await
        .unwrap();
    assert_eq!(cip.provider_name, Some(vec!["onfido".to_string()]));
}

#[tokio::test]
async fn test_upload_cip_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/accounts/{ACCOUNT_ID}/cip")))
        .and(body_json(json!({"provider_name": ["onfido"]})))
        .respond_with(ResponseTemplate::new(200).set_body_string(CIP_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .upload_cip_data_for_account_by_id(&account(), &json!({"provider_name": ["onfido"]}))
        .await
        .unwrap();
}

#[test]
fn test_document_type_wire_values() {
    // OwnerDocumentType (uploaddoctoaccount) and AccountDocumentType (getdocsforaccount).
    for wire in [
        "account_approval_letter", "address_verification", "cip_result", "company_formation",
        "date_of_birth_verification", "entity_operating_document", "entity_registration",
        "hio_declaration_form", "identity_verification", "limited_trading_authorization",
        "pep_declaration_form", "tax_id_verification", "w8ben", "w9",
    ] {
        let v: DocumentType = serde_json::from_value(json!(wire)).unwrap();
        assert_ne!(v, DocumentType::Unknown, "{wire}");
        assert_eq!(serde_json::to_value(&v).unwrap(), json!(wire));
    }
    for wire in [
        "account_application", "account_statement", "crypto_account_statement",
        "options_application", "tax_statement", "trade_confirmation_json", "trade_confirmation",
    ] {
        let v: TradeDocumentType = serde_json::from_value(json!(wire)).unwrap();
        assert_ne!(v, TradeDocumentType::Unknown, "{wire}");
        assert_eq!(serde_json::to_value(&v).unwrap(), json!(wire));
    }
}

#[tokio::test]
#[allow(deprecated)]
async fn test_deprecated_get_document_by_id_still_compiles() {
    // There is no such endpoint; the method is deprecated. This only pins that it still exists
    // (removal is a 0.7.0 change) and fails cleanly rather than panicking.
    let server = MockServer::start().await;
    let client = common::broker_client(&server.uri());
    assert!(client
        .get_trade_document_for_account_by_id(&account(), &document())
        .await
        .is_err());
}
