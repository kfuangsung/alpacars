use crate::common::client::{base_url, RestClient, DATA_V2_MAX_LIMIT};
use crate::data::enums::{Adjustment, DataFeed, TimeFrame};
use crate::data::models::*;
use crate::error::AlpacaError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Default)]
struct BarsParams {
    pub symbols: String,
    pub timeframe: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment: Option<Adjustment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asof: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct QuotesParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct TradesParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct LatestParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct SnapshotParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<DataFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

// ── Request types ─────────────────────────────────────────────────────────────

/// Request parameters for fetching stock bar data.
#[derive(Debug, Clone, Default)]
pub struct StockBarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: Option<TimeFrame>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub currency: Option<String>,
    pub sort: Option<String>,
    pub asof: Option<String>,
}

/// Request parameters for fetching stock quote data.
#[derive(Debug, Clone, Default)]
pub struct StockQuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub currency: Option<String>,
    pub sort: Option<String>,
}

/// Request parameters for fetching stock trade data.
#[derive(Debug, Clone, Default)]
pub struct StockTradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub currency: Option<String>,
    pub sort: Option<String>,
}

/// Request parameters for fetching latest stock data.
#[derive(Debug, Clone, Default)]
pub struct StockLatestRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<String>,
}

/// Request parameters for stock snapshots.
#[derive(Debug, Clone, Default)]
pub struct StockSnapshotRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<String>,
}

// ── Pagination response types ─────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct PagedBars {
    bars: Option<HashMap<String, Vec<Bar>>>,
    next_page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct PagedQuotes {
    quotes: Option<HashMap<String, Vec<Quote>>>,
    next_page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct PagedTrades {
    trades: Option<HashMap<String, Vec<Trade>>>,
    next_page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct LatestBarsResp {
    bars: Option<HashMap<String, Bar>>,
}

#[derive(serde::Deserialize)]
struct LatestQuotesResp {
    quotes: Option<HashMap<String, Quote>>,
}

#[derive(serde::Deserialize)]
struct LatestTradesResp {
    trades: Option<HashMap<String, Trade>>,
}

#[derive(serde::Deserialize)]
struct SnapshotsResp {
    snapshots: Option<HashMap<String, Snapshot>>,
    // Sometimes the API returns the map directly
    #[serde(flatten)]
    direct: Option<HashMap<String, Snapshot>>,
}

// ── Client ────────────────────────────────────────────────────────────────────

/// Async client for Alpaca historical stock market data.
#[derive(Clone)]
pub struct StockHistoricalDataClient {
    client: RestClient,
}

impl StockHistoricalDataClient {
    /// Create a new client with API key authentication.
    pub fn new(
        api_key: Option<&str>,
        secret_key: Option<&str>,
        sandbox: bool,
    ) -> Result<Self, AlpacaError> {
        let url = if sandbox {
            base_url::DATA_SANDBOX
        } else {
            base_url::DATA
        };
        Ok(Self {
            client: RestClient::new(
                api_key.map(str::to_string),
                secret_key.map(str::to_string),
                None,
                url.to_string(),
                "v2".to_string(),
                false,
            )?,
        })
    }

    /// Create a client pointed at a custom base URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(api_key: &str, secret_key: &str, base_url: &str) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                base_url.to_string(),
                "v2".to_string(),
                false,
            )?,
        })
    }

    fn symbols_str(symbols: &[String]) -> String {
        symbols.join(",")
    }

    /// Fetch historical bars (auto-paginated).
    pub async fn get_stock_bars(&self, req: &StockBarsRequest) -> Result<BarSet, AlpacaError> {
        let tf = req
            .timeframe
            .clone()
            .unwrap_or_else(TimeFrame::day)
            .value();
        let mut params = BarsParams {
            symbols: Self::symbols_str(&req.symbols),
            timeframe: tf,
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            adjustment: req.adjustment.clone(),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
            sort: req.sort.clone(),
            asof: req.asof.clone(),
            page_token: None,
        };

        let mut result: BarSet = HashMap::new();
        loop {
            let resp: PagedBars = self.client.get("/stocks/bars", Some(&params)).await?;
            if let Some(bars) = resp.bars {
                for (sym, b) in bars {
                    result.entry(sym).or_default().extend(b);
                }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    /// Fetch historical quotes (auto-paginated).
    pub async fn get_stock_quotes(&self, req: &StockQuotesRequest) -> Result<QuoteSet, AlpacaError> {
        let mut params = QuotesParams {
            symbols: Self::symbols_str(&req.symbols),
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
            sort: req.sort.clone(),
            page_token: None,
        };

        let mut result: QuoteSet = HashMap::new();
        loop {
            let resp: PagedQuotes = self.client.get("/stocks/quotes", Some(&params)).await?;
            if let Some(quotes) = resp.quotes {
                for (sym, q) in quotes {
                    result.entry(sym).or_default().extend(q);
                }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    /// Fetch historical trades (auto-paginated).
    pub async fn get_stock_trades(&self, req: &StockTradesRequest) -> Result<TradeSet, AlpacaError> {
        let mut params = TradesParams {
            symbols: Self::symbols_str(&req.symbols),
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
            sort: req.sort.clone(),
            page_token: None,
        };

        let mut result: TradeSet = HashMap::new();
        loop {
            let resp: PagedTrades = self.client.get("/stocks/trades", Some(&params)).await?;
            if let Some(trades) = resp.trades {
                for (sym, t) in trades {
                    result.entry(sym).or_default().extend(t);
                }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    /// Fetch the latest trade for each requested symbol.
    pub async fn get_stock_latest_trade(
        &self,
        req: &StockLatestRequest,
    ) -> Result<LatestTradeSet, AlpacaError> {
        let params = LatestParams {
            symbols: Self::symbols_str(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: LatestTradesResp = self.client.get("/stocks/trades/latest", Some(&params)).await?;
        Ok(resp.trades.unwrap_or_default())
    }

    /// Fetch the latest quote for each requested symbol.
    pub async fn get_stock_latest_quote(
        &self,
        req: &StockLatestRequest,
    ) -> Result<LatestQuoteSet, AlpacaError> {
        let params = LatestParams {
            symbols: Self::symbols_str(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: LatestQuotesResp = self.client.get("/stocks/quotes/latest", Some(&params)).await?;
        Ok(resp.quotes.unwrap_or_default())
    }

    /// Fetch the latest bar for each requested symbol.
    pub async fn get_stock_latest_bar(
        &self,
        req: &StockLatestRequest,
    ) -> Result<LatestBarSet, AlpacaError> {
        let params = LatestParams {
            symbols: Self::symbols_str(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: LatestBarsResp = self.client.get("/stocks/bars/latest", Some(&params)).await?;
        Ok(resp.bars.unwrap_or_default())
    }

    /// Fetch snapshots for multiple symbols.
    pub async fn get_stock_snapshot(
        &self,
        req: &StockSnapshotRequest,
    ) -> Result<SnapshotSet, AlpacaError> {
        let params = SnapshotParams {
            symbols: Self::symbols_str(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: SnapshotsResp = self.client.get("/stocks/snapshots", Some(&params)).await?;
        Ok(resp.snapshots.or(resp.direct).unwrap_or_default())
    }
}
