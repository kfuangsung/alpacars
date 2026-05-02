use crate::error::AlpacaError;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// A single decoded event from the Alpaca data WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
pub struct RawStreamEvent {
    /// Message type: "t" (trade), "q" (quote), "b" (bar), "T" (trading status), etc.
    #[serde(rename = "T")]
    pub msg_type: Option<String>,
    /// Symbol
    #[serde(rename = "S")]
    pub symbol: Option<String>,
    /// Remaining fields as raw JSON
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Serialize)]
struct AuthMsg<'a> {
    action: &'a str,
    key: &'a str,
    secret: &'a str,
}

#[derive(Debug, Serialize)]
pub struct SubscribeMsg {
    pub action: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quotes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bars: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub updated_bars: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub daily_bars: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub statuses: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub orderbooks: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub news: Vec<String>,
}

impl SubscribeMsg {
    pub fn subscribe(
        trades: Vec<String>,
        quotes: Vec<String>,
        bars: Vec<String>,
        updated_bars: Vec<String>,
        daily_bars: Vec<String>,
        statuses: Vec<String>,
        orderbooks: Vec<String>,
        news: Vec<String>,
    ) -> Self {
        Self {
            action: "subscribe".to_string(),
            trades,
            quotes,
            bars,
            updated_bars,
            daily_bars,
            statuses,
            orderbooks,
            news,
        }
    }
}

/// Low-level data stream connection. Handles auth and raw frame reading.
pub struct DataStreamConnection {
    ws_url: String,
    api_key: String,
    secret_key: String,
    subscribe_msg: SubscribeMsg,
}

impl DataStreamConnection {
    pub fn new(
        ws_url: String,
        api_key: String,
        secret_key: String,
        subscribe_msg: SubscribeMsg,
    ) -> Self {
        Self { ws_url, api_key, secret_key, subscribe_msg }
    }

    /// Connect, authenticate, subscribe, then call `on_event` for every incoming event.
    pub async fn run<F>(&self, mut on_event: F) -> Result<(), AlpacaError>
    where
        F: FnMut(RawStreamEvent),
    {
        let (ws, _) = connect_async(&self.ws_url)
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        let (mut write, mut read) = ws.split();

        // Authenticate using JSON (Alpaca data streams accept JSON in addition to msgpack)
        let auth = serde_json::to_string(&[AuthMsg {
            action: "auth",
            key: &self.api_key,
            secret: &self.secret_key,
        }])?;
        write
            .send(Message::Text(auth))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        // Subscribe
        let sub = serde_json::to_string(&[&self.subscribe_msg])?;
        write
            .send(Message::Text(sub))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        while let Some(msg) = read.next().await {
            let msg = msg.map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

            let events: Vec<RawStreamEvent> = match msg {
                Message::Text(text) => serde_json::from_str(&text).unwrap_or_default(),
                Message::Binary(bytes) => {
                    // Try msgpack decode first
                    match rmp_serde::from_slice::<Vec<RawStreamEvent>>(&bytes) {
                        Ok(evs) => evs,
                        Err(_) => serde_json::from_slice(&bytes).unwrap_or_default(),
                    }
                }
                Message::Ping(data) => {
                    let _ = write.send(Message::Pong(data)).await;
                    continue;
                }
                Message::Close(_) => break,
                _ => continue,
            };

            for event in events {
                // Skip control messages (connected, authenticated, subscription)
                if let Some(ref t) = event.msg_type {
                    if t == "success" || t == "subscription" || t == "error" {
                        continue;
                    }
                }
                on_event(event);
            }
        }

        Ok(())
    }
}
