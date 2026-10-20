//! Alpaca adds enum values over time. Every enum that appears in a response must parse an
//! unrecognized value as `Unknown` instead of failing the whole response.

use alpacars::trading::enums::{OrderStatus, TokenizationIssuer, TokenizationNetwork};
use alpacars::trading::models::{Order, TokenizationRequest};
use serde_json::json;

const NEW_VALUE: &str = "value_alpaca_added_later";

macro_rules! assert_unknown_fallback {
    ($($ty:path),+ $(,)?) => {
        $(
            {
                use $ty as E;
                let parsed: E = serde_json::from_value(json!(NEW_VALUE)).unwrap_or_else(|e| {
                    panic!("{} rejected an unseen value: {e}", stringify!($ty))
                });
                assert_eq!(parsed, E::Unknown, "{}", stringify!($ty));
            }
        )+
    };
}

#[test]
fn test_every_response_enum_has_unknown_fallback() {
    assert_unknown_fallback!(
        alpacars::broker::enums::AccountType,
        alpacars::broker::enums::TaxIdType,
        alpacars::broker::enums::VisaType,
        alpacars::broker::enums::FundingSource,
        alpacars::broker::enums::EmploymentStatus,
        alpacars::broker::enums::AgreementType,
        alpacars::broker::enums::DocumentType,
        alpacars::broker::enums::TradeDocumentType,
        alpacars::broker::enums::ACHRelationshipStatus,
        alpacars::broker::enums::BankAccountType,
        alpacars::broker::enums::TransferType,
        alpacars::broker::enums::TransferStatus,
        alpacars::broker::enums::TransferDirection,
        alpacars::broker::enums::JournalEntryType,
        alpacars::broker::enums::JournalStatus,
        alpacars::broker::enums::PortfolioStatus,
        alpacars::broker::enums::WeightType,
        alpacars::broker::enums::RunType,
        alpacars::broker::enums::RunInitiatedFrom,
        alpacars::broker::enums::SignedAgreementType,
        alpacars::broker::enums::AcatsTransferDirection,
        alpacars::broker::enums::AcatsTransferStatus,
        alpacars::broker::enums::AcatsDtccStatus,
        alpacars::broker::enums::AcatsDtccTransferType,
        alpacars::broker::enums::AcatsAccountType,
        alpacars::broker::enums::AcatsContraBrokerType,
        alpacars::broker::enums::AcatsRejectionReason,
        alpacars::broker::enums::AcatsPosition,
        alpacars::broker::enums::AcatsAssetClass,
        alpacars::broker::enums::AcatsAssetStatus,
        alpacars::data::enums::Exchange,
        alpacars::data::enums::DataFeed,
        alpacars::trading::enums::OrderType,
        alpacars::trading::enums::OrderSide,
        alpacars::trading::enums::OrderStatus,
        alpacars::trading::enums::OrderClass,
        alpacars::trading::enums::TimeInForce,
        alpacars::trading::enums::AssetClass,
        alpacars::trading::enums::AssetStatus,
        alpacars::trading::enums::BorrowStatus,
        alpacars::trading::enums::AssetExchange,
        alpacars::trading::enums::PositionSide,
        alpacars::trading::enums::CorporateActionType,
        alpacars::trading::enums::CorporateActionSubType,
        alpacars::trading::enums::AccountStatus,
        alpacars::trading::enums::TradeEvent,
        alpacars::trading::enums::TradeConfirmationEmail,
        alpacars::trading::enums::ContractType,
        alpacars::trading::enums::ExerciseStyle,
        alpacars::trading::enums::ActivityType,
        alpacars::trading::enums::TradeActivityType,
        alpacars::trading::enums::NonTradeActivityStatus,
        alpacars::trading::enums::LocateStatus,
        alpacars::trading::enums::PositionIntent,
        alpacars::trading::enums::OrderGroupRole,
        alpacars::trading::enums::TokenizationRequestStatus,
        alpacars::trading::enums::TokenizationRequestType,
        alpacars::trading::enums::TokenizationNetwork,
        alpacars::trading::enums::TokenizationIssuer,
    );
}

#[test]
fn test_known_values_still_parse_exactly() {
    // The fallback must not swallow values the crate does know.
    let issuer: TokenizationIssuer = serde_json::from_value(json!("st0x")).unwrap();
    assert_eq!(issuer, TokenizationIssuer::St0x);
    let status: OrderStatus = serde_json::from_value(json!("partially_filled")).unwrap();
    assert_eq!(status, OrderStatus::PartiallyFilled);
}

#[test]
fn test_tokenization_request_with_unseen_issuer_and_network_parses() {
    // Before the fallback, a single new issuer on any record failed get_tokenization_requests.
    let req: TokenizationRequest = serde_json::from_value(json!({
        "tokenization_request_id": "6c3f8f2e-1a44-4c9b-9d21-3e7a5b8c0d11",
        "status": "completed",
        "type": "mint",
        "underlying_symbol": "TSLA",
        "token_symbol": "TSLAx",
        "qty": "3",
        "created_at": "2026-07-16T10:00:00Z",
        "issuer": NEW_VALUE,
        "network": NEW_VALUE
    }))
    .unwrap();
    assert_eq!(req.issuer, TokenizationIssuer::Unknown);
    assert_eq!(req.network, TokenizationNetwork::Unknown);
}

#[test]
fn test_order_with_unseen_status_parses() {
    let order: Order = serde_json::from_value(json!({
        "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
        "client_order_id": "eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4",
        "order_class": "simple",
        "time_in_force": "day",
        "status": NEW_VALUE,
        "extended_hours": false
    }))
    .unwrap();
    assert_eq!(order.status, OrderStatus::Unknown);
}
