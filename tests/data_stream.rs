use alpacars::data::live::websocket::{DataStreamConnection, RawStreamEvent, SubscribeMsg};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;

/// Reads the next text frame from the client and parses it as JSON.
async fn next_json<S>(ws: &mut S) -> Value
where
    S: StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    match ws.next().await.expect("client closed early").unwrap() {
        Message::Text(text) => serde_json::from_str(&text).unwrap(),
        other => panic!("expected a text frame, got {other:?}"),
    }
}

/// Drives `DataStreamConnection` against a local WebSocket server end to end: the auth and
/// subscribe frames it sends, JSON text frames, msgpack binary frames, ping/pong, and close.
#[tokio::test]
async fn test_data_stream_round_trip() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(tcp).await.unwrap();

        let auth = next_json(&mut ws).await;
        assert_eq!(
            auth,
            json!([{"action": "auth", "key": "test-key", "secret": "test-secret"}])
        );

        let sub = next_json(&mut ws).await;
        assert_eq!(
            sub,
            json!([{"action": "subscribe", "trades": ["AAPL"], "quotes": ["AAPL"]}])
        );

        // Control messages are swallowed; the trade is delivered.
        let text = json!([
            {"T": "success", "msg": "authenticated"},
            {"T": "t", "S": "AAPL", "p": 227.5, "s": 100}
        ]);
        ws.send(Message::Text(text.to_string().into()))
            .await
            .unwrap();

        // The market data stream also sends msgpack in binary frames.
        let quote =
            rmp_serde::to_vec_named(&vec![json!({"T": "q", "S": "AAPL", "bp": 227.4})]).unwrap();
        ws.send(Message::Binary(quote.into())).await.unwrap();

        ws.send(Message::Ping(b"hb".to_vec().into())).await.unwrap();
        match ws.next().await.expect("no pong").unwrap() {
            Message::Pong(data) => assert_eq!(data.as_ref(), b"hb"),
            other => panic!("expected a pong, got {other:?}"),
        }

        ws.close(None).await.unwrap();
    });

    let subscribe = SubscribeMsg::subscribe(
        vec!["AAPL".to_string()],
        vec!["AAPL".to_string()],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    );
    let conn = DataStreamConnection::new(
        format!("ws://{addr}"),
        "test-key".to_string(),
        "test-secret".to_string(),
        subscribe,
    );

    let mut events: Vec<RawStreamEvent> = Vec::new();
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        conn.run(|e| events.push(e)),
    )
    .await
    .expect("stream did not finish after the server closed")
    .unwrap();
    server.await.unwrap();

    assert_eq!(events.len(), 2);
    assert_eq!(events[0].msg_type.as_deref(), Some("t"));
    assert_eq!(events[0].symbol.as_deref(), Some("AAPL"));
    assert_eq!(events[0].fields["p"], json!(227.5));
    assert_eq!(events[1].msg_type.as_deref(), Some("q"));
    assert_eq!(events[1].fields["bp"], json!(227.4));
}

/// `StockDataStream` end to end, including `Trade.symbol` (0.8.0), with the example messages from Alpaca's "Real-time Stock Data"
/// doc. `RawStreamEvent` takes `S` out of each message, so it has to be put back before
/// typed parsing; without that, every trading status failed to parse (its `symbol` is
/// required) and cancels / corrections lost their symbol.
#[tokio::test]
async fn test_stock_stream_status_cancel_correction_keep_symbol() {
    use alpacars::data::live::stock::StockDataStream;
    use std::sync::{Arc, Mutex};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(tcp).await.unwrap();
        next_json(&mut ws).await; // auth
        let sub = next_json(&mut ws).await;
        assert_eq!(
            sub,
            json!([{"action": "subscribe", "trades": ["AAPL"], "statuses": ["AAPL"]}])
        );
        let text = json!([
            {"T": "t", "i": 96921, "S": "AAPL", "x": "D", "p": 126.55, "s": 1,
             "t": "2021-02-22T15:51:44.208Z", "c": ["@", "I"], "z": "C"},
            {"T": "s", "S": "AAPL", "sc": "H", "sm": "Trading Halt", "rc": "T12",
             "rm": "Trading Halted; For information requested by NASDAQ",
             "t": "2021-02-22T19:15:00Z", "z": "C"},
            {"T": "x", "S": "GOOGL", "i": 465, "x": "D", "p": 105.31, "s": 300, "a": "C",
             "z": "C", "t": "2023-04-06T13:15:42.83540958Z"},
            {"T": "c", "S": "EEM", "x": "M", "oi": 52983525033527u64, "op": 39.1582,
             "os": 440000, "oc": [" ", "7"], "ci": 52983525034326u64, "cp": 39.1809,
             "cs": 440000, "cc": [" ", "7"], "z": "B", "t": "2023-04-06T14:25:06.542305024Z"}
        ]);
        ws.send(Message::Text(text.to_string().into()))
            .await
            .unwrap();
        ws.close(None).await.unwrap();
    });

    let seen: Arc<Mutex<Vec<String>>> = Arc::default();
    let mut stream =
        StockDataStream::new_with_url("test-key", "test-secret", &format!("ws://{addr}"));
    let s = seen.clone();
    stream.subscribe_trading_statuses(["AAPL"], move |status| {
        s.lock()
            .unwrap()
            .push(format!("status {} {:?}", status.symbol, status.status_code));
    });
    let s = seen.clone();
    stream.subscribe_trades(["AAPL"], move |trade| {
        s.lock().unwrap().push(format!("trade {}", trade.symbol));
    });
    let s = seen.clone();
    stream.register_trade_cancels(move |cancel| {
        s.lock()
            .unwrap()
            .push(format!("cancel {:?}", cancel.symbol));
    });
    let s = seen.clone();
    stream.register_trade_corrections(move |correction| {
        s.lock()
            .unwrap()
            .push(format!("correction {:?}", correction.symbol));
    });

    tokio::time::timeout(std::time::Duration::from_secs(10), stream.run())
        .await
        .expect("stream did not finish after the server closed")
        .unwrap();
    server.await.unwrap();

    assert_eq!(
        *seen.lock().unwrap(),
        vec![
            "trade AAPL".to_string(),
            "status AAPL Some(\"H\")".to_string(),
            "cancel Some(\"GOOGL\")".to_string(),
            "correction Some(\"EEM\")".to_string(),
        ]
    );
}
