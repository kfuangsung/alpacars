use crate::common::client::base_url;
use crate::error::AlpacaError;
use crate::trading::models::TradeUpdate;
use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info, warn};

/// `{"action": "auth", "key": ..., "secret": ...}`, per Alpaca's "Websocket Streaming" doc.
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
    url_override: Option<String>,
    trade_update_handler: Option<TradeUpdateHandler>,
}

impl TradingStream {
    pub fn new(api_key: &str, secret_key: &str, paper: bool) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            paper,
            url_override: None,
            trade_update_handler: None,
        }
    }

    /// Create a stream pointed at a custom WebSocket URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(api_key: &str, secret_key: &str, url: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            paper: true,
            url_override: Some(url.to_string()),
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

    fn stream_url(&self) -> &str {
        match &self.url_override {
            Some(url) => url,
            None if self.paper => base_url::TRADING_STREAM_PAPER,
            None => base_url::TRADING_STREAM_LIVE,
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
            .send(Message::Text(auth.into()))
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
            .send(Message::Text(listen.into()))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        let handler = self.trade_update_handler.clone();

        while let Some(msg) = read.next().await {
            let msg = msg.map_err(|e| AlpacaError::WebSocket(e.to_string()))?;
            match msg {
                Message::Text(text) => match serde_json::from_str(&text) {
                    Ok(event) => Self::handle_event(&event, handler.as_ref()),
                    Err(e) => warn!(error = %e, "failed to parse trading stream message"),
                },
                // The paper stream sends `trade_updates` in binary frames (Alpaca's
                // "Websocket Streaming" doc). They carry JSON, or MessagePack if negotiated.
                Message::Binary(bytes) => match serde_json::from_slice(&bytes)
                    .or_else(|_| rmp_serde::from_slice::<serde_json::Value>(&bytes))
                {
                    Ok(event) => Self::handle_event(&event, handler.as_ref()),
                    Err(e) => warn!(error = %e, "failed to parse binary trading stream message"),
                },
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

    fn handle_event(event: &serde_json::Value, handler: Option<&TradeUpdateHandler>) {
        let stream = event["stream"].as_str().unwrap_or("");
        if stream == "trade_updates" {
            let event_type = event["data"]["event"].as_str().unwrap_or("unknown");
            info!(event_type, "trading stream: trade_updates event received");
            if let Some(h) = handler {
                match serde_json::from_value::<TradeUpdate>(event["data"].clone()) {
                    Ok(update) => h(update),
                    Err(e) => {
                        warn!(error = %e, raw = %event["data"], "failed to deserialize TradeUpdate");
                    }
                }
            }
        } else if stream == "authorization" {
            let status = event["data"]["status"].as_str().unwrap_or("unknown");
            if status == "authorized" {
                info!("trading stream authenticated successfully");
            } else {
                warn!(%status, "trading stream authentication failed — no trade updates will be received");
            }
        } else if stream == "listening" {
            info!("trading stream subscription confirmed");
        } else if !stream.is_empty() {
            debug!(stream, "received message for unhandled stream");
        }
    }

    /// Stop the stream by dropping this client (the run loop will exit on the next message).
    pub fn stop(&self) {}
}
