use crate::broker::enums::*;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plaid_processor_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBankRequest {
    pub name: String,
    pub routing_number: String,
    pub account_number: String,
    pub bank_code: Option<String>,
    pub bank_code_type: Option<String>,
    pub country: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferRequest {
    pub transfer_type: TransferType,
    pub relationship_id: Option<Uuid>,
    pub bank_id: Option<Uuid>,
    pub amount: String,
    pub direction: TransferDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_information: Option<String>,
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
    ClosePositionRequest, CreateWatchlistRequest, GetAssetsRequest, GetCalendarRequest,
    OrderRequest, ReplaceOrderRequest, UpdateWatchlistRequest,
};

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
