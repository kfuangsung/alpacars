mod common;

use alpacars::data::historical::corporate_actions::CorporateActionsRequest;
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

// The `capital_gains_distribution` schema example on
// docs.alpaca.markets/us/reference/corporateactions-1.
const CGD_JSON: &str = r#"{
  "currency": "USD",
  "cusip": "36261K509",
  "ex_date": "2025-12-26",
  "id": "98c905a1-08c7-450c-8669-b8e7aee8b44d",
  "isin": "US36261K5090",
  "long_term_rate": 0.45855,
  "payable_date": "2026-01-09",
  "process_date": "2026-01-08",
  "record_date": "2025-12-29",
  "short_term_rate": 0.11019,
  "symbol": "GCAD"
}"#;

/// The endpoint is GA at /v1/corporate-actions; the crate previously called /v1beta1.
/// Pages are merged, including the action types added since the last sync.
#[tokio::test]
async fn test_get_corporate_actions_v1_path_filters_and_pagination() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param(
            "types",
            "capital_gains_distribution,reorganization",
        ))
        .and(query_param("data_quality", "all"))
        .and(query_param("region", "us"))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{"corporate_actions": {{"capital_gains_distributions": [{CGD_JSON}]}},
                "next_page_token": "page2"}}"#
        )))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1/corporate-actions"))
        .and(query_param("page_token", "page2"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{"corporate_actions": {{
                  "capital_gains_distributions": [{CGD_JSON}],
                  "reorganizations": [{{"id": "r1", "symbol": "XYZ"}}],
                  "partial_calls": [{{"id": "p1", "symbol": "ABC"}}]
                }},
                "next_page_token": null}}"#
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::corporate_actions_client(&server.uri());
    let req = CorporateActionsRequest {
        types: Some("capital_gains_distribution,reorganization".to_string()),
        data_quality: Some("all".to_string()),
        region: Some("us".to_string()),
        ..Default::default()
    };
    let set = client.get_corporate_actions(&req).await.unwrap();

    let cgd = set.capital_gains_distributions.expect("capital gains");
    assert_eq!(cgd.len(), 2, "both pages merged");
    assert_eq!(cgd[0]["symbol"], "GCAD");
    assert_eq!(set.reorganizations.map(|v| v.len()), Some(1));
    assert_eq!(set.partial_calls.map(|v| v.len()), Some(1));
}
