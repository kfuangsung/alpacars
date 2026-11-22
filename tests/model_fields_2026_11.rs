//! Request and response fields Alpaca added or retyped between 2026-10-20 and 2026-11-20, in
//! the shapes the API references document. Fixtures are the references' own examples where
//! they have one, and are otherwise built from the schema.

mod common;

use alpacars::broker::enums::TaxIdType;
use alpacars::broker::models::Account;
use alpacars::broker::requests::{
    AccountDrip, CreateAccountRequest, CreateIdentityRequest, Identification,
};
use alpacars::data::enums::Adjustment;
use alpacars::data::historical::stock::StockBarsRequest;
use alpacars::trading::enums::{
    CommissionType, OrderClass, OrderSide, OrderType, PortfolioHistoryTimeframe, PositionIntent,
    TokenizationTransferStatus,
};
use alpacars::trading::requests::{GetPortfolioHistoryRequest, OptionLegRequest, OrderRequest};
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_ID: &str = "b1f9c2d4-5e6a-4b7c-8d9e-0f1a2b3c4d5e";

/// createorderforaccount, request example `MultilegOptionsWithCommission` (changelog
/// 2026-11-17).
fn mleg_with_commission_request() -> serde_json::Value {
    json!({
        "commission": "1.50",
        "commission_type": "qty",
        "legs": [
            {"position_intent": "buy_to_open", "ratio_qty": "1", "side": "buy", "symbol": "AAPL250620C00100000"},
            {"position_intent": "sell_to_open", "ratio_qty": "1", "side": "sell", "symbol": "AAPL250620C00105000"}
        ],
        "limit_price": "2.50",
        "order_class": "mleg",
        "qty": "1",
        "time_in_force": "day",
        "type": "limit"
    })
}

/// createorderforaccount, response example `MultilegOptionsWithCommissionOrderResponse`.
/// Note that the parent order reports `asset_id`, `asset_class`, `side` and `symbol` as `""`.
const MLEG_WITH_COMMISSION_RESPONSE: &str = r#"{
  "asset_class": "", "asset_id": "", "canceled_at": null,
  "client_order_id": "9e4a7c21-3b5d-4f8a-a6e2-1d0c9b8f7e65",
  "commission": "1.5", "commission_type": "qty",
  "created_at": "2026-11-12T15:02:11.204518Z", "expired_at": null, "extended_hours": false,
  "failed_at": null, "filled_at": null, "filled_avg_price": null, "filled_qty": "0", "hwm": null,
  "id": "1b6f2c8e-4d3a-4f7e-9a51-6c2e8d0b7f34",
  "legs": [
    {"asset_class": "us_option", "asset_id": "f0ea14b2-8a49-4e9b-89d1-894c6e518a76",
     "canceled_at": null, "client_order_id": "c3f8a1d6-7e2b-4a9c-8d5f-2b1e0c6a9d47",
     "created_at": "2026-11-12T15:02:11.204518Z", "expired_at": null, "extended_hours": false,
     "failed_at": null, "filled_at": null, "filled_avg_price": null, "filled_qty": "0",
     "hwm": null, "id": "5a2d9e7c-8f1b-4c6a-b3e4-0d7f2a9c1e58", "legs": null,
     "limit_price": null, "notional": null, "order_class": "mleg", "order_type": "",
     "position_intent": "buy_to_open", "qty": "1", "ratio_qty": "1", "replaced_at": null,
     "replaced_by": null, "replaces": null, "side": "buy", "source": null, "status": "accepted",
     "stop_price": null, "submitted_at": "2026-11-12T15:02:11.193402Z", "subtag": null,
     "symbol": "AAPL250620C00100000", "time_in_force": "day", "trail_percent": null,
     "trail_price": null, "type": "", "updated_at": "2026-11-12T15:02:11.204518Z"},
    {"asset_class": "us_option", "asset_id": "f89940db-eeb1-46e6-8f9b-bb1f27a0b395",
     "canceled_at": null, "client_order_id": "6d2b9f4e-1a8c-4d7b-9e3f-5c0a8b7d2e16",
     "created_at": "2026-11-12T15:02:11.204518Z", "expired_at": null, "extended_hours": false,
     "failed_at": null, "filled_at": null, "filled_avg_price": null, "filled_qty": "0",
     "hwm": null, "id": "8e1c4b7a-2d9f-4e3c-a5b8-7f6d0e2c1a93", "legs": null,
     "limit_price": null, "notional": null, "order_class": "mleg", "order_type": "",
     "position_intent": "sell_to_open", "qty": "1", "ratio_qty": "1", "replaced_at": null,
     "replaced_by": null, "replaces": null, "side": "sell", "source": null, "status": "accepted",
     "stop_price": null, "submitted_at": "2026-11-12T15:02:11.193402Z", "subtag": null,
     "symbol": "AAPL250620C00105000", "time_in_force": "day", "trail_percent": null,
     "trail_price": null, "type": "", "updated_at": "2026-11-12T15:02:11.204518Z"}
  ],
  "limit_price": "2.5", "notional": null, "order_class": "mleg", "order_type": "limit",
  "qty": "1", "replaced_at": null, "replaced_by": null, "replaces": null, "side": "",
  "source": null, "status": "accepted", "stop_price": null,
  "submitted_at": "2026-11-12T15:02:11.193402Z", "subtag": null, "symbol": "",
  "time_in_force": "day", "trail_percent": null, "trail_price": null, "type": "limit",
  "updated_at": "2026-11-12T15:02:11.204518Z"
}"#;

