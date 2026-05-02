mod common;

use alpacars::trading::enums::{DTBPCheck, PDTCheck, TradeConfirmationEmail};
use alpacars::trading::models::{AccountConfiguration, TradeAccount};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_account() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/account"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "account_blocked": false,
              "accrued_fees": "0",
              "account_number": "010203ABCD",
              "buying_power": "262113.632",
              "cash": "-23140.2",
              "created_at": "2019-06-12T22:47:07.99658Z",
              "currency": "USD",
              "daytrade_count": 0,
              "daytrading_buying_power": "262113.632",
              "equity": "103820.56",
              "id": "e6fe16f3-64a4-4921-8928-cadf02f92f98",
              "initial_margin": "63480.38",
              "last_equity": "103529.24",
              "last_maintenance_margin": "38000.832",
              "non_marginable_buying_power": "98945.02",
              "long_market_value": "126960.76",
              "maintenance_margin": "38088.228",
              "multiplier": "4",
              "pattern_day_trader": false,
              "portfolio_value": "103820.56",
              "regt_buying_power": "80680.36",
              "short_market_value": "0",
              "shorting_enabled": true,
              "sma": "0",
              "status": "ACTIVE",
              "trade_suspended_by_user": false,
              "trading_blocked": false,
              "transfers_blocked": false,
              "options_buying_power": "262113.632",
              "options_approved_level": 1,
              "options_trading_level": 1
            }"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let account = client.get_account().await.unwrap();

    assert!(matches!(account, TradeAccount { .. }));
    assert_eq!(account.buying_power.as_deref(), Some("262113.632"));
    assert_eq!(account.options_buying_power.as_deref(), Some("262113.632"));
}

#[tokio::test]
async fn test_get_account_configurations() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/account/configurations"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
              "dtbp_check": "entry",
              "no_shorting": false,
              "suspend_trade": false,
              "fractional_trading": true,
              "max_margin_multiplier": "4",
              "pdt_check": "entry",
              "trade_confirm_email": "all",
              "ptp_no_exception_entry": false,
              "max_options_trading_level": 1
            }"#,
        ))
        .mount(&server)
        .await;

    let client = common::trading_client(&server.uri());
    let config = client.get_account_configurations().await.unwrap();

    assert!(matches!(config, AccountConfiguration { .. }));
    assert_eq!(config.dtbp_check, Some(DTBPCheck::Entry));
    assert_eq!(config.pdt_check, Some(PDTCheck::Entry));
    assert_eq!(config.trade_confirm_email, Some(TradeConfirmationEmail::All));
    assert_eq!(config.max_options_trading_level, Some(1));
}
