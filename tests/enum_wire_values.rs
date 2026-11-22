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
        &[
            "binance",
            "bitget",
            "coinbase",
            "ondo",
            "robinhood",
            "st0x",
            "xstocks",
        ],
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

// Enums synced with the references for 0.9.0 (changelog 2026-10-20 → 2026-11-20).

#[test]
fn test_broker_account_enums_match_reference() {
    use alpacars::broker::enums::{AgreementType, TaxIdType};
    // createaccount AgreementType. The crate used to send `custodial_customer_agreement`.
    assert_round_trips(
        &[
            "margin_agreement",
            "account_agreement",
            "customer_agreement",
            "crypto_agreement",
            "options_agreement",
            "custodian_customer_agreement",
            "trusted_contact_consent",
            "master_securities_lending_agreement",
            "certifications_and_resolutions_statement",
            "joint_account_agreement",
            "foreign_markets_addendum",
        ],
        AgreementType::Unknown,
    );
    // createaccount TaxIdType: all 235 values.
    assert_round_trips(TAX_ID_TYPES, TaxIdType::Unknown);
    assert_eq!(TAX_ID_TYPES.len(), 235);
}

const TAX_ID_TYPES: &[&str] = &[
    "USA_SSN",
    "USA_EIN",
    "USA_ITIN",
    "USA_TIN",
    "ABW_PN",
    "AGO_NIF",
    "ALB_NIPT",
    "AND_NRT",
    "ARE_EID",
    "ARE_TRN",
    "ARG_AG_CUIT",
    "ARG_AR_CUIT",
    "ARM_SSN",
    "ARM_TIN",
    "AUS_ABN",
    "AUS_TFN",
    "AUT_UID",
    "AUT_VNR",
    "AZE_FIN",
    "AZE_VOEN",
    "BEL_BCE",
    "BEL_NN",
    "BEN_IFU",
    "BFA_IFU",
    "BGD_TIN",
    "BGR_EGN",
    "BGR_EIK",
    "BHR_CPR",
    "BIH_JMBG",
    "BLR_ID",
    "BLR_UNP",
    "BLZ_TIN",
    "BOL_NIT",
    "BRA_CNPJ",
    "BRA_CPF",
    "BRB_TIN",
    "BTN_TPN",
    "BWA_TIN",
    "CAN_BN",
    "CAN_SIN",
    "CHE_AHV",
    "CHE_UID",
    "CHL_RUT",
    "CHN_RID",
    "CHN_USCC",
    "CIV_NCC",
    "CMR_NIU",
    "COK_RMD",
    "COL_CC",
    "COL_CE",
    "COL_NIT",
    "COL_NUIP",
    "COL_PPT",
    "CPV_NIF",
    "CRI_CJ",
    "CRI_DIMEX",
    "CRI_NITE",
    "CRI_PIN",
    "CUW_CRIB",
    "CYP_TIC",
    "CZE_TIN",
    "DEU_TAX_ID",
    "DEU_UST_ID",
    "DNK_CPR",
    "DNK_CVR",
    "DOM_CED",
    "DOM_RNC",
    "DZA_NIF",
    "ECU_RUC",
    "EGY_NID",
    "EGY_TRN",
    "ESP_DNI",
    "ESP_NIE",
    "ESP_NIF",
    "EST_IK",
    "EST_RC",
    "ETH_TIN",
    "FIN_HETU",
    "FIN_YTU",
    "FJI_TIN",
    "FRA_SIREN",
    "FRA_SPI",
    "GAB_NIF",
    "GBR_CRN",
    "GBR_NINO",
    "GBR_UTR",
    "GEO_PID",
    "GEO_TIN",
    "GHA_TIN",
    "GRC_AFM",
    "GTM_NIT",
    "HKG_BRN",
    "HKG_HKID",
    "HND_RTN",
    "HRV_OIB",
    "HTI_NIF",
    "HUN_ENT",
    "HUN_TIN",
    "IDN_KTP",
    "IDN_NWPW",
    "IND_GSTIN",
    "IND_PAN",
    "IRL_CRO",
    "IRL_PPS",
    "IRN_NID",
    "ISL_KT",
    "ISR_CORP",
    "ISR_TAX_ID",
    "ITA_PIVA",
    "ITA_TAX_ID",
    "JAM_TRN",
    "JOR_NID",
    "JPN_CORP",
    "JPN_TAX_ID",
    "KAZ_BIN",
    "KAZ_IIN",
    "KEN_KRA_PIN",
    "KGZ_INN",
    "KHM_NID",
    "KOR_BRN",
    "KOR_RRN",
    "KWT_CID",
    "LBN_TIN",
    "LKA_NIC",
    "LKA_TIN",
    "LTU_AK",
    "LTU_JAK",
    "LUX_NIN",
    "LUX_TVA",
    "LVA_PK",
    "LVA_REG",
    "MAC_TIN",
    "MAR_CIN",
    "MAR_ICE",
    "MDA_IDNO",
    "MDA_IDNP",
    "MEX_CURP",
    "MEX_RFC",
    "MKD_EDB",
    "MKD_EMBG",
    "MLI_NIF",
    "MLT_ID",
    "MLT_VAT",
    "MNE_JMBG",
    "MNE_PIB",
    "MNG_RD",
    "MOZ_NUIT",
    "MUS_NID",
    "MUS_TAN_ENT",
    "MUS_TAN_IND",
    "MWI_TPIN",
    "MYS_MYKAD",
    "MYS_ROC",
    "NAM_TIN",
    "NGA_NIN",
    "NGA_TIN",
    "NIC_RUC",
    "NLD_BTW",
    "NLD_KVK",
    "NLD_OBN",
    "NLD_TIN",
    "NOR_FNR",
    "NOR_ORG",
    "NPL_PAN",
    "NZL_IRD",
    "NZL_NZBN",
    "OMN_CID",
    "PAK_NIC",
    "PAK_NTN",
    "PAN_RUC",
    "PER_RUC",
    "PHL_TIN",
    "POL_NIP",
    "POL_PESEL",
    "PRT_NIF",
    "PRY_RUC",
    "QAT_QID",
    "ROU_CNP",
    "ROU_CUI",
    "RUS_INN_ENT",
    "RUS_INN_IND",
    "RWA_TIN",
    "SAU_TIN",
    "SAU_VAT",
    "SEN_NINEA",
    "SGP_ASGD",
    "SGP_FIN",
    "SGP_ITR",
    "SGP_NRIC",
    "SGP_UEN",
    "SLV_NIT",
    "SRB_JMBG",
    "SRB_PIB",
    "SVK_ICO",
    "SVK_TIN",
    "SVN_DAS",
    "SVN_MS",
    "SWE_ORG",
    "SWE_TAX_ID",
    "SWZ_TIN",
    "SXM_CRIB",
    "SYR_TIN",
    "THA_NID",
    "THA_TIN",
    "TJK_TIN",
    "TTO_BIR",
    "TUN_CIN",
    "TUN_MF",
    "TUR_TIN",
    "TWN_GUI",
    "TWN_NID",
    "TZA_TIN",
    "UGA_TIN",
    "UKR_EDRPOU",
    "UKR_RNTRC",
    "URY_CI",
    "URY_RUT",
    "UZB_INN",
    "UZB_PINFL",
    "VEN_RIF",
    "VNM_MST_BRN",
    "VNM_MST_CORP",
    "VNM_MST_PIN",
    "WSM_TIN",
    "XXK_PID",
    "ZAF_CRN",
    "ZAF_ID",
    "ZMB_TPIN",
    "ZWE_TIN",
    "NATIONAL_ID",
    "PASSPORT",
    "PERMANENT_RESIDENT",
    "DRIVER_LICENSE",
    "OTHER_GOV_ID",
    "NOT_SPECIFIED",
];

