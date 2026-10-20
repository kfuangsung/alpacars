//! Trading corporate action announcements (`GET /v2/corporate_actions/announcements`), checked
//! against get-v2-corporate_actions-announcements-1. The response fixture is that reference's
//! "Example 1", verbatim.

mod common;

use alpacars::trading::enums::{CorporateActionSubType, CorporateActionType};
use alpacars::trading::requests::GetCorporateAnnouncementsRequest;
use chrono::NaiveDate;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const EXAMPLE_1: &str = r#"[
  {
    "ca_sub_type": "DIV",
    "ca_type": "Dividend",
    "cash": "0.018",
    "corporate_actions_id": "F58684224_XY37",
    "declaration_date": "2021-01-05",
    "expiration_date": "2021-01-12",
    "id": "be3c368a-4c7c-4384-808e-f02c9f5a8afe",
    "initiating_original_cusip": "55275E101",
    "initiating_symbol": "MLLAX",
    "new_rate": "1",
    "old_rate": "1",
    "payable_date": "2021-01-14",
    "record_date": "2021-01-13",
    "target_original_cusip": "55275E101",
    "target_symbol": "MLLAX"
  },
  {
    "ca_sub_type": "cash",
    "ca_type": "Dividend",
    "cash": "0.145",
    "corporate_action_id": "48251W104_AD21",
    "declaration_date": "2021-11-01",
    "ex_date": "2021-11-12",
    "initiating_original_cusip": "G52830109",
    "initiating_symbol": "KKR",
    "new_rate": "1",
    "old_rate": "1",
    "payable_date": "2021-11-30",
    "record_date": "2021-11-15",
    "target_original_cusip": "G52830109",
    "target_symbol": "KKR"
  }
]"#;

/// `ca_types` is a comma-separated list of capitalized values (`Dividend`), and `ca_type` in
/// the response uses the same values; the crate used to send and expect `dividend`, so every
/// announcement's type parsed as `Unknown`.
#[tokio::test]
async fn test_get_corporate_announcements() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/corporate_actions/announcements"))
        .and(query_param("ca_types", "Dividend,Merger"))
        .and(query_param("since", "2021-01-01"))
        .and(query_param("until", "2021-03-01"))
        .respond_with(ResponseTemplate::new(200).set_body_string(EXAMPLE_1))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = GetCorporateAnnouncementsRequest {
        ca_types: vec![CorporateActionType::Dividend, CorporateActionType::Merger],
        since: NaiveDate::from_ymd_opt(2021, 1, 1),
        until: NaiveDate::from_ymd_opt(2021, 3, 1),
        symbol: None,
        cusip: None,
        date_type: None,
    };
    let announcements = client.get_corporate_announcements(&req).await.unwrap();
    assert_eq!(announcements.len(), 2);
    assert_eq!(
        announcements[0].ca_type,
        Some(CorporateActionType::Dividend)
    );
    assert_eq!(
        announcements[0].corporate_action_id.as_deref(),
        Some("F58684224_XY37")
    );
    assert_eq!(announcements[1].id, None);
    assert_eq!(
        announcements[1].ca_sub_type,
        Some(CorporateActionSubType::Cash)
    );
}
