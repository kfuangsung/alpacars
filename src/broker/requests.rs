use crate::broker::enums::*;
use crate::trading::enums::QueryOrderStatus;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccountRequest {
    pub contact: CreateContactRequest,
    pub identity: CreateIdentityRequest,
    pub disclosures: CreateDisclosuresRequest,
    pub agreements: Vec<AgreementRequest>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJournalEntryRequest {
    pub to_account: Uuid,
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchJournalRequest {
    pub from_account: Uuid,
    pub entry_type: JournalEntryType,
    pub entries: Vec<BatchJournalEntryRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseBatchJournalEntry {
    pub to_account: Uuid,
    pub from_account: Uuid,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReverseBatchJournalRequest {
    pub entry_type: JournalEntryType,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePortfolioRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub assets: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooldown_days: Option<u32>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRunRequest {
    pub portfolio_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetRunsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Portfolio run type: `full_rebalance` or `invest_cash`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub run_type: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<u64>,
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
