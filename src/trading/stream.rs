use crate::common::client::base_url;
use crate::error::AlpacaError;
use crate::trading::models::TradeUpdate;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, warn};

#[derive(Debug, Serialize)]
struct AuthMessage<'a> {
    action: &'a str,
    key: &'a str,
    secret: &'a str,
}

#[derive(Debug, Serialize)]
struct ListenMessage<'a> {
    action: &'a str,
    data: ListenData<'a>,
}

#[derive(Debug, Serialize)]
struct ListenData<'a> {
    streams: &'a [&'a str],
}

pub type TradeUpdateHandler = Arc<dyn Fn(TradeUpdate) + Send + Sync + 'static>;

/// Real-time WebSocket stream for trade updates.
///
/// Connect to the Alpaca trading stream to receive live order/fill notifications.
pub struct TradingStream {
    api_key: String,
    secret_key: String,
    paper: bool,
    trade_update_handler: Option<TradeUpdateHandler>,
}

impl TradingStream {
    pub fn new(api_key: &str, secret_key: &str, paper: bool) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            paper,
            trade_update_handler: None,
        }
    }

    /// Register a handler called for every trade update event.
    pub fn subscribe_trade_updates<F>(&mut self, handler: F)
    where
        F: Fn(TradeUpdate) + Send + Sync + 'static,
    {
        self.trade_update_handler = Some(Arc::new(handler));
    }

    fn stream_url(&self) -> &'static str {
        if self.paper {
            base_url::TRADING_STREAM_PAPER
        } else {
            base_url::TRADING_STREAM_LIVE
        }
    }

    /// Connect and run the event loop until an error occurs or the stream ends.
    pub async fn run(&self) -> Result<(), AlpacaError> {
        let url = self.stream_url();
        debug!(url, paper = self.paper, "connecting to trading stream");
        let (ws, _) = connect_async(url)
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        let (mut write, mut read) = ws.split();

        debug!("authenticating trading stream");
        let auth = serde_json::to_string(&AuthMessage {
            action: "auth",
            key: &self.api_key,
            secret: &self.secret_key,
        })?;
        write
            .send(Message::Text(auth))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        debug!("subscribing to trade_updates");
        let listen = serde_json::to_string(&ListenMessage {
            action: "listen",
            data: ListenData {
                streams: &["trade_updates"],
            },
        })?;
        write
            .send(Message::Text(listen))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        let handler = self.trade_update_handler.clone();

        while let Some(msg) = read.next().await {
            let msg = msg.map_err(|e| AlpacaError::WebSocket(e.to_string()))?;
            match msg {
                Message::Text(text) => {
                    let event: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(error = %e, "failed to parse trading stream message");
                            continue;
                        }
                    };
                    let stream = event["stream"].as_str().unwrap_or("");
                    if stream == "trade_updates" {
                        if let Some(ref h) = handler {
                            match serde_json::from_value::<TradeUpdate>(event["data"].clone()) {
                                Ok(update) => h(update),
                                Err(e) => {
                                    warn!(error = %e, "failed to deserialize TradeUpdate");
                                }
                            }
                        }
                    } else if !stream.is_empty() {
                        debug!(stream, "received message for unhandled stream");
                    }
                }
                Message::Close(frame) => {
                    debug!(frame = ?frame, "trading stream closed by server");
                    break;
                }
                Message::Ping(data) => {
                    if let Err(e) = write.send(Message::Pong(data)).await {
                        warn!(error = %e, "failed to send Pong");
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Stop the stream by dropping this client (the run loop will exit on the next message).
    pub fn stop(&self) {}
}
