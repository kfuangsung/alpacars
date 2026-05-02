mod common;

use alpacars::data::historical::stock::{
    StockBarsRequest, StockLatestRequest, StockQuotesRequest, StockSnapshotRequest,
    StockTradesRequest,
};
use alpacars::data::models::{Bar, Quote, Snapshot, Trade};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_stock_bars_single_symbol() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "bars": {
                "AAPL": [
                    {"t": "2022-02-01T05:00:00Z", "o": 174.0, "h": 174.84, "l": 172.31, "c": 174.61, "v": 85998033, "n": 732412, "vw": 173.703516},
                    {"t": "2022-02-02T05:00:00Z", "o": 174.64, "h": 175.88, "l": 173.33, "c": 175.84, "v": 84817432, "n": 675034, "vw": 174.941288}
                ]
            },
            "next_page_token": null
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockBarsRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let barset = client.get_stock_bars(&req).await.unwrap();

    assert!(barset.contains_key("AAPL"));
    let bars = &barset["AAPL"];
    assert_eq!(bars.len(), 2);
    assert!(matches!(bars[0], Bar { .. }));
    assert_eq!(bars[0].open, 174.0);
    assert_eq!(bars[0].high, 174.84);
    assert_eq!(bars[1].close, 175.84);
}

#[tokio::test]
async fn test_get_stock_bars_multi_symbol() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "bars": {
                "AAPL": [{"t": "2022-03-09T05:00:00Z", "o": 161.51, "h": 163.41, "l": 159.41, "c": 162.95, "v": 88496480, "n": 700291, "vw": 161.942117}],
                "TSLA": [{"t": "2022-03-09T05:00:00Z", "o": 839.0, "h": 860.56, "l": 832.01, "c": 858.97, "v": 19227323, "n": 528531, "vw": 850.616587}]
            },
            "next_page_token": null
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockBarsRequest {
        symbols: vec!["AAPL".to_string(), "TSLA".to_string()],
        ..Default::default()
    };
    let barset = client.get_stock_bars(&req).await.unwrap();

    assert!(barset.contains_key("AAPL"));
    assert!(barset.contains_key("TSLA"));
    assert_eq!(barset["TSLA"][0].open, 839.0);
    assert_eq!(barset["AAPL"][0].low, 159.41);
}

#[tokio::test]
async fn test_get_stock_bars_empty_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{"bars": {}, "next_page_token": null}"#,
        ))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockBarsRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let barset = client.get_stock_bars(&req).await.unwrap();

    assert!(barset.is_empty());
}

#[tokio::test]
async fn test_get_stock_quotes() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/quotes"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "quotes": {
                "AAPL": [
                    {"t": "2022-03-09T09:00:00.000059Z", "ax": "K", "ap": 158.65, "as": 1, "bx": "Q", "bp": 159.52, "bs": 4, "c": ["R"], "z": "C"},
                    {"t": "2022-03-09T09:01:00.000059Z", "ax": "K", "ap": 158.80, "as": 1, "bx": "Q", "bp": 159.52, "bs": 4, "c": ["R"], "z": "C"}
                ]
            },
            "next_page_token": null
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockQuotesRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let quoteset = client.get_stock_quotes(&req).await.unwrap();

    assert!(quoteset.contains_key("AAPL"));
    let quotes = &quoteset["AAPL"];
    assert_eq!(quotes.len(), 2);
    assert!(matches!(quotes[0], Quote { .. }));
    assert_eq!(quotes[0].ask_price, 158.65);
    assert_eq!(quotes[0].bid_size, 4.0);
}

#[tokio::test]
async fn test_get_stock_trades() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/trades"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "trades": {
                "AAPL": [
                    {"t": "2022-03-09T05:00:02.183Z", "x": "D", "p": 159.07, "s": 1, "c": ["@", "T", "I"], "i": 151, "z": "C"},
                    {"t": "2022-03-09T05:00:16.91Z",  "x": "D", "p": 159.07, "s": 2, "c": ["@", "T", "I"], "i": 168, "z": "C"}
                ]
            },
            "next_page_token": null
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockTradesRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let tradeset = client.get_stock_trades(&req).await.unwrap();

    assert!(tradeset.contains_key("AAPL"));
    let trades = &tradeset["AAPL"];
    assert_eq!(trades.len(), 2);
    assert!(matches!(trades[0], Trade { .. }));
    assert_eq!(trades[0].price, 159.07);
    assert_eq!(trades[0].size, 1.0);
}

