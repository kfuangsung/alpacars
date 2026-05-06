use crate::common::client::base_url;
use crate::data::enums::DataFeed;
use crate::data::live::websocket::{DataStreamConnection, RawStreamEvent, SubscribeMsg};
use crate::data::models::{Bar, Quote, Trade, TradeCancel, TradeCorrection, TradingStatus};
use crate::error::AlpacaError;
use std::sync::Arc;
use tracing::warn;

pub type Handler<T> = Arc<dyn Fn(T) + Send + Sync + 'static>;

/// Real-time WebSocket stream for US equity market data.
pub struct StockDataStream {
    api_key: String,
    secret_key: String,
    feed: DataFeed,
    // Subscriptions
    trade_syms: Vec<String>,
    quote_syms: Vec<String>,
    bar_syms: Vec<String>,
    updated_bar_syms: Vec<String>,
    daily_bar_syms: Vec<String>,
    status_syms: Vec<String>,
    // Handlers
    trade_handler: Option<Handler<Trade>>,
    quote_handler: Option<Handler<Quote>>,
    bar_handler: Option<Handler<Bar>>,
    updated_bar_handler: Option<Handler<Bar>>,
    daily_bar_handler: Option<Handler<Bar>>,
    trading_status_handler: Option<Handler<TradingStatus>>,
    trade_cancel_handler: Option<Handler<TradeCancel>>,
    trade_correction_handler: Option<Handler<TradeCorrection>>,
}

impl StockDataStream {
    pub fn new(api_key: &str, secret_key: &str, feed: DataFeed) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            feed,
            trade_syms: Vec::new(),
            quote_syms: Vec::new(),
            bar_syms: Vec::new(),
            updated_bar_syms: Vec::new(),
            daily_bar_syms: Vec::new(),
            status_syms: Vec::new(),
            trade_handler: None,
            quote_handler: None,
            bar_handler: None,
            updated_bar_handler: None,
            daily_bar_handler: None,
            trading_status_handler: None,
            trade_cancel_handler: None,
            trade_correction_handler: None,
        }
    }

    pub fn subscribe_trades<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Trade) + Send + Sync + 'static {
        self.trade_syms.extend(symbols.into_iter().map(Into::into));
        self.trade_handler = Some(Arc::new(handler));
    }

    pub fn subscribe_quotes<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Quote) + Send + Sync + 'static {
        self.quote_syms.extend(symbols.into_iter().map(Into::into));
        self.quote_handler = Some(Arc::new(handler));
    }

    pub fn subscribe_bars<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Bar) + Send + Sync + 'static {
        self.bar_syms.extend(symbols.into_iter().map(Into::into));
        self.bar_handler = Some(Arc::new(handler));
    }

    pub fn subscribe_updated_bars<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Bar) + Send + Sync + 'static {
        self.updated_bar_syms.extend(symbols.into_iter().map(Into::into));
        self.updated_bar_handler = Some(Arc::new(handler));
    }

    pub fn subscribe_daily_bars<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Bar) + Send + Sync + 'static {
        self.daily_bar_syms.extend(symbols.into_iter().map(Into::into));
        self.daily_bar_handler = Some(Arc::new(handler));
    }

    pub fn subscribe_trading_statuses<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(TradingStatus) + Send + Sync + 'static {
        self.status_syms.extend(symbols.into_iter().map(Into::into));
        self.trading_status_handler = Some(Arc::new(handler));
    }

    pub fn register_trade_cancels<F>(&mut self, handler: F)
    where F: Fn(TradeCancel) + Send + Sync + 'static {
        self.trade_cancel_handler = Some(Arc::new(handler));
    }

    pub fn register_trade_corrections<F>(&mut self, handler: F)
    where F: Fn(TradeCorrection) + Send + Sync + 'static {
        self.trade_correction_handler = Some(Arc::new(handler));
    }

    fn ws_url(&self) -> String {
        let feed_str = match self.feed {
            DataFeed::Iex => "iex",
            DataFeed::Sip => "sip",
            DataFeed::DelayedSip => "delayed_sip",
            DataFeed::Otc => "otc",
            _ => "iex",
        };
        format!("{}/v2/{}", base_url::MARKET_DATA_STREAM, feed_str)
    }

    /// Connect and run the event loop until the stream ends or an error occurs.
    pub async fn run(&self) -> Result<(), AlpacaError> {
        let sub = SubscribeMsg::subscribe(
            self.trade_syms.clone(),
            self.quote_syms.clone(),
            self.bar_syms.clone(),
            self.updated_bar_syms.clone(),
            self.daily_bar_syms.clone(),
            self.status_syms.clone(),
            vec![],
            vec![],
        );

        let conn = DataStreamConnection::new(
            self.ws_url(),
            self.api_key.clone(),
            self.secret_key.clone(),
            sub,
        );

        let trade_h = self.trade_handler.clone();
        let quote_h = self.quote_handler.clone();
        let bar_h = self.bar_handler.clone();
        let updated_bar_h = self.updated_bar_handler.clone();
        let daily_bar_h = self.daily_bar_handler.clone();
        let status_h = self.trading_status_handler.clone();
        let cancel_h = self.trade_cancel_handler.clone();
        let correction_h = self.trade_correction_handler.clone();

        conn.run(move |event: RawStreamEvent| {
            let msg_type = event.msg_type.as_deref().unwrap_or("");
            let raw = serde_json::Value::Object(
                event.fields.into_iter().collect::<serde_json::Map<_, _>>()
            );

            match msg_type {
                "t" => match serde_json::from_value::<Trade>(raw) {
                    Ok(v) => { if let Some(h) = &trade_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize stock Trade"),
                },
                "q" => match serde_json::from_value::<Quote>(raw) {
                    Ok(v) => { if let Some(h) = &quote_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize stock Quote"),
                },
                "b" => match serde_json::from_value::<Bar>(raw) {
                    Ok(v) => { if let Some(h) = &bar_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize stock Bar"),
                },
                "u" => match serde_json::from_value::<Bar>(raw) {
                    Ok(v) => { if let Some(h) = &updated_bar_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize stock updated Bar"),
                },
                "d" => match serde_json::from_value::<Bar>(raw) {
                    Ok(v) => { if let Some(h) = &daily_bar_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize stock daily Bar"),
                },
                "s" => match serde_json::from_value::<TradingStatus>(raw) {
                    Ok(v) => { if let Some(h) = &status_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize TradingStatus"),
                },
                "x" => match serde_json::from_value::<TradeCancel>(raw) {
                    Ok(v) => { if let Some(h) = &cancel_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize TradeCancel"),
                },
                "c" => match serde_json::from_value::<TradeCorrection>(raw) {
                    Ok(v) => { if let Some(h) = &correction_h { h(v); } }
                    Err(e) => warn!(error = %e, "failed to deserialize TradeCorrection"),
                },
                _ => {}
            }
        })
        .await
    }
}
