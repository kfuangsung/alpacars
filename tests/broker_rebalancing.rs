//! Rebalancing endpoints are served under a /v1/rebalancing prefix. Every mock
//! here asserts the full path, so a regression back to bare /v1/portfolios,
//! /v1/subscriptions or /v1/runs fails loudly rather than 404ing at runtime.
//!
//! Response fixtures follow the Portfolio / PortfolioRun / PortfolioSubscription schemas on
//! docs.alpaca.markets/us/reference, and request bodies are asserted against the reference's
//! own examples. An earlier fixture copied the crate's wrong `assets` key, which let
//! `create_portfolio` ship sending a field the API does not accept.

mod common;

use alpacars::broker::enums::{PortfolioStatus, RunInitiatedFrom, RunType, WeightType};
use alpacars::broker::requests::{
    CreatePortfolioRequest, CreateRunRequest, CreateSubscriptionRequest, GetPortfoliosRequest,
    GetRunsRequest, PortfolioWeightRequest,
};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const PORTFOLIO_ID: &str = "2d49d00e-ab1c-4f1a-9b3a-5f0e9c1d2a3b";
const SUBSCRIPTION_ID: &str = "3e5ab111-9d02-4c8e-8f77-1a2b3c4d5e6f";
const RUN_ID: &str = "4f6bc222-0e13-4d9f-9a88-2b3c4d5e6f70";
const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";

const PORTFOLIO_JSON: &str = r#"{
  "id": "2d49d00e-ab1c-4f1a-9b3a-5f0e9c1d2a3b",
  "name": "Balanced",
  "description": "60/40",
  "status": "active",
  "cooldown_days": 7,
  "created_at": "2026-07-20T10:00:00Z",
  "updated_at": "2026-07-20T10:00:00Z",
  "weights": [
    {"type": "asset", "symbol": "AAPL", "percent": "55"},
    {"type": "asset", "symbol": "BND", "percent": "40"},
    {"type": "cash", "symbol": null, "percent": "5"}
  ],
  "rebalancing_conditions": [{"type": "drift_band", "sub_type": "absolute", "percent": "5"}]
}"#;

const RUN_JSON: &str = r#"{
  "id": "4f6bc222-0e13-4d9f-9a88-2b3c4d5e6f70",
  "account_id": "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e",
  "portfolio_id": "2d49d00e-ab1c-4f1a-9b3a-5f0e9c1d2a3b",
  "type": "full_rebalance",
  "initiated_from": "api",
  "weights": [{"type": "asset", "symbol": "AAPL", "percent": "100"}],
  "canceled_at": null,
  "status": "COMPLETED_SUCCESS",
  "reason": null,
  "created_at": "2026-07-20T10:00:00Z",
  "updated_at": "2026-07-20T10:01:00Z",
  "completed_at": "2026-07-20T10:01:00Z",
  "orders": null,
  "failed_orders": null,
  "skipped_orders": null
}"#;

fn uuid(s: &str) -> Uuid {
    s.parse().unwrap()
}

fn asset_weight(symbol: &str, percent: &str) -> PortfolioWeightRequest {
    PortfolioWeightRequest {
        weight_type: WeightType::Asset,
        symbol: Some(symbol.to_string()),
        percent: percent.to_string(),
    }
}

#[tokio::test]
async fn test_create_portfolio() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/rebalancing/portfolios"))
        .and(body_json(json!({
            "name": "Balanced",
            "description": "60/40",
            "weights": [
                {"type": "asset", "symbol": "AAPL", "percent": "55"},
                {"type": "asset", "symbol": "BND", "percent": "40"},
                {"type": "cash", "percent": "5"}
            ],
            "cooldown_days": 7
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(PORTFOLIO_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreatePortfolioRequest {
        name: "Balanced".to_string(),
        description: Some("60/40".to_string()),
        weights: vec![
            asset_weight("AAPL", "55"),
            asset_weight("BND", "40"),
            PortfolioWeightRequest {
                weight_type: WeightType::Cash,
                symbol: None,
                percent: "5".to_string(),
            },
        ],
        cooldown_days: Some(7),
        rebalance_conditions: None,
    };
    let portfolio = client.create_portfolio(&req).await.unwrap();

    assert_eq!(portfolio.name.as_deref(), Some("Balanced"));
    assert_eq!(portfolio.status, Some(PortfolioStatus::Active));
    let weights = portfolio.weights.expect("weights");
    assert_eq!(weights.len(), 3);
    assert_eq!(weights[0].weight_type, WeightType::Asset);
    assert_eq!(weights[0].symbol.as_deref(), Some("AAPL"));
    assert_eq!(weights[2].weight_type, WeightType::Cash);
    assert_eq!(weights[2].symbol, None);
    assert!(portfolio.rebalancing_conditions.is_some());
}

#[tokio::test]
async fn test_get_all_portfolios() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/rebalancing/portfolios"))
        .and(query_param("status", "active"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!("[{}]", PORTFOLIO_JSON)))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetPortfoliosRequest {
        status: Some(PortfolioStatus::Active),
    };
    let portfolios = client.get_all_portfolios(Some(&filter)).await.unwrap();

    assert_eq!(portfolios.len(), 1);
    assert_eq!(portfolios[0].id, uuid(PORTFOLIO_ID));
}

#[tokio::test]
async fn test_get_portfolio_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(format!("/v1/rebalancing/portfolios/{}", PORTFOLIO_ID)))
        .respond_with(ResponseTemplate::new(200).set_body_string(PORTFOLIO_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let portfolio = client
        .get_portfolio_by_id(&uuid(PORTFOLIO_ID))
        .await
        .unwrap();

    assert_eq!(portfolio.cooldown_days, Some(7));
}

/// Archiving is a DELETE returning 204, not a POST to a `:inactivate` action.
#[tokio::test]
async fn test_archive_portfolio_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/v1/rebalancing/portfolios/{}", PORTFOLIO_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .archive_portfolio_by_id(&uuid(PORTFOLIO_ID))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_subscription() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/rebalancing/subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{
              "id": "{}",
              "account_id": "{}",
              "portfolio_id": "{}",
              "status": "active",
              "created_at": "2026-07-20T10:00:00Z",
              "updated_at": null,
              "last_rebalanced_at": "2026-07-21T14:30:00Z"
            }}"#,
            SUBSCRIPTION_ID, ACCOUNT_ID, PORTFOLIO_ID
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateSubscriptionRequest {
        account_id: uuid(ACCOUNT_ID),
        portfolio_id: uuid(PORTFOLIO_ID),
    };
    let sub = client.create_subscription(&req).await.unwrap();

    assert_eq!(sub.id, uuid(SUBSCRIPTION_ID));
    assert_eq!(sub.portfolio_id, Some(uuid(PORTFOLIO_ID)));
    assert!(sub.last_rebalanced_at.is_some());
}