fn leg(symbol: &str, side: OrderSide, intent: PositionIntent) -> OptionLegRequest {
    OptionLegRequest {
        symbol: symbol.to_string(),
        ratio_qty: "1".to_string(),
        side: Some(side),
        position_intent: Some(intent),
    }
}

/// An mleg order has no top-level `symbol` or `side`, and carries a per-contract commission.
/// The whole body must equal the reference example: the crate used to send `symbol` and
/// `side` on every order and a numeric `ratio_qty`.
#[tokio::test]
async fn test_broker_mleg_order_with_commission() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(format!("/v1/trading/accounts/{ACCOUNT_ID}/orders")))
        .and(body_json(mleg_with_commission_request()))
        .respond_with(ResponseTemplate::new(200).set_body_string(MLEG_WITH_COMMISSION_RESPONSE))
        .expect(1)
        .mount(&server)
        .await;

    let mut order = OrderRequest::mleg(
        "1",
        vec![
            leg(
                "AAPL250620C00100000",
                OrderSide::Buy,
                PositionIntent::BuyToOpen,
            ),
            leg(
                "AAPL250620C00105000",
                OrderSide::Sell,
                PositionIntent::SellToOpen,
            ),
        ],
    );
    order.order_type = OrderType::Limit;
    order.limit_price = Some("2.50".to_string());
    order.commission = Some("1.50".to_string());
    order.commission_type = Some(CommissionType::Qty);

    let client = common::broker_client(&server.uri());
    let account: Uuid = ACCOUNT_ID.parse().unwrap();
    let placed = client
        .submit_order_for_account(&account, &order)
        .await
        .unwrap();

    assert_eq!(placed.order_class, OrderClass::Mleg);
    assert_eq!(placed.commission.as_deref(), Some("1.5"));
    assert_eq!(placed.commission_type, Some(CommissionType::Qty));
    // The parent order reports an empty asset ID; it is not a parse error.
    assert_eq!(placed.asset_id, None);
    let legs = placed.legs.unwrap();
    assert_eq!(legs.len(), 2);
    assert_eq!(legs[1].position_intent, Some(PositionIntent::SellToOpen));
}

/// Order.settlement_currency (changelog 2026-10-23) is returned when the order has one.
#[test]
fn test_order_settlement_currency() {
    let mut value: serde_json::Value = serde_json::from_str(MLEG_WITH_COMMISSION_RESPONSE).unwrap();
    value["settlement_currency"] = json!("EUR");
    let order: alpacars::trading::models::Order = serde_json::from_value(value).unwrap();
    assert_eq!(order.settlement_currency.as_deref(), Some("EUR"));

    let without: alpacars::trading::models::Order =
        serde_json::from_str(MLEG_WITH_COMMISSION_RESPONSE).unwrap();
    assert_eq!(without.settlement_currency, None);
}

