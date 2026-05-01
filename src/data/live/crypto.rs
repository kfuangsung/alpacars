use crate::common::client::base_url;
use crate::data::live::websocket::{DataStreamConnection, RawStreamEvent, SubscribeMsg};
use crate::data::models::{Bar, Orderbook, Quote, Trade};
use crate::error::AlpacaError;
use std::sync::Arc;

pub type Handler<T> = Arc<dyn Fn(T) + Send + Sync + 'static>;

/// Real-time WebSocket stream for cryptocurrency market data.
pub struct CryptoDataStream {
    api_key: String,
    secret_key: String,
    trade_syms: Vec<String>,
    quote_syms: Vec<String>,
    bar_syms: Vec<String>,
    updated_bar_syms: Vec<String>,
    daily_bar_syms: Vec<String>,
    orderbook_syms: Vec<String>,
    trade_handler: Option<Handler<Trade>>,
    quote_handler: Option<Handler<Quote>>,
    bar_handler: Option<Handler<Bar>>,
    updated_bar_handler: Option<Handler<Bar>>,
    daily_bar_handler: Option<Handler<Bar>>,
    orderbook_handler: Option<Handler<Orderbook>>,
}

impl CryptoDataStream {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            trade_syms: Vec::new(),
            quote_syms: Vec::new(),
            bar_syms: Vec::new(),
            updated_bar_syms: Vec::new(),
            daily_bar_syms: Vec::new(),
            orderbook_syms: Vec::new(),
            trade_handler: None,
            quote_handler: None,
            bar_handler: None,
            updated_bar_handler: None,
            daily_bar_handler: None,
            orderbook_handler: None,
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

    pub fn subscribe_orderbooks<F>(&mut self, symbols: impl IntoIterator<Item = impl Into<String>>, handler: F)
    where F: Fn(Orderbook) + Send + Sync + 'static {
        self.orderbook_syms.extend(symbols.into_iter().map(Into::into));
        self.orderbook_handler = Some(Arc::new(handler));
    }

    pub async fn run(&self) -> Result<(), AlpacaError> {
        let sub = SubscribeMsg::subscribe(
            self.trade_syms.clone(),
            self.quote_syms.clone(),
            self.bar_syms.clone(),
            self.updated_bar_syms.clone(),
            self.daily_bar_syms.clone(),
            vec![],
            self.orderbook_syms.clone(),
            vec![],
        );

        let url = format!("{}/v2/crypto/us", base_url::MARKET_DATA_STREAM);
        let conn = DataStreamConnection::new(url, self.api_key.clone(), self.secret_key.clone(), sub);

        let trade_h = self.trade_handler.clone();
        let quote_h = self.quote_handler.clone();
        let bar_h = self.bar_handler.clone();
        let updated_bar_h = self.updated_bar_handler.clone();
        let daily_bar_h = self.daily_bar_handler.clone();
        let ob_h = self.orderbook_handler.clone();

        conn.run(move |event: RawStreamEvent| {
            let msg_type = event.msg_type.as_deref().unwrap_or("");
            let raw = serde_json::to_value(&event.fields).unwrap_or_default();
            match msg_type {
                "t" => { if let (Some(h), Ok(v)) = (&trade_h, serde_json::from_value(raw)) { h(v); } }
                "q" => { if let (Some(h), Ok(v)) = (&quote_h, serde_json::from_value(raw)) { h(v); } }
                "b" => { if let (Some(h), Ok(v)) = (&bar_h, serde_json::from_value(raw)) { h(v); } }
                "u" => { if let (Some(h), Ok(v)) = (&updated_bar_h, serde_json::from_value(raw)) { h(v); } }
                "d" => { if let (Some(h), Ok(v)) = (&daily_bar_h, serde_json::from_value(raw)) { h(v); } }
                "o" => { if let (Some(h), Ok(v)) = (&ob_h, serde_json::from_value(raw)) { h(v); } }
                _ => {}
            }
        })
        .await
    }
}
