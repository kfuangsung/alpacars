//! Every value an API reference lists for an enum must map to a real variant (not `Unknown`)
//! and serialize back to the same string. The lists are copied from docs.alpaca.markets/us/reference,
//! not derived from the crate.

use alpacars::broker::enums::{
    AccountType, JournalStatus, PartyAgreementType, PerAccountAgreementType, SignedAgreementType,
};
use alpacars::trading::enums::{TokenizationIssuer, TokenizationNetwork};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::fmt::Debug;

fn assert_round_trips<E>(reference_values: &[&str], unknown: E)
where
    E: DeserializeOwned + Serialize + PartialEq + Debug,
{
    for &wire in reference_values {
        let parsed: E = serde_json::from_value(json!(wire))
            .unwrap_or_else(|e| panic!("{wire:?} failed to parse: {e}"));
        assert_ne!(
            parsed, unknown,
            "{wire:?} is in the reference but parsed as Unknown"
        );
        assert_eq!(serde_json::to_value(&parsed).unwrap(), json!(wire));
    }
}

/// For request-only enums, which have no `Unknown`: every reference value must round-trip.
fn assert_round_trips_strict<E>(reference_values: &[&str])
where
    E: DeserializeOwned + Serialize + Debug,
{
    for &wire in reference_values {
        let parsed: E = serde_json::from_value(json!(wire))
            .unwrap_or_else(|e| panic!("{wire:?} failed to parse: {e}"));
        assert_eq!(serde_json::to_value(&parsed).unwrap(), json!(wire));
    }
}

#[test]
fn test_tokenization_network_matches_reference() {
    // posttokenizationmint / posttokenizationmintbroker, TokenizationNetwork
    assert_round_trips(
        &[
            "arbitrum",
            "base",
            "binance",
            "cronos",
            "ethereum",
            "hypercore",
            "hyperevm",
            "mantle",
            "robinhood",
            "solana",
            "ton",
            "tron",
        ],
        TokenizationNetwork::Unknown,
    );
}

#[test]
fn test_tokenization_issuer_matches_reference() {
    // posttokenizationmint / posttokenizationmintbroker, TokenizationIssuer
    assert_round_trips(
        &["binance", "coinbase", "ondo", "st0x", "xstocks"],
        TokenizationIssuer::Unknown,
    );
}

#[test]
fn test_broker_account_type_matches_reference() {
    // getaccount, AccountType
    assert_round_trips(
        &[
            "trading",
            "custodial",
            "donor_advised",
            "ira",
            "trust",
            "omnibus_non_disclosed",
            "omnibus_sub",
            "hsa",
            "joint",
        ],
        AccountType::Unknown,
    );
}

#[test]
fn test_journal_status_matches_reference() {
    // getalljournals, JournalStatus
    assert_round_trips(
        &[
            "pending",
            "canceled",
            "executed",
            "activity_created",
            "queued",
            "rejected",
            "deleted",
            "refused",
            "sent_to_clearing",
            "correct",
        ],
        JournalStatus::Unknown,
    );
}

// createagreement: PartyAgreementType / PerAccountAgreementType / SignedAgreementType
const PARTY_AGREEMENTS: &[&str] = &[
    "margin_agreement",
    "certifications_and_resolutions_statement",
    "nasdaq_agreement",
    "nyse_agreement",
    "oauth_api_agreement",
    "opra_agreement",
    "trusted_contact_consent",
    "custodian_customer_agreement",
];
const ACCOUNT_AGREEMENTS: &[&str] = &[
    "account_agreement",
    "customer_agreement",
    "options_agreement",
    "master_securities_lending_agreement",
    "crypto_agreement",
    "dvp_customer_agreement",
    "etc_agreement",
    "joint_account_agreement",
];

#[test]
fn test_agreement_types_match_reference() {
    assert_round_trips_strict::<PartyAgreementType>(PARTY_AGREEMENTS);
    assert_round_trips_strict::<PerAccountAgreementType>(ACCOUNT_AGREEMENTS);
    // SignedAgreementType is exactly the union of the two.
    let all: Vec<&str> = PARTY_AGREEMENTS
        .iter()
        .chain(ACCOUNT_AGREEMENTS)
        .copied()
        .collect();
    assert_round_trips(&all, SignedAgreementType::Unknown);
}

