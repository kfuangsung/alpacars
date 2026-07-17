use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deserializes the `"c"` conditions field, which stocks send as `["A","B"]`
/// but options send as a bare string `"A"` or `" "`.
fn deserialize_conditions<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: Option<serde_json::Value> = Option::deserialize(d)?;
    Ok(match v {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(s)) => Some(vec![s]),
        Some(serde_json::Value::Array(arr)) => Some(
            arr.into_iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect(),
        ),
        Some(other) => Some(vec![other.to_string()]),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Open price
    #[serde(rename = "o")]
    pub open: f64,
    /// High price
    #[serde(rename = "h")]
    pub high: f64,
    /// Low price
    #[serde(rename = "l")]
    pub low: f64,
    /// Close price
    #[serde(rename = "c")]
    pub close: f64,
    /// Volume
    #[serde(rename = "v")]
    pub volume: f64,
    /// Volume-weighted average price
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
    /// Number of trades
    #[serde(rename = "n")]
    pub trade_count: Option<f64>,
    /// Exchange
    #[serde(rename = "x")]
    pub exchange: Option<String>,
}

/// Map of symbol → list of bars (auto-paginated)
pub type BarSet = HashMap<String, Vec<Bar>>;
/// Map of symbol → latest bar
pub type LatestBarSet = HashMap<String, Bar>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Ask exchange
    #[serde(rename = "ax")]
    pub ask_exchange: Option<String>,
    /// Ask price
    #[serde(rename = "ap")]
    pub ask_price: f64,
    /// Ask size
    #[serde(rename = "as")]
    pub ask_size: f64,
    /// Bid exchange
    #[serde(rename = "bx")]
    pub bid_exchange: Option<String>,
    /// Bid price
    #[serde(rename = "bp")]
    pub bid_price: f64,
    /// Bid size
    #[serde(rename = "bs")]
    pub bid_size: f64,
    /// Conditions — stocks send an array, options send a bare string; accept both.
    #[serde(rename = "c", default, deserialize_with = "deserialize_conditions")]
    pub conditions: Option<Vec<String>>,
    /// Tape
    #[serde(rename = "z")]
    pub tape: Option<String>,
}

/// Map of symbol → list of quotes
pub type QuoteSet = HashMap<String, Vec<Quote>>;
/// Map of symbol → latest quote
pub type LatestQuoteSet = HashMap<String, Quote>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Timestamp
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    /// Exchange
    #[serde(rename = "x")]
    pub exchange: Option<String>,
    /// Price
    #[serde(rename = "p")]
    pub price: f64,
    /// Size
    #[serde(rename = "s")]
    pub size: f64,
    /// Trade ID
    #[serde(rename = "i")]
    pub trade_id: Option<serde_json::Value>,
    /// Conditions — stocks send an array, options send a bare string; accept both.
    #[serde(rename = "c", default, deserialize_with = "deserialize_conditions")]
    pub conditions: Option<Vec<String>>,
    /// Tape
    #[serde(rename = "z")]
    pub tape: Option<String>,
    /// Update (for corrections/cancels)
    #[serde(rename = "u")]
    pub update: Option<String>,
}

/// Map of symbol → list of trades
pub type TradeSet = HashMap<String, Vec<Trade>>;
/// Map of symbol → latest trade
pub type LatestTradeSet = HashMap<String, Trade>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookEntry {
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orderbook {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "a")]
    pub asks: Vec<OrderbookEntry>,
    #[serde(rename = "b")]
    pub bids: Vec<OrderbookEntry>,
}

/// Map of symbol → latest orderbook
pub type LatestOrderbookSet = HashMap<String, Orderbook>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<Trade>,
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<Quote>,
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<Bar>,
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<Bar>,
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<Bar>,
}

