# 🦙 alpacars

> An async-first Rust SDK for the [Alpaca Markets](https://alpaca.markets) API — a Rust equivalent of the official [alpaca-py](https://github.com/alpacahq/alpaca-py) Python SDK.

[![CI](https://github.com/kfuangsung/alpacars/actions/workflows/ci.yml/badge.svg)](https://github.com/kfuangsung/alpacars/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/alpacars.svg)](https://crates.io/crates/alpacars)
[![docs.rs](https://img.shields.io/docsrs/alpacars)](https://docs.rs/alpacars)
[![Rust](https://img.shields.io/badge/rust-1.88%2B-orange?logo=rust)](https://www.rust-lang.org)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#-license)

---

## 📚 Table of Contents

- [About](#about)
- [✨ Features](#-features)
- [📦 Installation](#-installation)
- [🔑 API Keys](#-api-keys)
- [🚀 Usage](#-usage)
  - [Trading API](#trading-api)
  - [Broker API](#broker-api)
  - [Market Data — Historical](#market-data--historical)
  - [Market Data — Live Streaming](#market-data--live-streaming)
- [▶️ Examples](#️-examples)
- [🧪 Running Tests](#-running-tests)
- [🗂️ Supported Clients](#️-supported-clients)
- [📝 Changelog](CHANGELOG.md)
- [📄 License](#-license)

---

## About

`alpacars` provides a fully async interface to all Alpaca API products — **Trading**, **Broker**, and **Market Data** (historical + live streaming). Every network call is `async` and returns a `Result<T, AlpacaError>`, making it easy to integrate into any `tokio`-based application.

The design mirrors `alpaca-py` closely so that anyone familiar with the Python SDK can pick up the Rust SDK immediately, while taking full advantage of Rust's **type system**, **ownership model**, and **zero-cost abstractions**.

---

## ✨ Features

| | Feature | Details |
|---|---|---|
| ⚡ | **Async by default** | Built on `tokio` and `reqwest` — no blocking calls |
| 📈 | **Trading API** | Orders (market, limit, stop, bracket, OTO, OCO, trailing-stop, multi-leg, DMA routing), positions, assets, watchlists, options contracts, locates (hard-to-borrow), tokenization |
| 🏦 | **Broker API** | Account management, documents, ACH/bank transfers, journals, portfolio rebalancing, subscriptions, tokenization |
| 🪙 | **Tokenization** | Mint tokenized assets and look up requests by Alpaca, client, or issuer ID — on both Trading and Broker |
| 🕰️ | **Historical market data** | Stocks, crypto, options, news, screener (most actives, movers) — all endpoints auto-paginate |
| 📡 | **Live streaming** | Stocks, crypto, options, and news WebSocket streams with per-event-type handler callbacks; JSON and msgpack frames |
| 🔐 | **Three auth modes** | API key headers, OAuth Bearer token, HTTP Basic (Broker) |
| 🔄 | **Retry with exponential backoff** | Automatic retry on HTTP 429 / 504 — wait doubles each attempt (3 s → 6 s → 12 s), capped at 60 s |
| 🔍 | **Structured observability** | `tracing`-instrumented throughout — attach any `tracing-subscriber` to get request logs, retry events, and stream warnings |
| 🧪 | **Paper & live** | Single flag switches between paper trading and live environments |

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpacars = "0.5"
```

> **Minimum Rust version:** 1.88 — enforced by a dedicated CI job.

> **Pre-1.0 versioning:** a `0.x` **minor** bump is the breaking-change bump. Check [CHANGELOG.md](CHANGELOG.md) before upgrading across one.

---

## 🔑 API Keys

Set your credentials as environment variables:

```bash
export APCA_API_KEY_ID="your-api-key"
export APCA_API_SECRET_KEY="your-secret-key"
```

> For Broker API, use your Broker API key and secret. For OAuth, pass the token directly to the client constructor.

Obtain API keys from the [Alpaca dashboard](https://app.alpaca.markets). 🖥️

---

## 🚀 Usage

### Trading API

```rust
use alpacars::trading::client::TradingClient;
use alpacars::trading::enums::{OrderSide, TimeInForce};
use alpacars::trading::requests::OrderRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TradingClient::new("API_KEY", "SECRET_KEY", true)?; // paper = true

    // 💰 Account info
    let account = client.get_account().await?;
    println!("Buying power: {:?}", account.buying_power);

    // 📬 Submit a market order
    let order = client
        .submit_order(&OrderRequest::market("SPY", OrderSide::Buy, "5"))
        .await?;
    println!("Order ID: {}", order.id);

    // 📋 Submit a limit order
    let limit = client
        .submit_order(&OrderRequest::limit(
            "AAPL", OrderSide::Buy, "10", "175.00", TimeInForce::Day,
        ))
        .await?;
    println!("Limit order status: {:?}", limit.status);

    // 📊 List open positions
    let positions = client.get_all_positions().await?;
    println!("Open positions: {}", positions.len());

    // 🗑️ Cancel all open orders
    client.cancel_orders().await?;

    Ok(())
}
```

### Broker API

```rust
use alpacars::broker::client::BrokerClient;
use alpacars::broker::requests::ListAccountsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 🏦 Broker API uses HTTP Basic auth
    let client = BrokerClient::new("BROKER_KEY", "BROKER_SECRET", true)?; // sandbox = true

    let accounts = client.list_accounts(Some(&ListAccountsRequest::default())).await?;
    println!("Accounts: {}", accounts.len());

    Ok(())
}
```

### Market Data — Historical

```rust
use alpacars::data::historical::stock::{StockBarsRequest, StockHistoricalDataClient};
use alpacars::data::historical::crypto::{CryptoBarsRequest, CryptoHistoricalDataClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📈 Stock bars
    let stock = StockHistoricalDataClient::new(Some("KEY"), Some("SECRET"), false)?;

    let bars = stock
        .get_stock_bars(&StockBarsRequest {
            symbols: vec!["AAPL".to_string(), "TSLA".to_string()],
            ..Default::default()
        })
        .await?;

    for (symbol, sym_bars) in &bars {
        println!("{}: {} bars, latest close = {}",
            symbol, sym_bars.len(),
            sym_bars.last().map(|b| b.close).unwrap_or(0.0));
    }

    // ₿ Crypto bars
    let crypto = CryptoHistoricalDataClient::new(Some("KEY"), Some("SECRET"))?;

    let crypto_bars = crypto
        .get_crypto_bars(&CryptoBarsRequest {
            symbols: vec!["BTC/USD".to_string()],
            ..Default::default()
        })
        .await?;

    println!("BTC/USD bars: {}",
        crypto_bars.get("BTC/USD").map(|b| b.len()).unwrap_or(0));

    Ok(())
}
```

### Market Data — Live Streaming

```rust
use alpacars::data::live::stock::StockDataStream;
use alpacars::data::enums::DataFeed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 📡 Connect to live stock stream
    let mut stream = StockDataStream::new("API_KEY", "SECRET_KEY", DataFeed::Iex);

    stream.subscribe_trades(["AAPL", "TSLA"], |trade| {
        println!("🔔 Trade: {} @ ${} x {}", trade.symbol, trade.price, trade.size);
    });

    stream.subscribe_quotes(["AAPL"], |quote| {
        println!("💬 Quote: {} bid={} ask={}", quote.symbol, quote.bid_price, quote.ask_price);
    });

    stream.run().await?; // blocks until stream ends or error

    Ok(())
}
```

Real-time trade updates (fills, order status changes):

```rust
use alpacars::trading::stream::TradingStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TradingStream::new("API_KEY", "SECRET_KEY", true); // paper

    stream.subscribe_trade_updates(|update| {
        println!("⚡ Event: {:?}", update.event);
    });

    stream.run().await?;

    Ok(())
}
```

---

## ▶️ Examples

Four runnable examples are in `examples/`. Set your credentials, then:

```bash
# 📈 Stock trading — orders, positions, historical data
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example stocks_trading_basic

# ₿ Crypto trading — BTC/USD orders, positions, crypto data
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example crypto_trading_basic

# 🎯 Options trading — contract discovery, orders, exercise, snapshots
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example options_trading_basic

# 🦋 Multi-leg options — straddle and iron condor
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example options_trading_mleg
```

---

## 🧪 Running Tests

Tests use [`wiremock`](https://crates.io/crates/wiremock) to mock HTTP responses — **no real API credentials needed**.

```bash
cargo test
```

57 tests across 9 files, all passing. Every mocked request asserts the exact path and HTTP verb, not just the response shape — so a client pointed at the wrong endpoint fails loudly rather than quietly deserializing a canned body.

CI additionally runs the suite on **macOS and Windows**, checks formatting and clippy (`-D warnings`), builds the docs, verifies the crate packages, scans dependencies for advisories and license violations, and checks every PR for breaking API changes. To reproduce that locally, see [CLAUDE.md](CLAUDE.md#reproducing-ci-locally).

---

## 🗂️ Supported Clients

| Client | Domain | Description |
|---|---|---|
| 📊 `TradingClient` | Trading | Orders, positions, assets, watchlists, options, locates, tokenization, corporate actions |
| 🔔 `TradingStream` | Trading | Real-time trade updates via WebSocket |
| 🏦 `BrokerClient` | Broker | Account management, funding, journals, rebalancing, tokenization |
| 📈 `StockHistoricalDataClient` | Data | Bars, quotes, trades, snapshots (auto-paginated) |
| ₿ `CryptoHistoricalDataClient` | Data | Bars, quotes, trades, orderbook, snapshots |
| 🎯 `OptionHistoricalDataClient` | Data | Bars, trades, quotes, snapshots, option chains |
| 📰 `NewsClient` | Data | News articles with optional full content |
| 🔍 `ScreenerClient` | Data | Most actives, market movers |
| 🏢 `CorporateActionsClient` | Data | Splits, dividends, spin-offs |
| 📡 `StockDataStream` | Streaming | Real-time stock trades, quotes, bars, statuses |
| 📡 `CryptoDataStream` | Streaming | Real-time crypto trades, quotes, bars, orderbooks |
| 📡 `OptionDataStream` | Streaming | Real-time options trades and quotes |
| 📡 `NewsDataStream` | Streaming | Real-time news events |

---

## ⚠️ Disclaimer

**PLEASE READ CAREFULLY BEFORE USE.**

`alpacars` is an **independent, unofficial, community-maintained** Rust SDK. It is not affiliated with, endorsed by, sponsored by, or in any way officially connected to [Alpaca Markets](https://alpaca.markets) or any of its subsidiaries. All trademarks, service marks, and brand names are the property of their respective owners.

### No Financial Advice

Nothing in this software, its documentation, or its source code constitutes financial, investment, legal, or tax advice. This library is provided for **educational and informational purposes only**. Any trading strategies or examples shown are purely illustrative and do not represent a recommendation to trade any particular instrument or strategy.

### Risk Warning

Trading financial instruments — including equities, options, cryptocurrencies, and other derivatives — involves **substantial risk of loss** and is not appropriate for all investors. Automated trading systems can malfunction, execute unintended orders, or behave unexpectedly under real market conditions.

**You may lose some or all of your invested capital. Past performance is not indicative of future results.**

### No Warranty or Liability

This software is provided **"as is"**, without warranty of any kind, express or implied, including but not limited to warranties of merchantability, fitness for a particular purpose, or non-infringement. In no event shall the authors or contributors be liable for any direct, indirect, incidental, special, exemplary, or consequential damages (including but not limited to loss of profits, loss of data, missed trades, or financial loss) arising out of or in connection with the use of this software, even if advised of the possibility of such damages.

### Your Responsibility

By using this software you acknowledge that:

- You are solely responsible for all trading decisions and their outcomes.
- You have read and agree to [Alpaca's Terms of Service](https://alpaca.markets/terms) and all applicable regulations in your jurisdiction.
- You will **paper-trade thoroughly** and validate all behaviour before deploying with real capital.
- Automated trading may be subject to regulatory requirements in your country — it is your responsibility to ensure compliance.

---

## 📄 License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
