use crate::trading::enums::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    #[serde(rename = "class")]
    pub asset_class: AssetClass,
    pub exchange: AssetExchange,
    pub symbol: String,
    pub name: Option<String>,
    pub status: AssetStatus,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    /// Deprecated by Alpaca; removed from the API on 2026-09-22. Use `borrow_status`.
    pub easy_to_borrow: Option<bool>,
    pub borrow_status: Option<BorrowStatus>,
    pub fractionable: bool,
    pub min_order_size: Option<f64>,
    pub min_trade_increment: Option<f64>,
    pub price_increment: Option<f64>,
    pub maintenance_margin_requirement: Option<f64>,
    pub attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsdPositionValues {
    pub avg_entry_price: String,
    pub market_value: String,
    pub cost_basis: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub unrealized_intraday_pl: String,
    pub unrealized_intraday_plpc: String,
    pub current_price: String,
    pub lastday_price: String,
    pub change_today: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub asset_id: Uuid,
    pub symbol: String,
    pub exchange: AssetExchange,
    pub asset_class: AssetClass,
    pub asset_marginable: Option<bool>,
    pub avg_entry_price: String,
    pub qty: String,
    pub side: PositionSide,
    pub market_value: Option<String>,
    pub cost_basis: String,
    pub unrealized_pl: Option<String>,
    pub unrealized_plpc: Option<String>,
    pub unrealized_intraday_pl: Option<String>,
    pub unrealized_intraday_plpc: Option<String>,
    pub current_price: Option<String>,
    pub lastday_price: Option<String>,
    pub change_today: Option<String>,
    pub swap_rate: Option<String>,
    pub avg_entry_swap_rate: Option<String>,
    pub prev_swap_rate: Option<String>,
    pub usd: Option<UsdPositionValues>,
    pub qty_available: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeProfitSpec {
    pub limit_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopLossSpec {
    pub stop_price: String,
    pub limit_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub client_order_id: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaced_by: Option<Uuid>,
    pub replaces: Option<Uuid>,
    pub asset_id: Option<Uuid>,
    pub symbol: Option<String>,
    pub asset_class: Option<AssetClass>,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub ratio_qty: Option<String>,
    pub filled_qty: Option<String>,
    pub filled_avg_price: Option<String>,
    pub order_class: OrderClass,
    pub order_type: Option<OrderType>,
    #[serde(rename = "type")]
    pub order_type_v2: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub time_in_force: TimeInForce,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: OrderStatus,
    pub extended_hours: bool,
    pub legs: Option<Vec<Order>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
    pub subtag: Option<String>,
    pub source: Option<String>,
    pub position_intent: Option<PositionIntent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosePositionResponse {
    pub order_id: Option<Uuid>,
    pub status: Option<u16>,
    pub symbol: Option<String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    pub id: Uuid,
    pub status: u16,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioHistory {
    pub timestamp: Vec<i64>,
    pub equity: Vec<Option<f64>>,
    pub profit_loss: Vec<Option<f64>>,
    pub profit_loss_pct: Vec<Option<f64>>,
    pub base_value: Option<f64>,
    pub base_value_asof: Option<String>,
    pub timeframe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistAsset {
    pub id: Uuid,
    pub symbol: String,
    pub name: Option<String>,
    #[serde(rename = "class")]
    pub asset_class: Option<AssetClass>,
    pub exchange: Option<AssetExchange>,
    pub status: Option<AssetStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watchlist {
    pub id: Uuid,
    pub name: String,
    pub account_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub assets: Option<Vec<WatchlistAsset>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub date: NaiveDate,
    pub open: String,
    pub close: String,
    pub session_open: Option<String>,
    pub session_close: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeAccount {
    pub id: Uuid,
    pub account_number: String,
    pub status: AccountStatus,
    pub crypto_status: Option<AccountStatus>,
    pub currency: Option<String>,
    pub buying_power: Option<String>,
    pub regt_buying_power: Option<String>,
    pub effective_buying_power: Option<String>,
    pub non_marginable_buying_power: Option<String>,
    pub options_buying_power: Option<String>,
    pub cash: Option<String>,
    pub accrued_fees: Option<String>,
    pub pending_transfer_in: Option<String>,
    pub portfolio_value: Option<String>,
    pub trading_blocked: Option<bool>,
    pub transfers_blocked: Option<bool>,
    pub account_blocked: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub trade_suspended_by_user: Option<bool>,
    pub multiplier: Option<String>,
    pub shorting_enabled: Option<bool>,
    pub equity: Option<String>,
    pub last_equity: Option<String>,
    pub long_market_value: Option<String>,
    pub short_market_value: Option<String>,
    pub initial_margin: Option<String>,
    pub maintenance_margin: Option<String>,
    pub last_maintenance_margin: Option<String>,
    pub sma: Option<String>,
    pub options_approved_level: Option<i32>,
    pub options_trading_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfiguration {
    pub trade_confirm_email: Option<TradeConfirmationEmail>,
    pub suspend_trade: Option<bool>,
    pub no_shorting: Option<bool>,
    pub fractional_trading: Option<bool>,
    pub max_margin_multiplier: Option<String>,
    pub max_options_trading_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeActivity {
    pub id: String,
    pub activity_type: ActivityType,
    pub transaction_time: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,
    pub symbol: Option<String>,
    pub side: Option<OrderSide>,
    pub qty: Option<String>,
    pub price: Option<String>,
    pub cum_qty: Option<String>,
    pub leaves_qty: Option<String>,
    pub order_status: Option<OrderStatus>,
    #[serde(rename = "type")]
    pub trade_type: Option<TradeActivityType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonTradeActivity {
    pub id: String,
    pub activity_type: ActivityType,
    pub date: Option<NaiveDate>,
    pub net_amount: Option<String>,
    pub symbol: Option<String>,
    pub qty: Option<String>,
    pub per_share_amount: Option<String>,
    pub description: Option<String>,
    pub status: Option<NonTradeActivityStatus>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountActivity {
    Trade(TradeActivity),
    NonTrade(NonTradeActivity),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeUpdate {
    pub event: TradeEvent,
    pub order: Order,
    pub timestamp: Option<DateTime<Utc>>,
    pub position_qty: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateActionAnnouncement {
    pub id: Uuid,
    pub corporate_action_id: String,
    pub ca_type: CorporateActionType,
    pub ca_sub_type: CorporateActionSubType,
    pub initiating_symbol: String,
    pub initiating_original_cusip: Option<String>,
    pub target_symbol: Option<String>,
    pub target_original_cusip: Option<String>,
    pub declaration_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub effective_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
    pub cash: Option<String>,
    pub old_rate: Option<String>,
    pub new_rate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    pub id: Uuid,
    pub symbol: String,
    pub name: Option<String>,
    pub status: Option<AssetStatus>,
    pub tradable: Option<bool>,
    pub expiration_date: Option<NaiveDate>,
    pub root_symbol: Option<String>,
    pub underlying_symbol: Option<String>,
    pub underlying_asset_id: Option<Uuid>,
    #[serde(rename = "type")]
    pub contract_type: Option<ContractType>,
    pub style: Option<ExerciseStyle>,
    pub strike_price: Option<String>,
    pub multiplier: Option<String>,
    pub size: Option<String>,
    pub open_interest: Option<String>,
    pub open_interest_date: Option<NaiveDate>,
    pub close_price: Option<String>,
    pub close_price_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContractsResponse {
    pub option_contracts: Vec<OptionContract>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locate {
    pub id: String,
    pub symbol: String,
    pub status: LocateStatus,
    pub requested_qty: i64,
    pub located_qty: Option<i64>,
    pub located_price: Option<String>,
    pub total_fee: Option<String>,
    pub limit_price: Option<String>,
    pub all_or_none: bool,
    pub rejection_reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocateQuote {
    pub symbol: String,
    pub available_qty: i64,
    pub price: Option<String>,
    pub quoted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocateQuoteError {
    pub symbol: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLocatesResponse {
    pub locates: Vec<Locate>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLocateQuotesResponse {
    pub quotes: Vec<LocateQuote>,
    pub errors: Option<Vec<LocateQuoteError>>,
}

/// A request to mint or redeem a tokenized asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationRequest {
    /// Unique identifier of the tokenization request, assigned by Alpaca.
    pub tokenization_request_id: String,
    pub status: TokenizationRequestStatus,
    #[serde(rename = "type")]
    pub request_type: TokenizationRequestType,
    pub underlying_symbol: String,
    pub token_symbol: String,
    pub qty: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub issuer: TokenizationIssuer,
    pub network: TokenizationNetwork,
    /// Authorized Participant-supplied label for this request.
    pub client_request_id: Option<String>,
    /// Alpaca account UUID of the Authorized Participant.
    pub client_account_id: Option<Uuid>,
    /// Issuer-side account identifier of the Authorized Participant.
    pub client_external_account_id: Option<String>,
    /// Unique identifier of the request set by the issuer.
    pub issuer_request_id: Option<String>,
    pub fees: Option<String>,
    /// Transaction hash of the completed request on the blockchain.
    pub tx_hash: Option<String>,
    /// Deprecated by Alpaca; removed from the API on 2026-10-15. Use `client_account_id`.
    pub account: Option<String>,
    /// Deprecated by Alpaca; removed from the API on 2026-10-15. Use `client_external_account_id`.
    pub issuer_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationMintResponse {
    pub tokenization_request_id: String,
    pub status: TokenizationRequestStatus,
    pub underlying_symbol: String,
    pub token_symbol: String,
    pub qty: String,
    pub created_at: DateTime<Utc>,
    pub issuer: TokenizationIssuer,
    pub network: TokenizationNetwork,
}
