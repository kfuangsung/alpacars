//! Every value an API reference lists for an enum must map to a real variant (not `Unknown`)
//! and serialize back to the same string. The lists are copied from docs.alpaca.markets/us/reference,
//! not derived from the crate.

use alpacars::broker::enums::{AccountType, JournalStatus};
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