// ACATS references (listacatstransfers, requestacatstransfer, listacatsassets,
// listacatscontrabrokers): every listed value of every ACATS enum.
#[test]
fn test_acats_enums_match_reference() {
    assert_round_trips(
        &["INCOMING", "OUTGOING"],
        alpacars::broker::enums::AcatsTransferDirection::Unknown,
    );
    assert_round_trips(
        &["PENDING", "IN_PROGRESS", "REJECTED", "SETTLED"],
        alpacars::broker::enums::AcatsTransferStatus::Unknown,
    );
    assert_round_trips(
        &[
            "REQUEST",
            "REQUEST_ADJUST",
            "REQUEST_ADJUST_PAST",
            "REQUEST_PAST",
            "REQUEST_REJECT",
            "REVIEW",
            "REVIEW_ADJUST_DELIVERER",
            "REVIEW_ERROR",
            "REVIEW_ACCELERATE",
            "REVIEW_ADJUST_RECEIVER_ACCELERATE",
            "SETTLE_PREP",
            "SETTLE_CLOSE",
            "CLOSE_PURGE",
            "REQUEST_PTR",
            "MEMO_PURGE_PARTIAL_TRANSFER_REQUEST_RECEIVER",
            "REJECT",
            "SYSTEM_REJECTED",
        ],
        alpacars::broker::enums::AcatsDtccStatus::Unknown,
    );
    assert_round_trips(
        &["FUL", "FRV", "MFC", "PTD", "PTR", "RCL", "RCR", "PTF"],
        alpacars::broker::enums::AcatsDtccTransferType::Unknown,
    );
    assert_round_trips(
        &[
            "AGENCY",
            "BANK_CUSTODY",
            "BENEFICIARY",
            "BENEFICIARY_ROTH_IRA",
            "CORPORATE",
            "CO_TRUSTEE",
            "COVERDELL_IRA",
            "CUSTODIAN_UGMA",
            "DIRECT_ROLLOVER",
            "ESTATE",
            "HSA",
            "IRA",
            "JOINT",
            "MEDICAL_SAVINGS_ACCOUNT",
            "OTHER",
            "QUALIFIED_OR_PROFIT_SHARING_OR_PENSION",
            "ROTH_401K",
            "ROTH_IRA",
            "ROTH_SEP",
            "ROTH_SIMPLE",
            "ROTH_SOLO_401K",
            "SARSEP",
            "SEP_IRA",
            "SIMPLE_IRA",
            "SINGLE",
            "SOLO_401K",
            "TRUST",
            "TYPE_401K",
            "TYPE_403B",
            "TYPE_457_PLAN",
            "TYPE_529_PLAN",
        ],
        alpacars::broker::enums::AcatsAccountType::Unknown,
    );
    assert_round_trips(
        &["BANK", "BROKER"],
        alpacars::broker::enums::AcatsContraBrokerType::Unknown,
    );
    assert_round_trips(
        &[
            "SS_TAX_ID_MISMATCH",
            "ACCOUNT_TITLE_MISMATCH",
            "DOCUMENTATION_NEEDED",
            "ACCOUNT_FLAT",
            "INVALID_ACCOUNT_NUMBER",
            "DUPLICATE",
            "ACCOUNT_IN_DISTRIBUTION_OR_TRANSFER",
            "CLIENT_RESCINDED",
            "MISSING_AUTHORIZATION_SIGNATURE",
            "ACCOUNT_VIOLATES_CREDIT_POLICY_OF_RECEIVING_FIRM",
            "UNRECOGNIZED_FOR_RESIDUAL_CREDIT_BALANCE",
            "PARTIAL_TRANSFER_DELIVERER_INITIATED_REJECT",
            "FAIL_REVERSAL_REJECT",
            "RECLAIM_REJECT",
            "MUTUAL_FUND_CLEANUP",
            "SYSTEM_REJECTED",
            "INVALID_PARTICIPANT",
            "ASSOCIATED_RECORD_NOT_PRESENT_OR_INVALID",
            "MISSED_CUTOFF",
        ],
        alpacars::broker::enums::AcatsRejectionReason::Unknown,
    );
    assert_round_trips(
        &["LONG", "SHORT", "CREDIT", "DEBIT"],
        alpacars::broker::enums::AcatsPosition::Unknown,
    );
    assert_round_trips(
        &["EQUITY", "CASH", "OPTION", "OTHER"],
        alpacars::broker::enums::AcatsAssetClass::Unknown,
    );
    assert_round_trips(
        &[
            "NOT_VALIDATED",
            "REQUESTED",
            "DELETED",
            "SETTLED",
            "REVERSED",
            "INTERFACE_REJECTED",
        ],
        alpacars::broker::enums::AcatsAssetStatus::Unknown,
    );
    assert_round_trips_strict::<alpacars::broker::enums::AcatsSortDirection>(&["ASC", "DESC"]);
    assert_round_trips_strict::<alpacars::broker::enums::AcatsSortBy>(&[
        "account_number",
        "total_transfer_value",
        "created_at",
        "contra_account_number",
        "contra_broker_number",
        "external_transfer_id",
        "settled_at",
        "status",
        "transfer_type",
        "updated_at",
    ]);
}

