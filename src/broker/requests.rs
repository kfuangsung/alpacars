use crate::broker::enums::*;
use crate::broker::models::BankExtraFields;
use crate::trading::enums::{ActivityType, QueryOrderStatus};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub street_address: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIdentityRequest {
    pub given_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    pub family_name: String,
    pub date_of_birth: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<TaxIdType>,
    pub country_of_citizenship: Option<String>,
    pub country_of_birth: Option<String>,
    pub country_of_tax_residence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_type: Option<VisaType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_expiration_date: Option<NaiveDate>,
    pub funding_source: Vec<FundingSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDisclosuresRequest {
    pub is_control_person: bool,
    pub is_affiliated_exchange_or_finra: bool,
    pub is_politically_exposed: bool,
    pub immediate_family_exposed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgreementRequest {
    pub agreement: AgreementType,
    pub signed_at: DateTime<Utc>,
    pub ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrustedContactRequest {
    pub given_name: String,
    pub family_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

/// Request body for `POST /v1/accounts`.
///
/// Which fields are required depends on `account_type` (default `trading`): trading accounts
/// need `contact` and `identity` (or `primary_account_holder_id` for an existing holder),
/// custodial accounts also need `minor_identity`, IRA accounts need `account_sub_type`, and
/// donor-advised accounts need `entity_id`. The server enforces these, so every one of them
/// is optional here.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Required for IRA accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sub_type: Option<AccountSubType>,
    /// Open the account for an existing holder instead of supplying `contact` / `identity`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_account_holder_id: Option<Uuid>,
    /// Required for donor-advised accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// The minor's identity, required for custodial accounts. Kept as raw JSON; see the
    /// Create an Account reference for its shape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_identity: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<CreateContactRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<CreateIdentityRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<CreateDisclosuresRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreements: Option<Vec<AgreementRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<CreateTrustedContactRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_assets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclosures: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_contact: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_assets: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListAccountsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountActivitiesRequest {
    /// Restrict to one account. Omit to list activities across all accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    /// Sent as a comma-separated `activity_types` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<Vec<ActivityType>>,
    /// `trade_activity` or `non_trade_activity`. Mutually exclusive with `activity_types`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Only return activities (such as fills) belonging to this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadDocumentRequest {
    pub document_type: DocumentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_sub_type: Option<String>,
    pub content: String,
    pub mime_type: UploadDocumentMimeType,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTradeDocumentsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateACHRelationshipRequest {
    pub account_owner_name: String,
    pub bank_account_type: BankAccountType,
    pub bank_account_number: String,
    pub bank_routing_number: String,
    /// A Plaid processor token, if you use Plaid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

/// Filter for `GET /v1/accounts/{account_id}/ach_relationships`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetACHRelationshipsRequest {
    /// Sent as a comma-separated `statuses` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ACHRelationshipStatus>>,
}

/// Filter for `GET /v1/accounts/{account_id}/recipient_banks`.
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetBanksRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BankStatusFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
}

/// Create a recipient bank (`POST /v1/accounts/{account_id}/recipient_banks`). The bank is
/// identified by `bank_code`: an ABA routing number or a BIC, per `bank_code_type`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBankRequest {
    pub name: String,
    pub bank_code: String,
    pub bank_code_type: BankCodeType,
    pub account_number: String,
    /// Intermediary banks, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_fields: Option<BankExtraFields>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    pub transfer_type: TransferType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<Uuid>,
    pub amount: String,
    pub direction: TransferDirection,
    /// Wire transfers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
    /// Who pays the fees of an outgoing wire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_payment_method: Option<FeePaymentMethod>,
    /// IRA accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ira: Option<TransferIra>,
}

/// IRA distribution details of a new transfer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferIra {
    /// e.g. `normal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_reason: Option<String>,
    /// e.g. `2024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_withholding: Option<TransferIraTaxWithholding>,
}

/// Withholding percentages, e.g. `"10.25"`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransferIraTaxWithholding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fed_pct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_pct: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTransfersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TransferDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalRequest {
    pub from_account: Uuid,
    pub to_account: Uuid,
    pub entry_type: JournalEntryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_financial_institution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_timestamp: Option<String>,
}

/// One recipient of a one-to-many batch journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJournalEntryRequest {
    pub to_account: Uuid,
    /// Cash amount. The API requires the key but allows `null`, so it is always sent.
    pub amount: Option<String>,
    /// ISO currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// For security journals, together with `qty`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_financial_institution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitter_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchJournalRequest {
    pub from_account: Uuid,
    pub entry_type: JournalEntryType,
    pub entries: Vec<BatchJournalEntryRequest>,
    /// The originating correspondent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondent: Option<String>,
}

/// One sender of a many-to-one reverse batch journal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseBatchJournalEntry {
    pub from_account: Uuid,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Many-to-one: every entry journals into the single `to_account`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReverseBatchJournalRequest {
    pub entry_type: JournalEntryType,
    pub to_account: Uuid,
    pub entries: Vec<ReverseBatchJournalEntry>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetJournalsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JournalStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<JournalEntryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_account: Option<Uuid>,
}

/// One allocation in a create-portfolio or create-run request. Percentages across all
/// weights must be positive and total exactly 100.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioWeightRequest {
    #[serde(rename = "type")]
    pub weight_type: WeightType,
    /// Required for an asset weight; omit for cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    pub percent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub weights: Vec<PortfolioWeightRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_days: Option<u32>,
    /// Note the request spells this `rebalance_conditions`, while `Portfolio` responses use
    /// `rebalancing_conditions` — both per the API reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebalance_conditions: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPortfoliosRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PortfolioStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub account_id: Uuid,
    pub portfolio_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetSubscriptionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portfolio_id: Option<Uuid>,
}

