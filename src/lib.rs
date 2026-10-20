//! An async Rust SDK for the [Alpaca](https://alpaca.markets) Trading, Broker and Market Data
//! APIs, shaped after the official [`alpaca-py`](https://github.com/alpacahq/alpaca-py) SDK.
//!
//! Every network call is `async` (on `tokio`) and returns `Result<T, AlpacaError>`.
//!
//! # Clients
//!
//! | API | Client | Module |
//! |---|---|---|
//! | Trading: orders, positions, assets, watchlists, options, calendar | [`TradingClient`](trading::client::TradingClient) | [`trading`] |
//! | Trading: live order updates over WebSocket | [`TradingStream`](trading::stream::TradingStream) | [`trading::stream`] |
//! | Broker: accounts, funding, journals, documents, rebalancing, ACATS | [`BrokerClient`](broker::client::BrokerClient) | [`broker`] |
//! | Historical stocks, crypto, options, news, screener, corporate actions | `StockHistoricalDataClient`, `CryptoHistoricalDataClient`, … | [`data::historical`] |
//! | Live stocks, crypto, options, news over WebSocket | `StockDataStream`, `CryptoDataStream`, … | [`data::live`] |
//!
//! Each module follows the same layout: `client` (the methods), `requests` (request bodies and
//! query filters), `models` (responses) and `enums`.
//!
//! # Environments and authentication
//!
//! - **Trading:** `TradingClient::new(key, secret, paper)` sends API-key headers to the paper
//!   (`paper = true`) or live API. [`TradingClient::with_oauth`](trading::client::TradingClient::with_oauth)
//!   uses an OAuth bearer token instead.
//! - **Broker:** `BrokerClient::new(key, secret, sandbox)` uses HTTP Basic auth against the
//!   sandbox (`sandbox = true`) or production Broker API.
//! - **Market data:** keys are optional for the crypto clients; the stock client takes a
//!   `sandbox` flag.
//!
//! Requests that fail with HTTP 429 or 504 are retried with exponential backoff (3 s, 6 s, 12 s, …,
//! capped at 60 s). Historical bars, quotes and trades are paginated for you: one call returns
//! the whole range.
//!
//! # Trading
//!
//! ```no_run
//! use alpacars::trading::client::TradingClient;
//! use alpacars::trading::enums::{OrderSide, QueryOrderStatus};
//! use alpacars::trading::requests::{GetOrdersRequest, OrderRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), alpacars::AlpacaError> {
//!     let client = TradingClient::new("API_KEY", "SECRET_KEY", true)?; // paper
//!
//!     let account = client.get_account().await?;
//!     println!("buying power: {:?}", account.buying_power);
//!
//!     let order = client
//!         .submit_order(&OrderRequest::market("AAPL", OrderSide::Buy, "1"))
//!         .await?;
//!     println!("submitted {}", order.id);
//!
//!     // Request types implement `Default`, so set only the filters you need.
//!     let open = client
//!         .get_orders(Some(&GetOrdersRequest {
//!             status: Some(QueryOrderStatus::Open),
//!             ..Default::default()
//!         }))
//!         .await?;
//!     println!("{} open orders", open.len());
//!     Ok(())
//! }
//! ```
//!
//! # Broker
//!
//! ```no_run
//! use alpacars::broker::client::BrokerClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), alpacars::AlpacaError> {
//!     let broker = BrokerClient::new("BROKER_KEY", "BROKER_SECRET", true)?; // sandbox
//!
//!     for account in broker.list_accounts(None).await? {
//!         let banks = broker.get_banks_for_account(&account.id, None).await?;
//!         println!("{}: {} recipient banks", account.id, banks.len());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Historical market data
//!
//! ```no_run
//! use alpacars::data::historical::stock::{StockBarsRequest, StockHistoricalDataClient};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), alpacars::AlpacaError> {
//!     let data = StockHistoricalDataClient::new(Some("API_KEY"), Some("SECRET_KEY"), false)?;
//!
//!     let bars = data
//!         .get_stock_bars(&StockBarsRequest {
//!             symbols: vec!["AAPL".to_string()],
//!             ..Default::default()
//!         })
//!         .await?;
//!     for (symbol, bars) in &bars {
//!         println!("{symbol}: {} bars", bars.len());
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Streaming
//!
//! Register a handler per event type, then `run` the stream; it returns when the connection
//! ends or fails.
//!
//! ```no_run
//! use alpacars::data::enums::DataFeed;
//! use alpacars::data::live::stock::StockDataStream;
//! use alpacars::trading::stream::TradingStream;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), alpacars::AlpacaError> {
//!     let mut quotes = StockDataStream::new("API_KEY", "SECRET_KEY", DataFeed::Iex);
//!     // Handlers receive the event without its symbol: subscribe one symbol per handler
//!     // when you need to tell them apart.
//!     quotes.subscribe_trades(["AAPL"], |trade| {
//!         println!("AAPL {} x {}", trade.price, trade.size);
//!     });
//!
//!     let mut orders = TradingStream::new("API_KEY", "SECRET_KEY", true);
//!     orders.subscribe_trade_updates(|update| {
//!         println!("{:?}: {:?}", update.event, update.order.symbol);
//!     });
//!
//!     tokio::try_join!(quotes.run(), orders.run())?;
//!     Ok(())
//! }
//! ```
//!
//! # Errors
//!
//! A non-2xx response becomes [`AlpacaError::Api`] with the HTTP status and Alpaca's `code` and
//! `message`. Transport, JSON and WebSocket failures have their own variants.
//!
//! ```no_run
//! # use alpacars::trading::client::TradingClient;
//! # async fn f(client: TradingClient) {
//! match client.get_asset("NOT_A_SYMBOL").await {
//!     Ok(asset) => println!("{:?}", asset.status),
//!     Err(alpacars::AlpacaError::Api { status_code: 404, message, .. }) => {
//!         println!("no such asset: {message}")
//!     }
//!     Err(e) => eprintln!("request failed: {e}"),
//! }
//! # }
//! ```
//!
//! # Forward compatibility
//!
//! Every enum is `#[non_exhaustive]`. Enums that appear in responses have an `Unknown` variant,
//! so a value Alpaca adds later still parses instead of failing the whole response. Match on
//! them with a wildcard arm.
//!
//! Before 1.0, a minor version bump (`0.6` → `0.7`) is the breaking one; see the
//! [CHANGELOG](https://github.com/kfuangsung/alpacars/blob/main/CHANGELOG.md).

pub mod broker;
pub mod common;
pub mod data;
pub mod error;
pub mod trading;

pub use error::AlpacaError;

/// Compiles the README's examples as `no_run` doctests, so they cannot drift from the API.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