#[test]
fn test_rebalancing_enums_match_reference() {
    use alpacars::broker::enums::{RunInitiatedFrom, RunType, TradeMarker};
    // get-v1-rebalancing-runs PortfolioRunType / PortfolioRunInitiator.
    assert_round_trips(
        &["full_rebalance", "partial_liquidation", "invest_cash"],
        RunType::Unknown,
    );
    assert_round_trips(&["system", "api", "dashboard"], RunInitiatedFrom::Unknown);
    // post-v1-rebalancing-subscriptions TradeMarker.
    assert_round_trips(&["discretionary", "unsolicited"], TradeMarker::Unknown);
}

#[test]
fn test_activity_enums_match_reference() {
    use alpacars::trading::enums::{ActivityType, TradeActivitySide};
    // getaccountactivities-2: the union of ActivityType (returned) and ActivityTypeFilter
    // (accepted), since the crate uses one enum for both.
    assert_round_trips(
        &[
            "FILL", "ACATC", "ACATS", "ALLOC", "CFEE", "CGD", "CIL", "CORR", "CSD", "CSR", "CSW",
            "DIV", "DIVCGL", "DIVCGS", "DIVFEE", "DIVFT", "DIVNRA", "DIVROC", "DIVTW", "DIVTXEX",
            "DIVWH", "DTCS", "DVF", "ECEXP", "EXTRD", "FEE", "FIMAT", "FXCNV", "FXTRD", "INT",
            "INTNRA", "INTTW", "INVPNL", "JNL", "JNLC", "JNLS", "MA", "MEM", "NC", "OPASN", "OPCA",
            "OPCSH", "OPEXC", "OPEXP", "OPTRD", "PTC", "PTR", "REO", "REORG", "SC", "SEG", "SPIN",
            "SPLIT", "SSO", "SSP", "SUB", "VOF", "WH", "FOPT", "OCT", "SWP", "TRD", "MISC",
            "TRANS",
        ],
        ActivityType::Unknown,
    );
    // getaccountactivities TradeActivitySide.
    assert_round_trips(&["buy", "sell", "sell_short"], TradeActivitySide::Unknown);
}