#[test]
fn test_funding_enums_match_reference() {
    use alpacars::broker::enums::{BankCodeType, BankStatus, BankStatusFilter, FeePaymentMethod};
    // createrecipientbank / getrecipientbanks.
    assert_round_trips(&["ABA", "BIC"], BankCodeType::Unknown);
    assert_round_trips(
        &[
            "QUEUED",
            "SENT_TO_CLEARING",
            "APPROVED",
            "REJECTED",
            "CANCELED",
        ],
        BankStatus::Unknown,
    );
    assert_round_trips_strict::<BankStatusFilter>(&["ACTIVE", "INACTIVE"]);
    // createtransferforaccount: FeePaymentMethod is "user" or "invoice".
    assert_round_trips(&["user", "invoice"], FeePaymentMethod::Unknown);
}

#[test]
fn test_typed_filter_enums_match_reference() {
    use alpacars::broker::enums::{AccountEntity, RunStatus};
    use alpacars::common::enums::Sort;
    use alpacars::data::enums::MarketType;
    use alpacars::trading::enums::{
        ActivityCategory, AssetAttribute, CalendarDateType, CorporateActionType, IntradayReporting,
        PnlReset,
    };
    // `sort` / `direction` on every list endpoint (getallorders-1, getallaccounts, stockbars, …).
    assert_round_trips_strict::<Sort>(&["asc", "desc"]);
    // getallaccounts `entities`.
    assert_round_trips_strict::<AccountEntity>(&[
        "agreements",
        "contact",
        "disclosures",
        "documents",
        "identity",
        "trading_configurations",
        "trusted_contact",
    ]);
    // getaccountactivities / getaccountactivities-2 `category`.
    assert_round_trips_strict::<ActivityCategory>(&["trade_activity", "non_trade_activity"]);
    // getaccountportfoliohistory-1.
    assert_round_trips_strict::<IntradayReporting>(&[
        "market_hours",
        "extended_hours",
        "continuous",
    ]);
    assert_round_trips_strict::<PnlReset>(&["no_reset", "per_day"]);
    // legacycalendar `date_type`.
    assert_round_trips_strict::<CalendarDateType>(&["TRADING", "SETTLEMENT"]);
    // get-v2-assets-1 `attributes` (also on the Asset response).
    assert_round_trips(
        &[
            "ptp_no_exception",
            "ptp_with_exception",
            "ipo",
            "has_options",
            "options_late_close",
            "fractional_eh_enabled",
            "overnight_tradable",
            "overnight_halted",
        ],
        AssetAttribute::Unknown,
    );
    // get-v1-rebalancing-runs-run_id PortfolioRunStatus.
    assert_round_trips(
        &[
            "QUEUED",
            "IN_PROGRESS",
            "CANCELED",
            "CANCELED_MID_RUN",
            "ERROR",
            "TIMEOUT",
            "COMPLETED_ADJUSTED",
            "COMPLETED_SUCCESS",
        ],
        RunStatus::Unknown,
    );
    // movers-1 `market_type`.
    assert_round_trips(&["stocks", "crypto"], MarketType::Unknown);
    // get-v2-corporate_actions-announcements-1 CorporateActionCaType: capitalized.
    assert_round_trips(
        &["Spinoff", "Merger", "Split", "Reorg", "Dividend"],
        CorporateActionType::Unknown,
    );
}
