//! Crypto market data (`/v1beta3/crypto/{loc}/...`), checked against the cryptobars-1 ...
//! cryptosnapshots-1 references. Fixtures are the references' 200 response examples.

mod common;

use alpacars::data::enums::CryptoFeed;
use alpacars::data::historical::crypto::{
    CryptoBarsRequest, CryptoLatestRequest, CryptoQuotesRequest, CryptoSnapshotRequest,
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
async fn test_get_crypto_bars_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(
        &server,
        "/v1beta3/crypto/us/bars",
        "MTY0MDk0ODkyMzAwMDAwMDAwMHwyNDg0MzE3MQ==",
        r#"{
  "bars": {
    "BTC/USD": [
      {
        "c": 29003,
        "h": 29003,
        "l": 28999,
        "n": 4,
        "o": 28999,
        "t": "2022-05-27T10:18:00Z",
        "v": 0.01,
        "vw": 29001
      }
    ]
  },
  "next_page_token": "MTY0MDk0ODkyMzAwMDAwMDAwMHwyNDg0MzE3MQ=="
}"#,
        r#"{
  "bars": {
    "BTC/USD": [
      {
        "c": 29003,
        "h": 29003,
        "l": 28999,
        "n": 4,
        "o": 28999,
        "t": "2022-05-27T10:18:00Z",
        "v": 0.01,
        "vw": 29001
      }
    ]
  },
  "next_page_token": null
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoBarsRequest {
        symbols: syms(&["BTC/USD"]),
        ..Default::default()
    };
    let bars = client.get_crypto_bars(&req).await.unwrap();
    assert!(bars["BTC/USD"].iter().all(|b| b.symbol == "BTC/USD"));
    assert_eq!(bars["BTC/USD"].len(), 2);
}

/// `loc` selects the path segment; it is not sent as a query parameter.
#[tokio::test]
async fn test_get_crypto_bars_uses_loc_in_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/eu-1/bars"))
        .and(query_param_is_missing("loc"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"{
  "bars": {
    "BTC/USD": [
      {
        "c": 29003,
        "h": 29003,
        "l": 28999,
        "n": 4,
        "o": 28999,
        "t": "2022-05-27T10:18:00Z",
        "v": 0.01,
        "vw": 29001
      }
    ]
  },
  "next_page_token": null
}"#,
        ))
        .expect(1)
        .mount(&server)
        .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoBarsRequest {
        symbols: syms(&["BTC/USD"]),
        loc: Some(CryptoFeed::Eu1),
        ..Default::default()
    };
    client.get_crypto_bars(&req).await.unwrap();
}

#[tokio::test]
async fn test_get_crypto_quotes_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(
        &server,
        "/v1beta3/crypto/us/quotes",
        "MTY1MzU2NzYzODQ5OTQ3ODI3Mnwx",
        r#"{
  "next_page_token": "MTY1MzU2NzYzODQ5OTQ3ODI3Mnwx",
  "quotes": {
    "BTC/USD": [
      {
        "ap": 29059,
        "as": 3.252,
        "bp": 29058,
        "bs": 0.3544,
        "t": "2022-05-26T11:47:18.44347136Z"
      }
    ],
    "ETH/USD": [
      {
        "ap": 1817.7,
        "as": 6.137,
        "bp": 1817,
        "bs": 4.76,
        "t": "2022-05-26T11:47:18.499478272Z"
      }
    ]
  }
}"#,
        r#"{
  "next_page_token": null,
  "quotes": {
    "BTC/USD": [
      {
        "ap": 29059,
        "as": 3.252,
        "bp": 29058,
        "bs": 0.3544,
        "t": "2022-05-26T11:47:18.44347136Z"
      }
    ],
    "ETH/USD": [
      {
        "ap": 1817.7,
        "as": 6.137,
        "bp": 1817,
        "bs": 4.76,
        "t": "2022-05-26T11:47:18.499478272Z"
      }
    ]
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoQuotesRequest {
        symbols: syms(&["BTC/USD"]),
        ..Default::default()
    };
    let quotes = client.get_crypto_quotes(&req).await.unwrap();
    assert!(quotes["BTC/USD"].iter().all(|q| q.symbol == "BTC/USD"));
    assert_eq!(quotes["BTC/USD"].len(), 2);
}

