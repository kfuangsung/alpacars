use crate::common::client::{base_url, RestClient, DATA_V2_MAX_LIMIT};
use crate::data::enums::{OptionsFeed, TimeFrame};
use crate::data::models::*;
use crate::error::AlpacaError;
use crate::trading::enums::ContractType;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Default)]
struct OptionBarsParams {
    pub symbols: String,
    pub timeframe: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<OptionsFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct OptionTradesParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<OptionsFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct OptionLatestParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<OptionsFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
struct OptionSnapshotParams {
    pub symbols: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<OptionsFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_lte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
}

// Request types

#[derive(Default)]
pub struct OptionBarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: Option<TimeFrame>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub feed: Option<OptionsFeed>,
    pub currency: Option<String>,
    pub sort: Option<String>,
}

#[derive(Default)]
pub struct OptionTradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub feed: Option<OptionsFeed>,
    pub currency: Option<String>,
    pub sort: Option<String>,
}

#[derive(Default)]
pub struct OptionLatestRequest {
    pub symbols: Vec<String>,
    pub feed: Option<OptionsFeed>,
    pub currency: Option<String>,
}

#[derive(Default)]
pub struct OptionSnapshotRequest {
    pub symbols: Vec<String>,
    pub feed: Option<OptionsFeed>,
    pub currency: Option<String>,
    pub updated_since: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub expiration_date: Option<NaiveDate>,
}

/// Snapshots of every contract on an underlying (`GET /v1beta1/options/snapshots/{underlying}`),
/// narrowed by the optional filters. Results are paginated for you.
#[derive(Debug, Clone, Default)]
pub struct OptionChainRequest {
    pub underlying_symbol: String,
    pub feed: Option<OptionsFeed>,
    /// Page size; every page is fetched regardless.
    pub limit: Option<u32>,
    pub updated_since: Option<DateTime<Utc>>,
    /// Only calls or only puts.
    pub r#type: Option<ContractType>,
    pub strike_price_gte: Option<f64>,
    pub strike_price_lte: Option<f64>,
    pub expiration_date: Option<NaiveDate>,
    pub expiration_date_gte: Option<NaiveDate>,
    pub expiration_date_lte: Option<NaiveDate>,
    /// e.g. `AAPL1` for adjusted contracts.
    pub root_symbol: Option<String>,
}

#[derive(Serialize)]
struct OptionChainParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    feed: Option<&'a OptionsFeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_since: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<&'a ContractType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strike_price_gte: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strike_price_lte: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date_gte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date_lte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_symbol: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

// Paged response types