#[tokio::test]
async fn test_get_stock_latest_trade() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/trades/latest"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "trades": {
                "AAPL": {"t": "2022-03-18T14:02:09.722539521Z", "x": "D", "p": 161.2958, "s": 100, "c": ["@"], "i": 22730, "z": "C"}
            }
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockLatestRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let trades = client.get_stock_latest_trade(&req).await.unwrap();

    assert!(trades.contains_key("AAPL"));
    let trade = &trades["AAPL"];
    assert!(matches!(trade, Trade { .. }));
    assert_eq!(trade.price, 161.2958);
    assert_eq!(trade.size, 100.0);
}

#[tokio::test]
async fn test_get_stock_latest_trade_empty() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/trades/latest"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"trades": {}}"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockLatestRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let trades = client.get_stock_latest_trade(&req).await.unwrap();

    assert!(trades.is_empty());
}

#[tokio::test]
async fn test_get_stock_latest_quote() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/quotes/latest"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "quotes": {
                "AAPL": {"t": "2022-03-18T14:02:43.651613184Z", "ax": "P", "ap": 161.11, "as": 13, "bx": "K", "bp": 161.10, "bs": 2, "c": ["R"], "z": "C"}
            }
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockLatestRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let quotes = client.get_stock_latest_quote(&req).await.unwrap();

    assert!(quotes.contains_key("AAPL"));
    let quote = &quotes["AAPL"];
    assert!(matches!(quote, Quote { .. }));
    assert_eq!(quote.ask_price, 161.11);
    assert_eq!(quote.bid_size, 2.0);
}

#[tokio::test]
async fn test_get_stock_latest_bar() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/bars/latest"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "bars": {
                "SPY": {"t": "2022-07-26T20:50:00Z", "o": 392.18, "h": 392.18, "l": 392.18, "c": 392.18, "v": 2100, "n": 2, "vw": 392.18}
            }
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockLatestRequest {
        symbols: vec!["SPY".to_string()],
        ..Default::default()
    };
    let bars = client.get_stock_latest_bar(&req).await.unwrap();

    assert!(bars.contains_key("SPY"));
    let bar = &bars["SPY"];
    assert!(matches!(bar, Bar { .. }));
    assert_eq!(bar.open, 392.18);
}

#[tokio::test]
async fn test_get_stock_snapshot() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{
            "AAPL": {
                "latestTrade": {"t": "2022-03-18T14:33:58.448432206Z", "x": "D", "p": 161.1998, "s": 200, "c": ["@"], "i": 39884, "z": "C"},
                "latestQuote": {"t": "2022-03-18T14:33:58.547942Z", "ax": "K", "ap": 161.2, "as": 2, "bx": "K", "bp": 161.19, "bs": 5, "c": ["R"], "z": "C"},
                "minuteBar": {"t": "2022-03-18T14:32:00Z", "o": 161.595, "h": 161.63, "l": 161.31, "c": 161.365, "v": 195503, "n": 1880, "vw": 161.448073},
                "dailyBar": {"t": "2022-03-18T04:00:00Z", "o": 160.59, "h": 161.92, "l": 159.76, "c": 161.365, "v": 31749988, "n": 186143, "vw": 160.683364},
                "prevDailyBar": {"t": "2022-03-17T04:00:00Z", "o": 158.6, "h": 161.0, "l": 157.63, "c": 160.62, "v": 73839892, "n": 609067, "vw": 159.425082}
            }
        }"#))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockSnapshotRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let snapshots = client.get_stock_snapshot(&req).await.unwrap();

    assert!(snapshots.contains_key("AAPL"));
    let snap = &snapshots["AAPL"];
    assert!(matches!(snap, Snapshot { .. }));
    assert_eq!(snap.latest_trade.as_ref().unwrap().price, 161.1998);
    assert_eq!(snap.latest_quote.as_ref().unwrap().bid_size, 5.0);
    assert_eq!(snap.minute_bar.as_ref().unwrap().close, 161.365);
    assert_eq!(snap.daily_bar.as_ref().unwrap().volume, 31749988.0);
    assert_eq!(snap.prev_daily_bar.as_ref().unwrap().high, 161.0);
}

#[tokio::test]
async fn test_get_stock_snapshot_empty() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v2/stocks/snapshots"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&server)
        .await;

    let client = common::stock_client(&server.uri());
    let req = StockSnapshotRequest {
        symbols: vec!["AAPL".to_string()],
        ..Default::default()
    };
    let snapshots = client.get_stock_snapshot(&req).await.unwrap();

    assert!(snapshots.is_empty());
}