/// Map of symbol → snapshot
pub type SnapshotSet = HashMap<String, Snapshot>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsSnapshot {
    pub symbol: Option<String>,
    #[serde(rename = "latestTrade")]
    pub latest_trade: Option<Trade>,
    #[serde(rename = "latestQuote")]
    pub latest_quote: Option<Quote>,
    #[serde(rename = "minuteBar")]
    pub minute_bar: Option<Bar>,
    #[serde(rename = "dailyBar")]
    pub daily_bar: Option<Bar>,
    #[serde(rename = "prevDailyBar")]
    pub prev_daily_bar: Option<Bar>,
    pub greeks: Option<OptionsGreeks>,
    #[serde(rename = "impliedVolatility")]
    pub implied_volatility: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsGreeks {
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub rho: Option<f64>,
    pub theta: Option<f64>,
    pub vega: Option<f64>,
}

/// Map of symbol → options snapshot
pub type OptionsSnapshotSet = HashMap<String, OptionsSnapshot>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingStatus {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "S")]
    pub symbol: String,
    #[serde(rename = "sc")]
    pub status_code: Option<String>,
    #[serde(rename = "sm")]
    pub status_message: Option<String>,
    #[serde(rename = "rc")]
    pub reason_code: Option<String>,
    #[serde(rename = "rm")]
    pub reason_message: Option<String>,
    #[serde(rename = "z")]
    pub tape: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCancel {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "x")]
    pub exchange: Option<String>,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: f64,
    #[serde(rename = "i")]
    pub trade_id: Option<serde_json::Value>,
    #[serde(rename = "z")]
    pub tape: Option<String>,
    #[serde(rename = "S")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCorrection {
    #[serde(rename = "t")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "x")]
    pub exchange: Option<String>,
    #[serde(rename = "oi")]
    pub original_trade_id: Option<serde_json::Value>,
    #[serde(rename = "op")]
    pub original_price: Option<f64>,
    #[serde(rename = "os")]
    pub original_size: Option<f64>,
    #[serde(rename = "oc")]
    pub original_conditions: Option<Vec<String>>,
    #[serde(rename = "ci")]
    pub corrected_trade_id: Option<serde_json::Value>,
    #[serde(rename = "cp")]
    pub corrected_price: Option<f64>,
    #[serde(rename = "cs")]
    pub corrected_size: Option<f64>,
    #[serde(rename = "cc")]
    pub corrected_conditions: Option<Vec<String>>,
    #[serde(rename = "z")]
    pub tape: Option<String>,
    #[serde(rename = "S")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsImage {
    pub size: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct News {
    pub id: u64,
    pub headline: String,
    pub author: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub url: Option<String>,
    pub images: Option<Vec<NewsImage>>,
    pub symbols: Option<Vec<String>>,
    pub source: Option<String>,
}

/// Collection of news articles
pub type NewsSet = Vec<News>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveStock {
    pub symbol: String,
    pub volume: Option<f64>,
    pub trade_count: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostActives {
    pub most_actives: Vec<ActiveStock>,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mover {
    pub symbol: String,
    pub percent_change: Option<f64>,
    pub change: Option<f64>,
    pub price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movers {
    pub gainers: Vec<Mover>,
    pub losers: Vec<Mover>,
    pub market_type: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
}

// Corporate action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwardSplit {
    pub symbol: String,
    pub new_rate: f64,
    pub old_rate: f64,
    pub process_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
    pub due_bill_redemption_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseSplit {
    pub symbol: String,
    pub new_symbol: Option<String>,
    pub new_rate: f64,
    pub old_rate: f64,
    pub process_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitSplit {
    pub old_symbol: String,
    pub old_rate: f64,
    pub new_symbol: String,
    pub new_rate: f64,
    pub alternate_symbol: Option<String>,
    pub alternate_rate: Option<f64>,
    pub process_date: Option<NaiveDate>,
    pub effective_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDividend {
    pub symbol: String,
    pub rate: f64,
    pub special: bool,
    pub foreign: bool,
    pub process_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
    pub due_bill_on_date: Option<NaiveDate>,
    pub due_bill_off_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockDividend {
    pub symbol: String,
    pub rate: f64,
    pub process_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinOff {
    pub source_symbol: String,
    pub source_rate: f64,
    pub new_symbol: String,
    pub new_rate: f64,
    pub process_date: Option<NaiveDate>,
    pub ex_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub payable_date: Option<NaiveDate>,
    pub due_bill_redemption_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateActionsSet {
    pub reverse_splits: Option<Vec<ReverseSplit>>,
    pub forward_splits: Option<Vec<ForwardSplit>>,
    pub unit_splits: Option<Vec<UnitSplit>>,
    pub cash_dividends: Option<Vec<CashDividend>>,
    pub stock_dividends: Option<Vec<StockDividend>>,
    pub spin_offs: Option<Vec<SpinOff>>,
    pub cash_mergers: Option<Vec<serde_json::Value>>,
    pub stock_mergers: Option<Vec<serde_json::Value>>,
    pub stock_and_cash_mergers: Option<Vec<serde_json::Value>>,
    pub redemptions: Option<Vec<serde_json::Value>>,
    pub name_changes: Option<Vec<serde_json::Value>>,
    pub worthless_removals: Option<Vec<serde_json::Value>>,
    pub rights_distributions: Option<Vec<serde_json::Value>>,
    pub next_page_token: Option<String>,
}
