//! Option market data (`/v1beta1/options/...`), checked against the optionbars, optiontrades,
//! optionlatestquotes, optionlatesttrades, optionsnapshots, optionchain and optionmetaexchanges
//! references. Fixtures are their 200 response examples.

mod common;

use alpacars::data::historical::option::{
    OptionBarsRequest, OptionLatestRequest, OptionSnapshotRequest, OptionTradesRequest,
};
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Serve `first` (whose `next_page_token` is `token`) when no page_token is sent, and `last`
/// when `page_token == token`: the client must follow the token and merge both pages.
async fn mount_two_pages(server: &MockServer, p: &str, token: &str, first: &str, last: &str) {
    Mock::given(method("GET"))
        .and(path(p))
        .and(query_param_is_missing("page_token"))
        .respond_with(ResponseTemplate::new(200).set_body_string(first))
        .expect(1)
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path(p))
        .and(query_param("page_token", token))
        .respond_with(ResponseTemplate::new(200).set_body_string(last))
        .expect(1)
        .mount(server)
        .await;
}

async fn mount_one(server: &MockServer, p: &str, body: &str) {
    Mock::given(method("GET"))
        .and(path(p))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .expect(1)
        .mount(server)
        .await;
}

fn syms(s: &[&str]) -> Vec<String> {
    s.iter().map(|x| x.to_string()).collect()
}

#[tokio::test]
async fn test_get_option_bars_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(
        &server,
        "/v1beta1/options/bars",
        "QUFQTHxNfDIwMjItMDEtMDNUMDk6MDA6MDAuMDAwMDAwMDAwWg==",
        r#"{
  "bars": {
    "AAPL240419P00140000": [
      {
        "c": 0.34,
        "h": 0.38,
        "l": 0.34,
        "n": 7,
        "o": 0.38,
        "t": "2024-01-18T05:00:00Z",
        "v": 12,
        "vw": 0.3525
      }
    ]
  },
  "next_page_token": "QUFQTHxNfDIwMjItMDEtMDNUMDk6MDA6MDAuMDAwMDAwMDAwWg=="
}"#,
        r#"{
  "bars": {
    "AAPL240419P00140000": [
      {
        "c": 0.34,
        "h": 0.38,
        "l": 0.34,
        "n": 7,
        "o": 0.38,
        "t": "2024-01-18T05:00:00Z",
        "v": 12,
        "vw": 0.3525
      }
    ]
  },
  "next_page_token": null
}"#,
    )
    .await;
    let client = common::option_client(&server.uri());
    let req = OptionBarsRequest {
        symbols: syms(&["AAPL240419P00140000"]),
        ..Default::default()
    };
    let bars = client.get_option_bars(&req).await.unwrap();
    assert!(bars["AAPL240419P00140000"]
        .iter()
        .all(|b| b.symbol == "AAPL240419P00140000"));
    assert_eq!(bars["AAPL240419P00140000"].len(), 2);
}

#[tokio::test]
async fn test_get_option_trades_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(&server, "/v1beta1/options/trades", "QUFQTHwyMDIyLTAxLTAzVDA5OjAwOjAwLjI0NDgzOTY4MFp8UHwwOTIyMzM3MjAzNjg1NDc3NTgxMA==", r#"{
  "next_page_token": "QUFQTHwyMDIyLTAxLTAzVDA5OjAwOjAwLjI0NDgzOTY4MFp8UHwwOTIyMzM3MjAzNjg1NDc3NTgxMA==",
  "trades": {
    "AAPL240112C00182500": [
      {
        "c": "I",
        "p": 0.37,
        "s": 1,
        "t": "2024-01-18T15:03:44.56339456Z",
        "x": "B"
      },
      {
        "c": "g",
        "p": 0.34,
        "s": 1,
        "t": "2024-01-18T16:02:38.994758144Z",
        "x": "C"
      }
    ]
  }
}"#, r#"{
  "next_page_token": null,
  "trades": {
    "AAPL240112C00182500": [
      {
        "c": "I",
        "p": 0.37,
        "s": 1,
        "t": "2024-01-18T15:03:44.56339456Z",
        "x": "B"
      },
      {
        "c": "g",
        "p": 0.34,
        "s": 1,
        "t": "2024-01-18T16:02:38.994758144Z",
        "x": "C"
      }
    ]
  }
}"#).await;
    let client = common::option_client(&server.uri());
    let req = OptionTradesRequest {
        symbols: syms(&["AAPL240112C00182500"]),
        ..Default::default()
    };
    let trades = client.get_option_trades(&req).await.unwrap();
    assert!(trades["AAPL240112C00182500"]
        .iter()
        .all(|t| t.symbol == "AAPL240112C00182500"));
    assert_eq!(trades["AAPL240112C00182500"].len(), 4);
}

#[tokio::test]
async fn test_get_option_latest_quote() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta1/options/quotes/latest",
        r#"{
  "quotes": {
    "AAPL240419P00140000": {
      "ap": 0.16,
      "as": 669,
      "ax": "w",
      "bp": 0.15,
      "bs": 164,
      "bx": "W",
      "c": "A",
      "t": "2024-02-28T15:30:28.046330624Z"
    },
    "AAPL250321C00190000": {
      "ap": 17,
      "as": 622,
      "ax": "X",
      "bp": 16.75,
      "bs": 368,
      "bx": "X",
      "c": " ",
      "t": "2024-02-28T15:47:13.663636224Z"
    }
  }
}"#,
    )
    .await;
    let client = common::option_client(&server.uri());
    let req = OptionLatestRequest {
        symbols: syms(&["AAPL240419P00140000"]),
        ..Default::default()
    };
    let quotes = client.get_option_latest_quote(&req).await.unwrap();
    assert_eq!(quotes["AAPL240419P00140000"].symbol, "AAPL240419P00140000");
}

