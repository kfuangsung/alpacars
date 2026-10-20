//! `TradingStream` against a local WebSocket server, following Alpaca's "Websocket Streaming"
//! doc: the `auth` / `listen` messages it must send, and `trade_updates` delivered in **binary**
//! frames (as the paper stream sends them) as well as text frames.

use alpacars::trading::enums::TradeEvent;
use alpacars::trading::models::TradeUpdate;
use alpacars::trading::stream::TradingStream;
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

// The `trade_updates` example message in the "Websocket Streaming" doc.
const FILL_EVENT: &str = r#"{
  "stream": "trade_updates",
  "data": {
    "event": "fill",
    "execution_id": "2f63ea93-423d-4169-b3f6-3fdafc10c418",
    "order": {
      "asset_class": "crypto",
      "asset_id": "1cf35270-99ee-44e2-a77f-6fab902c7f80",
      "cancel_requested_at": null,
      "canceled_at": null,
      "client_order_id": "4642fd68-d59a-47d7-a9ac-e22f536828d1",
      "created_at": "2022-04-19T13:45:04.981350886-04:00",
      "expired_at": null,
      "extended_hours": false,
      "failed_at": null,
      "filled_at": "2022-04-19T17:45:05.024916716Z",
      "filled_avg_price": "105.8988475",
      "filled_qty": "1790.86",
      "hwm": null,
      "id": "a5be8f5e-fdfa-41f5-a644-7a74fe947a8f",
      "legs": null,
      "limit_price": null,
      "notional": null,
      "order_class": "",
      "order_type": "market",
      "qty": "1790.86",
      "replaced_at": null,
      "replaced_by": null,
      "replaces": null,
      "side": "sell",
      "status": "filled",
      "stop_price": null,
      "submitted_at": "2022-04-19T13:45:04.980944666-04:00",
      "symbol": "SOLUSD",
      "time_in_force": "gtc",
      "trail_percent": null,
      "trail_price": null,
      "type": "market",
      "updated_at": "2022-04-19T13:45:05.027690731-04:00"
    },
    "position_qty": "0",
    "price": "105.8988475",
    "qty": "1790.86",
    "timestamp": "2022-04-19T17:45:05.024916716Z"
  }
}"#;

async fn next_json<S>(ws: &mut S) -> Value
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    match ws.next().await.expect("client closed early").unwrap() {
        Message::Text(text) => serde_json::from_str(&text).unwrap(),
        other => panic!("expected a text frame, got {other:?}"),
    }
}

#[tokio::test]
async fn test_trading_stream_round_trip() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(tcp).await.unwrap();

        // Documented auth message shape.
        assert_eq!(
            next_json(&mut ws).await,
            json!({"action": "auth", "key": "test-key", "secret": "test-secret"})
        );
        assert_eq!(
            next_json(&mut ws).await,
            json!({"action": "listen", "data": {"streams": ["trade_updates"]}})
        );

        let authorized = json!({"stream": "authorization", "data": {"status": "authorized", "action": "authenticate"}});
        ws.send(Message::Text(authorized.to_string().into()))
            .await
            .unwrap();
        let listening = json!({"stream": "listening", "data": {"streams": ["trade_updates"]}});
        ws.send(Message::Text(listening.to_string().into()))
            .await
            .unwrap();

        // The paper stream sends trade updates in binary frames.
        ws.send(Message::Binary(FILL_EVENT.as_bytes().to_vec().into()))
            .await
            .unwrap();
        ws.send(Message::Text(FILL_EVENT.to_string().into()))
            .await
            .unwrap();

        ws.send(Message::Ping(b"hb".to_vec().into())).await.unwrap();
        match ws.next().await.expect("no pong").unwrap() {
            Message::Pong(data) => assert_eq!(data.as_ref(), b"hb"),
            other => panic!("expected a pong, got {other:?}"),
        }
        ws.close(None).await.unwrap();
    });

    let updates: Arc<Mutex<Vec<TradeUpdate>>> = Arc::default();
    let sink = updates.clone();
    let mut stream =
        TradingStream::new_with_url("test-key", "test-secret", &format!("ws://{addr}"));
    stream.subscribe_trade_updates(move |u| sink.lock().unwrap().push(u));

    tokio::time::timeout(std::time::Duration::from_secs(10), stream.run())
        .await
        .expect("stream did not finish after the server closed")
        .unwrap();
    server.await.unwrap();

    let updates = updates.lock().unwrap();
    assert_eq!(
        updates.len(),
        2,
        "both the binary and the text frame must be delivered"
    );
    assert_eq!(updates[0].event, TradeEvent::Fill);
    assert!(updates[0].price.is_some());
}
