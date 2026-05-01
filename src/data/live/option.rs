use crate::common::client::base_url;
use crate::data::live::websocket::{DataStreamConnection, RawStreamEvent, SubscribeMsg};
use crate::data::models::{Quote, Trade};
use crate::error::AlpacaError;
use std::sync::Arc;

pub type Handler<T> = Arc<dyn Fn(T) + Send + Sync + 'static>;

/// Real-time WebSocket stream for options market data.
pub struct OptionDataStream {
    api_key: String,
    secret_key: String,
    trade_syms: Vec<String>,
    quote_syms: Vec<String>,
    trade_handler: Option<Handler<Trade>>,
    quote_handler: Option<Handler<Quote>>,
}

impl OptionDataStream {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            trade_syms: Vec::new(),
            quote_syms: Vec::new(),
            trade_handler: None,
            quote_handler: None,
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

    pub async fn run(&self) -> Result<(), AlpacaError> {
        let sub = SubscribeMsg::subscribe(
            self.trade_syms.clone(),
            self.quote_syms.clone(),
            vec![], vec![], vec![], vec![], vec![], vec![],
        );

        let url = format!("{}/v2/options/opra", base_url::MARKET_DATA_STREAM);
        let conn = DataStreamConnection::new(url, self.api_key.clone(), self.secret_key.clone(), sub);

        let trade_h = self.trade_handler.clone();
        let quote_h = self.quote_handler.clone();

        conn.run(move |event: RawStreamEvent| {
            let msg_type = event.msg_type.as_deref().unwrap_or("");
            let raw = serde_json::to_value(&event.fields).unwrap_or_default();
            match msg_type {
                "t" => { if let (Some(h), Ok(v)) = (&trade_h, serde_json::from_value(raw)) { h(v); } }
                "q" => { if let (Some(h), Ok(v)) = (&quote_h, serde_json::from_value(raw)) { h(v); } }
                _ => {}
            }
        })
        .await
    }
}