#[tokio::test]
async fn test_get_option_latest_trade() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta1/options/trades/latest",
        r#"{
  "trades": {
    "AAPL250321C00190000": {
      "c": "e",
      "p": 17.15,
      "s": 900,
      "t": "2024-02-28T15:26:12.728701696Z",
      "x": "B"
    }
  }
}"#,
    )
    .await;
    let client = common::option_client(&server.uri());
    let req = OptionLatestRequest {
        symbols: syms(&["AAPL250321C00190000"]),
        ..Default::default()
    };
    let trades = client.get_option_latest_trade(&req).await.unwrap();
    assert_eq!(trades["AAPL250321C00190000"].symbol, "AAPL250321C00190000");
}

#[tokio::test]
async fn test_get_option_snapshot_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(
        &server,
        "/v1beta1/options/snapshots",
        "QUFQTDI0MDYyMTAwMDIyMHwx",
        r#"{
  "next_page_token": "QUFQTDI0MDYyMTAwMDIyMHwx",
  "snapshots": {
    "AAPL240426C00162500": {
      "greeks": {
        "delta": 0.7521304109871954,
        "gamma": 0.06241426404871288,
        "rho": 0.009910739032549095,
        "theta": -0.2847623059595503,
        "vega": 0.047540520834498785
      },
      "impliedVolatility": 0.3372405712050441,
      "latestQuote": {
        "ap": 4.3,
        "as": 91,
        "ax": "B",
        "bp": 4.15,
        "bs": 16,
        "bx": "C",
        "c": "A",
        "t": "2024-04-22T19:59:59.992734208Z"
      },
      "latestTrade": {
        "c": "I",
        "p": 4.1,
        "s": 1,
        "t": "2024-04-22T19:57:32.589554432Z",
        "x": "A"
      }
    }
  }
}"#,
        r#"{
  "next_page_token": null,
  "snapshots": {
    "AAPL240426C00162500": {
      "greeks": {
        "delta": 0.7521304109871954,
        "gamma": 0.06241426404871288,
        "rho": 0.009910739032549095,
        "theta": -0.2847623059595503,
        "vega": 0.047540520834498785
      },
      "impliedVolatility": 0.3372405712050441,
      "latestQuote": {
        "ap": 4.3,
        "as": 91,
        "ax": "B",
        "bp": 4.15,
        "bs": 16,
        "bx": "C",
        "c": "A",
        "t": "2024-04-22T19:59:59.992734208Z"
      },
      "latestTrade": {
        "c": "I",
        "p": 4.1,
        "s": 1,
        "t": "2024-04-22T19:57:32.589554432Z",
        "x": "A"
      }
    }
  }
}"#,
    )
    .await;
    let client = common::option_client(&server.uri());
    let req = OptionSnapshotRequest {
        symbols: syms(&["AAPL240426C00162500"]),
        ..Default::default()
    };
    let snaps = client.get_option_snapshot(&req).await.unwrap();
    let snap = &snaps["AAPL240426C00162500"];
    assert_eq!(snap.symbol, "AAPL240426C00162500");
    if let Some(t) = &snap.latest_trade {
        assert_eq!(t.symbol, "AAPL240426C00162500");
    }
    assert!(snaps.contains_key("AAPL240426C00162500"));
}

/// The chain endpoint is `/options/snapshots/{underlying_symbol}`. The crate called
/// `/options/snapshots?underlying_symbols=…`, an endpoint that requires contract `symbols`.
#[tokio::test]
async fn test_get_option_chain_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1beta1/options/snapshots/AAPL"))
        .and(query_param_is_missing("underlying_symbols"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
  "next_page_token": null,
  "snapshots": {
    "AAPL240426C00162500": {
      "greeks": {
        "delta": 0.7521304109871954,
        "gamma": 0.06241426404871288,
        "rho": 0.009910739032549095,
        "theta": -0.2847623059595503,
        "vega": 0.047540520834498785
      },
      "impliedVolatility": 0.3372405712050441,
      "latestQuote": {
        "ap": 4.3,
        "as": 91,
        "ax": "B",
        "bp": 4.15,
        "bs": 16,
        "bx": "C",
        "c": "A",
        "t": "2024-04-22T19:59:59.992734208Z"
      },
      "latestTrade": {
        "c": "I",
        "p": 4.1,
        "s": 1,
        "t": "2024-04-22T19:57:32.589554432Z",
        "x": "A"
      }
    }
  }
}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::option_client(&server.uri());
    let chain = client.get_option_chain("AAPL", None).await.unwrap();
    assert!(chain["snapshots"].is_object());
}

#[tokio::test]
async fn test_get_option_exchange_codes() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta1/options/meta/exchanges",
        r#"{"A": "AMEX - NYSE American", "C": "CBOE"}"#,
    )
    .await;
    let client = common::option_client(&server.uri());
    let codes = client.get_option_exchange_codes().await.unwrap();
    assert_eq!(codes["C"], "CBOE");
}