/// Omit `weights` (or pass an empty list) for a run driven by the account's active
/// subscription; pass weights for a manual one-off run, which the API rejects if the account
/// already has an active subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunRequest {
    pub account_id: Uuid,
    #[serde(rename = "type")]
    pub run_type: RunType,
    /// Decimal amount used by the run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<PortfolioWeightRequest>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetRunsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<RunType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetEventsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    /// Deprecated by Alpaca, removed on 2027-02-15; use `since_ulid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<u64>,
    /// Deprecated by Alpaca, removed on 2027-02-15; use `until_ulid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<u64>,
    /// Mutually exclusive with `since` and `since_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_ulid: Option<String>,
    /// Requires `since_ulid`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_ulid: Option<String>,
}

// Broker-scoped order/position request types
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrdersForAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
}

pub use crate::trading::requests::{
    ClosePositionRequest, GetAssetsRequest, GetCalendarRequest, OrderRequest, ReplaceOrderRequest,
};

/// Body of `POST /v1/trading/accounts/{account_id}/watchlists`. Unlike the Trading API's
/// [`crate::trading::requests::CreateWatchlistRequest`], the Broker API requires `symbols`
/// (it may be empty).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    pub symbols: Vec<String>,
}

/// Body of `PUT /v1/trading/accounts/{account_id}/watchlists/{watchlist_id}`, which replaces
/// the watchlist: the Broker API requires both fields, even when only one changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWatchlistRequest {
    pub name: String,
    pub symbols: Vec<String>,
}

/// Sign a party-level agreement after the party was created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartyAgreementRequest {
    pub party_id: Uuid,
    pub agreement: PartyAgreementType,
    pub signed_at: DateTime<Utc>,
    pub ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// For a legal-entity party: the natural person signing on its behalf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_party_id: Option<Uuid>,
}

/// Sign an agreement for one of a party's accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountAgreementRequest {
    pub party_id: Uuid,
    pub account_id: Uuid,
    pub agreement: PerAccountAgreementType,
    pub signed_at: DateTime<Utc>,
    pub ip_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// For a legal-entity party: the natural person signing on its behalf.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_party_id: Option<Uuid>,
}

/// Body of `POST /v1/agreements`: a party-level or a per-account agreement.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CreateAgreementRequest {
    Party(CreatePartyAgreementRequest),
    Account(CreateAccountAgreementRequest),
}

// ── ACATS (Beta) ─────────────────────────────────────────────────────────────

/// Initiate an incoming ACATS transfer (`POST /v1beta1/acats/{account_id}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcatsInitiateTransferRequest {
    pub contra_account_details: AcatsContraAccountDetails,
    pub transfer_details: AcatsTransferDetails,
    /// IP address of the customer who authorized the transfer.
    pub ip_address: String,
    /// When the customer authorized the transfer.
    pub signed_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_account_number: Option<bool>,
}

/// The account at the delivering (contra) firm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcatsContraAccountDetails {
    pub account_number: String,
    /// DTCC participant number of the delivering firm (see `list_acats_contra_brokers`).
    pub broker_number: String,
    pub customer_name: String,
    pub customer_tax_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
}

/// What to transfer. `Full` moves the entire account and must not list assets; `Partial`
/// needs at least one non-empty `cash` or `securities` list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum AcatsTransferDetails {
    Full,
    Partial {
        #[serde(skip_serializing_if = "Option::is_none")]
        cash: Option<Vec<AcatsCashBalance>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        securities: Option<AcatsSecurities>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcatsCashBalance {
    /// Only `USD` is supported.
    pub currency: String,
    /// Decimal string. Omit when `transfer_all` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcatsSecurities {
    pub equities: Vec<AcatsEquity>,
}

/// An equity in a partial transfer, identified by exactly one of CUSIP or symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcatsEquity {
    #[serde(flatten)]
    pub security: AcatsSecurityId,
    /// Only `Long` and `Short` are accepted here.
    pub position: AcatsPosition,
    /// Decimal string. Omit when `transfer_all` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_all: Option<bool>,
}

/// How an equity is identified. Prefer `Cusip`: it is stable and unambiguous, whereas ticker
/// symbols can change or be reused.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AcatsSecurityId {
    Cusip(String),
    Symbol(String),
}

/// Filters for listing ACATS transfers across all accounts.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListAcatsTransfersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<AcatsDtccTransferType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_direction: Option<AcatsTransferDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<AcatsTransferStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtcc_status: Option<AcatsDtccStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contra_broker_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AcatsSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<AcatsSortDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// `page_token_next` / `page_token_previous` from a previous response, unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// Paging for listing one account's ACATS transfers.
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListAcatsTransfersForAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// Body of `POST /v1/trading/accounts/{account_id}/orders/estimation`: estimate how a notional
/// order would fill without placing it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateOrderRequest {
    pub symbol: String,
    pub side: crate::trading::enums::OrderSide,
    #[serde(rename = "type")]
    pub order_type: crate::trading::enums::OrderType,
    pub time_in_force: crate::trading::enums::TimeInForce,
    /// Dollar amount to estimate for.
    pub notional: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_fee_bps: Option<u32>,
}
