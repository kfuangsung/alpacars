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