#[derive(serde::Deserialize)]
struct PagedBars {
    bars: Option<HashMap<String, Vec<Bar>>>,
    next_page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct PagedTrades {
    trades: Option<HashMap<String, Vec<Trade>>>,
    next_page_token: Option<String>,
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
struct OptionSnapshotsResp {
    snapshots: Option<HashMap<String, OptionsSnapshot>>,
    next_page_token: Option<String>,
    #[serde(flatten)]
    direct: Option<HashMap<String, OptionsSnapshot>>,
}

/// Async client for Alpaca historical options market data.
#[derive(Clone)]
pub struct OptionHistoricalDataClient {
    client: RestClient,
}

impl OptionHistoricalDataClient {
    pub fn new(api_key: Option<&str>, secret_key: Option<&str>) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                api_key.map(str::to_string),
                secret_key.map(str::to_string),
                None,
                base_url::DATA.to_string(),
                "v1beta1".to_string(),
                false,
            )?,
        })
    }

    /// Create a client pointed at a custom base URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(
        api_key: &str,
        secret_key: &str,
        base_url: &str,
    ) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                base_url.to_string(),
                "v1beta1".to_string(),
                false,
            )?,
        })
    }

    fn syms(symbols: &[String]) -> String {
        symbols.join(",")
    }

    pub async fn get_option_bars(&self, req: &OptionBarsRequest) -> Result<BarSet, AlpacaError> {
        let tf = req.timeframe.clone().unwrap_or_else(TimeFrame::day).value();
        let mut params = OptionBarsParams {
            symbols: Self::syms(&req.symbols),
            timeframe: tf,
            start: req.start,
            end: req.end,
            limit: Some(req.limit.unwrap_or(DATA_V2_MAX_LIMIT)),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
            sort: req.sort.clone(),
            page_token: None,
        };

        let mut result: BarSet = HashMap::new();
        loop {
            let resp: PagedBars = self.client.get("/options/bars", Some(&params)).await?;
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
        Ok(with_symbols(result))
    }

    pub async fn get_option_latest_quote(
        &self,
        req: &OptionLatestRequest,
    ) -> Result<LatestQuoteSet, AlpacaError> {
        let params = OptionLatestParams {
            symbols: Self::syms(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: LatestQuotesResp = self
            .client
            .get("/options/quotes/latest", Some(&params))
            .await?;
        Ok(with_symbols(resp.quotes.unwrap_or_default()))
    }

    pub async fn get_option_latest_trade(
        &self,
        req: &OptionLatestRequest,
    ) -> Result<LatestTradeSet, AlpacaError> {
        let params = OptionLatestParams {
            symbols: Self::syms(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
        };
        let resp: LatestTradesResp = self
            .client
            .get("/options/trades/latest", Some(&params))
            .await?;
        Ok(with_symbols(resp.trades.unwrap_or_default()))
    }

    pub async fn get_option_trades(
        &self,
        req: &OptionTradesRequest,
    ) -> Result<TradeSet, AlpacaError> {
        let mut params = OptionTradesParams {
            symbols: Self::syms(&req.symbols),
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
            let resp: PagedTrades = self.client.get("/options/trades", Some(&params)).await?;
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
        Ok(with_symbols(result))
    }

    pub async fn get_option_snapshot(
        &self,
        req: &OptionSnapshotRequest,
    ) -> Result<OptionsSnapshotSet, AlpacaError> {
        let mut params = OptionSnapshotParams {
            symbols: Self::syms(&req.symbols),
            feed: req.feed.clone(),
            currency: req.currency.clone(),
            updated_since: req.updated_since,
            limit: req.limit,
            expiration_date: req.expiration_date,
            page_token: None,
            ..Default::default()
        };

        let mut result: OptionsSnapshotSet = HashMap::new();
        loop {
            let resp: OptionSnapshotsResp =
                self.client.get("/options/snapshots", Some(&params)).await?;
            let snaps = resp.snapshots.or(resp.direct).unwrap_or_default();
            result.extend(snaps);
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(with_symbols(result))
    }

    pub async fn get_option_exchange_codes(&self) -> Result<serde_json::Value, AlpacaError> {
        self.client
            .get_raw("/options/meta/exchanges", None::<&()>)
            .await
    }

    /// Snapshots of every contract on `req.underlying_symbol`, keyed by contract symbol,
    /// following `next_page_token` until all pages are fetched.
    pub async fn get_option_chain(
        &self,
        req: &OptionChainRequest,
    ) -> Result<OptionsSnapshotSet, AlpacaError> {
        // The chain endpoint takes the underlying as a path segment; the plain
        // `/options/snapshots` endpoint requires contract `symbols` instead.
        let path = format!("/options/snapshots/{}", req.underlying_symbol);
        let mut params = OptionChainParams {
            feed: req.feed.as_ref(),
            limit: req.limit,
            updated_since: req.updated_since,
            r#type: req.r#type.as_ref(),
            strike_price_gte: req.strike_price_gte,
            strike_price_lte: req.strike_price_lte,
            expiration_date: req.expiration_date,
            expiration_date_gte: req.expiration_date_gte,
            expiration_date_lte: req.expiration_date_lte,
            root_symbol: req.root_symbol.as_deref(),
            page_token: None,
        };

        let mut result: OptionsSnapshotSet = HashMap::new();
        loop {
            let resp: OptionSnapshotsResp = self.client.get(&path, Some(&params)).await?;
            result.extend(resp.snapshots.or(resp.direct).unwrap_or_default());
            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }
        Ok(with_symbols(result))
    }
}