#[tokio::test]
async fn test_get_crypto_trades_paginates() {
    let server = MockServer::start().await;
    mount_two_pages(
        &server,
        "/v1beta3/crypto/us/trades",
        "MTY1MzA1MDgwMDAwMDAwMDAwMHwzMTQ1NTI4OQ==",
        r#"{
  "next_page_token": "MTY1MzA1MDgwMDAwMDAwMDAwMHwzMTQ1NTI4OQ==",
  "trades": {
    "BTC/USD": [
      {
        "i": 31455289,
        "p": 29791,
        "s": 0.0016,
        "t": "2022-05-18T12:01:00.537052Z",
        "tks": "S"
      }
    ],
    "ETH/USD": [
      {
        "i": 31455287,
        "p": 2027.6,
        "s": 0.06,
        "t": "2022-05-18T12:01:00.363547Z",
        "tks": "S"
      },
      {
        "i": 31455288,
        "p": 2027.6,
        "s": 0.136,
        "t": "2022-05-18T12:01:00.363547Z",
        "tks": "S"
      }
    ]
  }
}"#,
        r#"{
  "next_page_token": null,
  "trades": {
    "BTC/USD": [
      {
        "i": 31455289,
        "p": 29791,
        "s": 0.0016,
        "t": "2022-05-18T12:01:00.537052Z",
        "tks": "S"
      }
    ],
    "ETH/USD": [
      {
        "i": 31455287,
        "p": 2027.6,
        "s": 0.06,
        "t": "2022-05-18T12:01:00.363547Z",
        "tks": "S"
      },
      {
        "i": 31455288,
        "p": 2027.6,
        "s": 0.136,
        "t": "2022-05-18T12:01:00.363547Z",
        "tks": "S"
      }
    ]
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoQuotesRequest {
        symbols: syms(&["BTC/USD"]),
        ..Default::default()
    };
    let trades = client.get_crypto_trades(&req).await.unwrap();
    assert!(trades["BTC/USD"].iter().all(|t| t.symbol == "BTC/USD"));
    assert_eq!(trades["BTC/USD"].len(), 2);
}

#[tokio::test]
async fn test_get_crypto_latest_bar() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta3/crypto/us/latest/bars",
        r#"{
  "bars": {
    "BTC/USD": {
      "c": 29003,
      "h": 29003,
      "l": 28999,
      "n": 4,
      "o": 28999,
      "t": "2022-05-27T10:18:00Z",
      "v": 0.01,
      "vw": 29001
    }
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoLatestRequest {
        symbols: syms(&["BTC/USD"]),
        loc: None,
    };
    let set = client.get_crypto_latest_bar(&req).await.unwrap();
    assert_eq!(set["BTC/USD"].symbol, "BTC/USD");
    assert!(set.contains_key("BTC/USD"));
}

#[tokio::test]
async fn test_get_crypto_latest_quote() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta3/crypto/us/latest/quotes",
        r#"{
  "quotes": {
    "BTC/USD": {
      "ap": 29059,
      "as": 3.252,
      "bp": 29058,
      "bs": 0.3544,
      "t": "2022-05-26T11:47:18.44347136Z"
    },
    "ETH/USD": {
      "ap": 1817.7,
      "as": 6.137,
      "bp": 1817,
      "bs": 4.76,
      "t": "2022-05-26T11:47:18.499478272Z"
    }
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoLatestRequest {
        symbols: syms(&["BTC/USD"]),
        loc: None,
    };
    let set = client.get_crypto_latest_quote(&req).await.unwrap();
    assert_eq!(set["BTC/USD"].symbol, "BTC/USD");
    assert!(set.contains_key("BTC/USD"));
}

#[tokio::test]
async fn test_get_crypto_latest_trade() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta3/crypto/us/latest/trades",
        r#"{
  "trades": {
    "BTC/USD": {
      "i": 31455289,
      "p": 29791,
      "s": 0.0016,
      "t": "2022-05-18T12:01:00.537052Z",
      "tks": "S"
    },
    "ETH/USD": {
      "i": 31455287,
      "p": 2027.6,
      "s": 0.06,
      "t": "2022-05-18T12:01:00.363547Z",
      "tks": "S"
    }
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoLatestRequest {
        symbols: syms(&["BTC/USD"]),
        loc: None,
    };
    let set = client.get_crypto_latest_trade(&req).await.unwrap();
    assert_eq!(set["BTC/USD"].symbol, "BTC/USD");
    assert!(set.contains_key("BTC/USD"));
}

#[tokio::test]
async fn test_get_crypto_latest_orderbook() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta3/crypto/us/latest/orderbooks",
        r#"{
  "orderbooks": {
    "BTC/USD": {
      "a": [
        {
          "p": 20902,
          "s": 0.0097
        },
        {
          "p": 21444,
          "s": 0
        }
      ],
      "b": [
        {
          "p": 20846,
          "s": 0.1902
        },
        {
          "p": 20350,
          "s": 0
        }
      ],
      "t": "2022-06-24T08:00:14.137774336Z"
    }
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoLatestRequest {
        symbols: syms(&["BTC/USD"]),
        loc: None,
    };
    let set = client.get_crypto_latest_orderbook(&req).await.unwrap();
    assert_eq!(set["BTC/USD"].symbol, "BTC/USD");
    assert!(set.contains_key("BTC/USD"));
}

#[tokio::test]
async fn test_get_crypto_snapshot() {
    let server = MockServer::start().await;
    mount_one(
        &server,
        "/v1beta3/crypto/us/snapshots",
        r#"{
  "snapshots": {
    "BTC/USD": {
      "dailyBar": {
        "c": 31744,
        "h": 31807,
        "l": 31416,
        "n": 438,
        "o": 31660,
        "t": "2022-05-31T05:00:00Z",
        "v": 67.3518,
        "vw": 31582.7034526175
      },
      "latestQuote": {
        "ap": 31742,
        "as": 0.395,
        "bp": 31741,
        "bs": 0.395,
        "t": "2022-05-31T11:55:58.507608832Z"
      },
      "latestTrade": {
        "i": 32396097,
        "p": 31744,
        "s": 0.0543,
        "t": "2022-05-31T11:53:45.027481Z",
        "tks": "B"
      },
      "minuteBar": {
        "c": 31744,
        "h": 31744,
        "l": 31744,
        "n": 2,
        "o": 31744,
        "t": "2022-05-31T11:53:00Z",
        "v": 0.0886,
        "vw": 31744
      },
      "prevDailyBar": {
        "c": 31649,
        "h": 32251,
        "l": 30251,
        "n": 8221,
        "o": 30310,
        "t": "2022-05-30T05:00:00Z",
        "v": 1856.4065,
        "vw": 30877.2751897281
      }
    }
  }
}"#,
    )
    .await;
    let client = common::crypto_client(&server.uri());
    let req = CryptoSnapshotRequest {
        symbols: syms(&["BTC/USD"]),
        loc: None,
    };
    let set = client.get_crypto_snapshot(&req).await.unwrap();
    assert_eq!(set["BTC/USD"].symbol, "BTC/USD");
    if let Some(q) = &set["BTC/USD"].latest_quote {
        assert_eq!(q.symbol, "BTC/USD");
    }
    assert!(set.contains_key("BTC/USD"));
}

/// cryptoquotes-1 / cryptotrades-1: `loc` is a path segment, as for bars.
#[tokio::test]
async fn test_get_crypto_quotes_and_trades_use_loc() {
    let server = MockServer::start().await;
    mount_one(&server, "/v1beta3/crypto/us-1/quotes", r#"{"quotes": {}}"#).await;
    mount_one(&server, "/v1beta3/crypto/eu-1/trades", r#"{"trades": {}}"#).await;
    let client = common::crypto_client(&server.uri());
    let quotes = CryptoQuotesRequest {
        symbols: syms(&["BTC/USD"]),
        loc: Some(CryptoFeed::Us1),
        ..Default::default()
    };
    client.get_crypto_quotes(&quotes).await.unwrap();
    let trades = CryptoQuotesRequest {
        symbols: syms(&["BTC/USD"]),
        loc: Some(CryptoFeed::Eu1),
        ..Default::default()
    };
    client.get_crypto_trades(&trades).await.unwrap();
}
