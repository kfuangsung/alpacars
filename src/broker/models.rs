use crate::broker::enums::*;
use crate::trading::enums::AccountStatus;
use crate::trading::models::Order;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub email_address: String,
    pub phone_number: Option<String>,
    pub street_address: Vec<String>,
    pub unit: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub given_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub tax_id: Option<String>,
    pub tax_id_type: Option<TaxIdType>,
    pub country_of_citizenship: Option<String>,
    pub country_of_birth: Option<String>,
    pub country_of_tax_residence: Option<String>,
    pub visa_type: Option<VisaType>,
    pub visa_expiration_date: Option<NaiveDate>,
    pub date_of_departure_from_usa: Option<NaiveDate>,
    pub permanent_resident: Option<bool>,
    pub funding_source: Option<Vec<FundingSource>>,
    pub annual_income_min: Option<String>,
    pub annual_income_max: Option<String>,
    pub liquid_net_worth_min: Option<String>,
    pub liquid_net_worth_max: Option<String>,
    pub total_net_worth_min: Option<String>,
    pub total_net_worth_max: Option<String>,
    pub employer_name: Option<String>,
    pub employer_address: Option<String>,
    pub employment_position: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disclosures {
    pub is_control_person: bool,
    pub is_affiliated_exchange_or_finra: bool,
    pub is_politically_exposed: bool,
    pub immediate_family_exposed: bool,
    pub is_affiliated_exchange_or_iiroc: Option<bool>,
    pub context: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agreement {
    pub agreement: AgreementType,
    pub signed_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedContact {
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub street_address: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDocument {
    pub document_type: Option<DocumentType>,
    pub document_sub_type: Option<String>,
    pub content: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub account_number: Option<String>,
    pub status: AccountStatus,
    pub crypto_status: Option<AccountStatus>,
    pub kyc_results: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub last_equity: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub contact: Option<Contact>,
    pub identity: Option<Identity>,
    pub disclosures: Option<Disclosures>,
    pub agreements: Option<Vec<Agreement>>,
    pub documents: Option<Vec<AccountDocument>>,
    pub trusted_contact: Option<TrustedContact>,
    pub enabled_assets: Option<Vec<String>>,
    pub account_type: Option<AccountType>,
    pub trading_configurations: Option<serde_json::Value>,
    pub allow_instant_ach: Option<bool>,
    pub instant_ach_blocked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDocument {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub document_type: Option<TradeDocumentType>,
    pub document_sub_type: Option<String>,
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACHRelationship {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: ACHRelationshipStatus,
    pub account_owner_name: Option<String>,
    pub bank_account_type: Option<BankAccountType>,
    pub bank_account_number: Option<String>,
    pub bank_routing_number: Option<String>,
    pub nickname: Option<String>,
    pub processor_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
    pub country: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub street_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub relationship_id: Option<Uuid>,
    pub bank_id: Option<Uuid>,
    pub r#type: Option<TransferType>,
    pub status: Option<TransferStatus>,
    pub amount: Option<String>,
    pub direction: Option<TransferDirection>,
    pub reason: Option<String>,
    pub hold_until: Option<DateTime<Utc>>,
    pub additional_information: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    pub id: Uuid,
    pub to_account: Option<Uuid>,
    pub from_account: Option<Uuid>,
    pub entry_type: Option<JournalEntryType>,
    pub symbol: Option<String>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub status: Option<JournalStatus>,
    pub settle_date: Option<NaiveDate>,
    pub system_date: Option<NaiveDate>,
    pub net_amount: Option<String>,
    pub description: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJournalEntry {
    pub to_account: Uuid,
    pub amount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJournalResponse {
    pub journals: Vec<Journal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioAsset {
    pub symbol: String,
    pub percent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<PortfolioStatus>,
    pub cooldown_days: Option<u32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub assets: Option<Vec<PortfolioAsset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub portfolio_id: Option<Uuid>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalancingRun {
    pub id: Uuid,
    pub portfolio_id: Option<Uuid>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub orders: Option<Vec<Order>>,
    pub failed_orders: Option<Vec<serde_json::Value>>,
    pub skipped_orders: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRunsResponse {
    pub runs: Vec<RebalancingRun>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIPInfo {
    pub id: Uuid,
    pub account_id: Option<Uuid>,
    pub provider_name: Option<Vec<String>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub kyc: Option<serde_json::Value>,
    pub document: Option<serde_json::Value>,
    pub photo: Option<serde_json::Value>,
    pub identity: Option<serde_json::Value>,
    pub watchlist: Option<serde_json::Value>,
}

pub use crate::trading::models::{
    AccountActivity, Asset, Calendar, Clock, ClosePositionResponse, NonTradeActivity,
    OptionContract, Position, TokenizationMintResponse, TokenizationRequest, TradeAccount,
    TradeActivity, Watchlist,
};
pub use crate::trading::requests::{
    CreateWatchlistRequest, GetAssetsRequest, GetCalendarRequest, GetTokenizationRequestsRequest,
    TokenizationMintRequest, UpdateWatchlistRequest,
};