/// createaccount: `drip` (AccountDRIPRequest, changelog 2026-11-05) and
/// `identity.identifications` (Identification, changelog 2026-11-16). No named example
/// covers either, so the body follows the schemas.
#[tokio::test]
async fn test_create_account_with_drip_and_identifications() {
    let server = MockServer::start().await;
    let body = json!({
        "identity": {
            "given_name": "John",
            "family_name": "Doe",
            "date_of_birth": "1990-01-01",
            "tax_id": "666-55-4321",
            "tax_id_type": "USA_SSN",
            "country_of_citizenship": "USA",
            "country_of_birth": "USA",
            "country_of_tax_residence": "USA",
            "funding_source": ["employment_income"],
            "identifications": [{
                "type": "passport",
                "number": "X1234567",
                "issued_country": "USA",
                "issued_date": "2020-01-15",
                "expiry_date": "2030-01-14"
            }]
        },
        "drip": {"equities_dividend_reinvestments": true}
    });
    // AccountDRIPResponse requires `equities_dividend_reinvestments`; identifications come
    // back with `type` only, and include the tax ID.
    let response = json!({
        "id": ACCOUNT_ID,
        "account_number": "601843184",
        "status": "SUBMITTED",
        "identity": {
            "given_name": "John",
            "family_name": "Doe",
            "identifications": [{"type": "passport"}, {"type": "USA_SSN"}]
        },
        "drip": {"equities_dividend_reinvestments": true}
    });
    Mock::given(method("POST"))
        .and(path("/v1/accounts"))
        .and(body_json(&body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response))
        .expect(1)
        .mount(&server)
        .await;

    let req = CreateAccountRequest {
        identity: Some(CreateIdentityRequest {
            given_name: "John".into(),
            middle_name: None,
            family_name: "Doe".into(),
            date_of_birth: "1990-01-01".parse().unwrap(),
            tax_id: Some("666-55-4321".into()),
            tax_id_type: Some(TaxIdType::UsaSsn),
            country_of_citizenship: Some("USA".into()),
            country_of_birth: Some("USA".into()),
            country_of_tax_residence: "USA".into(),
            visa_type: None,
            visa_expiration_date: None,
            funding_source: vec![alpacars::broker::enums::FundingSource::EmploymentIncome],
            identifications: Some(vec![Identification {
                id_type: "passport".into(),
                number: Some("X1234567".into()),
                issued_country: Some("USA".into()),
                issued_date: Some("2020-01-15".parse().unwrap()),
                expiry_date: Some("2030-01-14".parse().unwrap()),
            }]),
        }),
        drip: Some(AccountDrip {
            equities_dividend_reinvestments: Some(true),
        }),
        ..Default::default()
    };

    let client = common::broker_client(&server.uri());
    let account: Account = client.create_account(&req).await.unwrap();

    assert_eq!(
        account.drip.unwrap().equities_dividend_reinvestments,
        Some(true)
    );
    let ids = account.identity.unwrap().identifications.unwrap();
    let types: Vec<&str> = ids.iter().map(|i| i.id_type.as_str()).collect();
    assert_eq!(types, ["passport", "USA_SSN"]);
    assert_eq!(ids[0].number, None);
}

/// gettokenizationrequest 200 example, which carries `transfer_status` (changelog 2026-10-27).
#[tokio::test]
async fn test_tokenization_request_transfer_status() {
    let server = MockServer::start().await;
    let id = "5b1d6a3e-7f0a-4d2c-b8e1-9e6f1c0d2c4a";
    Mock::given(method("GET"))
        .and(path(format!("/v2/tokenization/requests/{id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "created_at": "2025-11-04T18:38:01.942282Z",
            "fees": "0.5",
            "issuer": "xstocks",
            "network": "solana",
            "qty": "1.0",
            "status": "completed",
            "token_symbol": "TSLAx",
            "tokenization_request_id": id,
            "transfer_status": "completed",
            "tx_hash": "5J7ZxK4QwR3vH2pY1aB6sN8cM9tD0fA2eXyVwUjL3kPqR4mZcF7nB1tA8sH6dG2pE",
            "type": "mint",
            "underlying_symbol": "TSLA",
            "updated_at": "2025-11-04T18:38:42.117004Z",
            "wallet_address": "5dXY1aH2tQpV3wXmJg6Z7c8B4nKvF9bA1pQrSt2uVwYxXz"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let req = client.get_tokenization_request(id).await.unwrap();
    assert_eq!(
        req.transfer_status,
        Some(TokenizationTransferStatus::Completed)
    );
}

/// getaccountportfoliohistory-1: `timeframe` is the PortfolioHistoryTimeframe enum (changelog
/// 2026-10-21). Response is the `example-query-1d-7d` example.
#[tokio::test]
async fn test_portfolio_history_timeframe_is_typed() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/account/portfolio/history"))
        .and(query_param("timeframe", "1D"))
        .and(query_param("period", "1W"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "base_value": 2784.79,
            "cashflow": {"CSD": [0, 0, 100, 0, 0]},
            "equity": [2784.79, 2794.79, 2805.46, 2774.16, 2748.73],
            "profit_loss": [0, 10.0022, 10.6692, -31.2996, -25.4232],
            "profit_loss_pct": [0, 0.0035, 0.0074, -0.0038, -0.0129],
            "timeframe": "1D",
            "timestamp": [1697241600, 1697500800, 1697587200, 1697673600, 1697760000]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let history = client
        .get_portfolio_history(Some(&GetPortfolioHistoryRequest {
            period: Some("1W".into()),
            timeframe: Some(PortfolioHistoryTimeframe::OneDay),
            ..Default::default()
        }))
        .await
        .unwrap();
    assert_eq!(history.equity.len(), 5);
}

/// stockbars: `adjustment` is an array sent as one comma-separated value (changelog
/// 2026-11-02), so adjustments can be combined.
#[tokio::test]
async fn test_stock_bars_adjustment_is_a_list() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .and(query_param("adjustment", "split,spin-off"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"bars": {"AAPL": []}, "next_page_token": null})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    client
        .get_stock_bars(&StockBarsRequest {
            symbols: vec!["AAPL".into()],
            adjustment: Some(vec![Adjustment::Split, Adjustment::SpinOff]),
            ..Default::default()
        })
        .await
        .unwrap();
}
