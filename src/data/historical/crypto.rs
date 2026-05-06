use crate::common::client::{base_url, RestClient, DATA_V2_MAX_LIMIT};
use crate::data::enums::{CryptoFeed, TimeFrame};
use crate::data::models::*;
use crate::error::AlpacaError;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Default)]
struct CryptoBarsParams {
    pub symbols: String,
    pub timeframe: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<CryptoFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct CryptoQuotesParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct CryptoLatestParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<CryptoFeed>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct CryptoSnapshotParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<CryptoFeed>,
}

// Request types

#[derive(Default)]
pub struct CryptoBarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: Option<TimeFrame>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
    pub loc: Option<CryptoFeed>,
}

#[derive(Default)]
pub struct CryptoQuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub sort: Option<String>,
}

#[derive(Default)]
pub struct CryptoLatestRequest {
    pub symbols: Vec<String>,
    pub loc: Option<CryptoFeed>,
}

#[derive(Default)]
pub struct CryptoSnapshotRequest {
    pub symbols: Vec<String>,
    pub loc: Option<CryptoFeed>,
}

// Pagination response types

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
struct LatestBarsResp { bars: Option<HashMap<String, Bar>> }
#[derive(serde::Deserialize)]
struct LatestQuotesResp { quotes: Option<HashMap<String, Quote>> }
#[derive(serde::Deserialize)]
struct LatestTradesResp { trades: Option<HashMap<String, Trade>> }
#[derive(serde::Deserialize)]
struct LatestOrderbooksResp { orderbooks: Option<HashMap<String, Orderbook>> }
#[derive(serde::Deserialize)]
struct CryptoSnapshotsResp {
    snapshots: Option<HashMap<String, Snapshot>>,
    #[serde(flatten)]
    direct: Option<HashMap<String, Snapshot>>,
}

/// Async client for Alpaca historical cryptocurrency market data.
#[derive(Clone)]
pub struct CryptoHistoricalDataClient {
    client: RestClient,
}

impl CryptoHistoricalDataClient {
    pub fn new(api_key: Option<&str>, secret_key: Option<&str>) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                api_key.map(str::to_string),
                secret_key.map(str::to_string),
                None,
                base_url::DATA.to_string(),
                "v1beta3".to_string(),
                false,
            )?,
        })
    }

    fn syms(symbols: &[String]) -> String {
        symbols.join(",")
    }

    pub async fn get_crypto_bars(&self, req: &CryptoBarsRequest) -> Result<BarSet, AlpacaError> {
        let tf = req.timeframe.clone().unwrap_or_else(TimeFrame::day).value();
        let mut params = CryptoBarsParams {
            symbols: Self::syms(&req.symbols),
            timeframe: tf,
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            sort: req.sort.clone(),
            loc: req.loc.clone(),
            page_token: None,
        };

        let mut result: BarSet = HashMap::new();
        loop {
            let resp: PagedBars = self.client.get("/crypto/us/bars", Some(&params)).await?;
            if let Some(bars) = resp.bars {
                for (sym, b) in bars { result.entry(sym).or_default().extend(b); }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    pub async fn get_crypto_quotes(&self, req: &CryptoQuotesRequest) -> Result<QuoteSet, AlpacaError> {
        let mut params = CryptoQuotesParams {
            symbols: Self::syms(&req.symbols),
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            sort: req.sort.clone(),
            page_token: None,
        };

        let mut result: QuoteSet = HashMap::new();
        loop {
            let resp: PagedQuotes = self.client.get("/crypto/us/quotes", Some(&params)).await?;
            if let Some(quotes) = resp.quotes {
                for (sym, q) in quotes { result.entry(sym).or_default().extend(q); }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    pub async fn get_crypto_trades(&self, req: &CryptoQuotesRequest) -> Result<TradeSet, AlpacaError> {
        let mut params = CryptoQuotesParams {
            symbols: Self::syms(&req.symbols),
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            sort: req.sort.clone(),
            page_token: None,
        };

        let mut result: TradeSet = HashMap::new();
        loop {
            let resp: PagedTrades = self.client.get("/crypto/us/trades", Some(&params)).await?;
            if let Some(trades) = resp.trades {
                for (sym, t) in trades { result.entry(sym).or_default().extend(t); }
            }
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(result)
    }

    pub async fn get_crypto_latest_trade(&self, req: &CryptoLatestRequest) -> Result<LatestTradeSet, AlpacaError> {
        let params = CryptoLatestParams { symbols: Self::syms(&req.symbols), loc: req.loc.clone() };
        let resp: LatestTradesResp = self.client.get("/crypto/us/latest/trades", Some(&params)).await?;
        Ok(resp.trades.unwrap_or_default())
    }

    pub async fn get_crypto_latest_quote(&self, req: &CryptoLatestRequest) -> Result<LatestQuoteSet, AlpacaError> {
        let params = CryptoLatestParams { symbols: Self::syms(&req.symbols), loc: req.loc.clone() };
        let resp: LatestQuotesResp = self.client.get("/crypto/us/latest/quotes", Some(&params)).await?;
        Ok(resp.quotes.unwrap_or_default())
    }

    pub async fn get_crypto_latest_bar(&self, req: &CryptoLatestRequest) -> Result<LatestBarSet, AlpacaError> {
        let params = CryptoLatestParams { symbols: Self::syms(&req.symbols), loc: req.loc.clone() };
        let resp: LatestBarsResp = self.client.get("/crypto/us/latest/bars", Some(&params)).await?;
        Ok(resp.bars.unwrap_or_default())
    }

    pub async fn get_crypto_latest_orderbook(&self, req: &CryptoLatestRequest) -> Result<LatestOrderbookSet, AlpacaError> {
        let params = CryptoLatestParams { symbols: Self::syms(&req.symbols), loc: req.loc.clone() };
        let resp: LatestOrderbooksResp = self.client.get("/crypto/us/latest/orderbooks", Some(&params)).await?;
        Ok(resp.orderbooks.unwrap_or_default())
    }

    pub async fn get_crypto_snapshot(&self, req: &CryptoSnapshotRequest) -> Result<SnapshotSet, AlpacaError> {
        let params = CryptoSnapshotParams { symbols: Self::syms(&req.symbols), loc: req.loc.clone() };
        let resp: CryptoSnapshotsResp = self.client.get("/crypto/us/snapshots", Some(&params)).await?;
        Ok(resp.snapshots.or(resp.direct).unwrap_or_default())
    }
}
