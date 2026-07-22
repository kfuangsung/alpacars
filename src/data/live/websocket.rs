use crate::error::AlpacaError;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, warn};

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
    // One Vec<String> param per subscription channel, mirroring `SubscribeMsg`'s fields 1:1.
    #[allow(clippy::too_many_arguments)]
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
        Self {
            ws_url,
            api_key,
            secret_key,
            subscribe_msg,
        }
    }

    /// Connect, authenticate, subscribe, then call `on_event` for every incoming event.
    pub async fn run<F>(&self, mut on_event: F) -> Result<(), AlpacaError>
    where
        F: FnMut(RawStreamEvent),
    {
        debug!(url = %self.ws_url, "connecting to data stream");
        let (ws, _) = connect_async(&self.ws_url)
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        let (mut write, mut read) = ws.split();

        debug!("authenticating data stream");
        let auth = serde_json::to_string(&[AuthMsg {
            action: "auth",
            key: &self.api_key,
            secret: &self.secret_key,
        }])?;
        write
            .send(Message::Text(auth))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        debug!("sending subscription message");
        let sub = serde_json::to_string(&[&self.subscribe_msg])?;
        write
            .send(Message::Text(sub))
            .await
            .map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

        while let Some(msg) = read.next().await {
            let msg = msg.map_err(|e| AlpacaError::WebSocket(e.to_string()))?;

            let events: Vec<RawStreamEvent> = match msg {
                Message::Text(text) => match serde_json::from_str(&text) {
                    Ok(evs) => evs,
                    Err(e) => {
                        warn!(error = %e, "failed to parse text frame");
                        continue;
                    }
                },
                Message::Binary(bytes) => {
                    match rmp_serde::from_slice::<Vec<RawStreamEvent>>(&bytes) {
                        Ok(evs) => evs,
                        Err(msgpack_err) => {
                            match serde_json::from_slice::<Vec<RawStreamEvent>>(&bytes) {
                                Ok(evs) => evs,
                                Err(json_err) => {
                                    warn!(
                                        msgpack_error = %msgpack_err,
                                        json_error = %json_err,
                                        "failed to parse binary frame as msgpack or JSON"
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                }
                Message::Ping(data) => {
                    if let Err(e) = write.send(Message::Pong(data)).await {
                        warn!(error = %e, "failed to send Pong");
                    }
                    continue;
                }
                Message::Close(frame) => {
                    debug!(frame = ?frame, "stream closed by server");
                    break;
                }
                _ => continue,
            };

            for event in events {
                match event.msg_type.as_deref() {
                    Some("success") => {
                        debug!("stream authenticated/connected");
                        continue;
                    }
                    Some("subscription") => {
                        debug!("subscription confirmed");
                        continue;
                    }
                    Some("error") => {
                        error!(
                            code = ?event.fields.get("code"),
                            msg = ?event.fields.get("msg"),
                            "Alpaca stream error"
                        );
                        continue;
                    }
                    _ => {}
                }
                on_event(event);
            }
        }

        Ok(())
    }
}