#[tokio::test]
async fn test_unsubscribe_account() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!(
            "/v1/rebalancing/subscriptions/{}",
            SUBSCRIPTION_ID
        )))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client
        .unsubscribe_account(&uuid(SUBSCRIPTION_ID))
        .await
        .unwrap();
}

/// Body must equal the `manualRun` request example on the Create Run reference page.
#[tokio::test]
async fn test_create_manual_run() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/rebalancing/runs"))
        .and(body_json(json!({
            "account_id": "bf2b0f93-f296-4276-a9cf-288586cf4fb7",
            "type": "full_rebalance",
            "weights": [
                {"percent": "35", "symbol": "AAPL", "type": "asset"},
                {"percent": "20", "symbol": "TSLA", "type": "asset"},
                {"percent": "45", "symbol": "SPY", "type": "asset"}
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(RUN_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateRunRequest {
        account_id: uuid("bf2b0f93-f296-4276-a9cf-288586cf4fb7"),
        run_type: RunType::FullRebalance,
        amount: None,
        weights: Some(vec![
            asset_weight("AAPL", "35"),
            asset_weight("TSLA", "20"),
            asset_weight("SPY", "45"),
        ]),
    };
    let run = client.create_manual_run(&req).await.unwrap();

    assert_eq!(run.id, uuid(RUN_ID));
    assert_eq!(run.account_id, Some(uuid(ACCOUNT_ID)));
    assert_eq!(run.run_type, Some(RunType::FullRebalance));
    assert_eq!(run.initiated_from, Some(RunInitiatedFrom::Api));
    assert_eq!(run.weights.map(|w| w.len()), Some(1));
}

/// Body must equal the `subscriptionRun` request example: no weights at all.
#[tokio::test]
async fn test_create_subscription_run() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/rebalancing/runs"))
        .and(body_json(json!({
            "account_id": "bf2b0f93-f296-4276-a9cf-288586cf4fb7",
            "type": "full_rebalance"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_string(RUN_JSON))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let req = CreateRunRequest {
        account_id: uuid("bf2b0f93-f296-4276-a9cf-288586cf4fb7"),
        run_type: RunType::FullRebalance,
        amount: None,
        weights: None,
    };
    client.create_manual_run(&req).await.unwrap();
}

/// List All Runs returns {runs, next_page_token}, not a bare array.
#[tokio::test]
async fn test_get_all_runs_is_paginated() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/rebalancing/runs"))
        .and(query_param("account_id", ACCOUNT_ID))
        .and(query_param("type", "full_rebalance"))
        .respond_with(ResponseTemplate::new(200).set_body_string(format!(
            r#"{{"runs": [{}], "next_page_token": "eyJwYWdlIjoyfQ"}}"#,
            RUN_JSON
        )))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    let filter = GetRunsRequest {
        account_id: Some(uuid(ACCOUNT_ID)),
        run_type: Some(RunType::FullRebalance),
        ..Default::default()
    };
    let resp = client.get_all_runs(Some(&filter)).await.unwrap();

    assert_eq!(resp.runs.len(), 1);
    assert_eq!(resp.runs[0].id, uuid(RUN_ID));
    assert_eq!(resp.next_page_token.as_deref(), Some("eyJwYWdlIjoyfQ"));
}

#[tokio::test]
async fn test_cancel_run_by_id() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/v1/rebalancing/runs/{}", RUN_ID)))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::broker_client(&server.uri());
    client.cancel_run_by_id(&uuid(RUN_ID)).await.unwrap();
}