#[test]
fn test_order_and_asset_enums_match_reference() {
    use alpacars::data::enums::Adjustment;
    use alpacars::trading::enums::{
        AdvancedAlgorithm, AssetExchange, CommissionType, PortfolioHistoryTimeframe,
    };
    // getallopenpositions ExchangeForPosition (Exchange plus the empty string).
    assert_round_trips(
        &[
            "AMEX", "ARCA", "BATS", "NYSE", "NASDAQ", "NYSEARCA", "OTC", "CRYPTO", "TXSE", "ERSX",
            "CRXL", "ASCX", "INDEX", "",
        ],
        AssetExchange::Unknown,
    );
    // createorderforaccount CommissionType.
    assert_round_trips(&["notional", "qty", "bps"], CommissionType::Unknown);
    // postorder advanced_instructions.algorithm.
    assert_round_trips_strict::<AdvancedAlgorithm>(&["DMA", "TWAP", "VWAP"]);
    // getaccountportfoliohistory-1 PortfolioHistoryTimeframe.
    assert_round_trips_strict::<PortfolioHistoryTimeframe>(&["1Min", "5Min", "15Min", "1H", "1D"]);
    // stockbars stock_adjustment.
    assert_round_trips_strict::<Adjustment>(&["raw", "split", "dividend", "spin-off", "all"]);
}

#[test]
fn test_tokenization_transfer_status_matches_reference() {
    use alpacars::trading::enums::TokenizationTransferStatus;
    // gettokenizationrequest TokenizationTransferStatus.
    assert_round_trips(
        &["pending", "completed", "rejected"],
        TokenizationTransferStatus::Unknown,
    );
}

#[test]
fn test_trade_event_matches_websocket_doc() {
    use alpacars::trading::enums::TradeEvent;
    // The trade_updates events listed in the "Websocket Streaming" doc.
    assert_round_trips(
        &[
            "new",
            "fill",
            "partial_fill",
            "canceled",
            "expired",
            "done_for_day",
            "replaced",
            "accepted",
            "rejected",
            "pending_new",
            "stopped",
            "pending_cancel",
            "pending_replace",
            "calculated",
            "suspended",
            "order_replace_rejected",
            "order_cancel_rejected",
        ],
        TradeEvent::Unknown,
    );
}
