# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`alpacars` is an async-first Rust SDK for the Alpaca Markets API (Trading, Broker, and Market Data — historical + live streaming). It mirrors the official `alpaca-py` Python SDK's shape so users of that library can transfer knowledge directly. Every network call is `async`, built on `tokio`/`reqwest`, and returns `Result<T, AlpacaError>`.

## Commands

```bash
cargo build                      # build the library
cargo test                       # run the full test suite (wiremock-mocked, no real credentials needed)
cargo test --test trading_orders # run a single test file
cargo test test_submit_market_order  # run a single test by name (substring match)
cargo check                      # fast type-check without codegen
```

There is no CI config, lint config (rustfmt/clippy), or Makefile in the repo — don't assume conventions beyond what `cargo fmt`/`cargo clippy` defaults provide.

Running an example against the real API (requires live credentials):
```bash
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example stocks_trading_basic
```

## Architecture

### Domain modules mirror Alpaca's API surface

The crate is split into four top-level modules under `src/`, each independent and each following the same internal file layout (`client.rs`, `enums.rs`, `models.rs`, `requests.rs`):

- `trading/` — `TradingClient` (orders, positions, assets, watchlists, options contracts, locates) and `TradingStream` (WebSocket trade-update events) against the paper/live trading REST + stream APIs.
- `broker/` — `BrokerClient` for the Broker API (account management, ACH/bank transfers, journals, rebalancing). Always uses HTTP Basic auth, unlike Trading/Data clients.
- `data/` — market data, split further into `historical/` (one file per asset class: `stock.rs`, `crypto.rs`, `option.rs`, `news.rs`, `corporate_actions.rs`, `screener.rs`) and `live/` (per-asset-class WebSocket stream wrappers over the shared `websocket.rs` connection primitive).
- `common/` — shared plumbing used by every domain: `RestClient` (auth, retry, request execution) and `base_url` (all REST/WS endpoint constants for paper/live/sandbox environments).

When adding a new endpoint, follow the existing domain's file split rather than inventing a new grouping.

### `RestClient` is the single HTTP chokepoint

Every domain client (`TradingClient`, `BrokerClient`, `StockHistoricalDataClient`, etc.) wraps a `common::client::RestClient` rather than talking to `reqwest` directly. `RestClient` centralizes:

- **Auth mode selection**: API key headers (`APCA-API-KEY-ID`/`APCA-API-SECRET-KEY`), OAuth Bearer token, or HTTP Basic — chosen via constructor flags, not per-request.
- **Retry with exponential backoff**: automatic retry on HTTP 429/504, wait doubling each attempt (3s → 6s → 12s...), capped at 60s (`DEFAULT_RETRY_ATTEMPTS`, `DEFAULT_RETRY_WAIT_SECS`, `DEFAULT_RETRY_STATUS_CODES` in `common/client.rs`).
- **Typed verbs**: `get`/`post`/`patch`/`put`/`delete`/`delete_with_body` deserialize into a typed response; `get_raw` returns `serde_json::Value` for endpoints whose shape varies (e.g. account activities); `post_void`/`delete_void` discard the body.

New client methods should be thin wrappers calling one of these verbs — don't reimplement request building or retry logic at the call site.

### Every client has a `new_with_url` test-only constructor

Alongside the public `new(api_key, secret_key, paper/sandbox)` constructor, each client exposes a `#[doc(hidden)] new_with_url(...)` that points at an arbitrary base URL. This exists purely so tests can point the client at a `wiremock::MockServer`. `tests/common/mod.rs` has thin factory functions (`trading_client(base_url)`, `stock_client(base_url)`) built on top of it — extend that file rather than constructing clients ad hoc in each test.

### Two independent WebSocket stacks

Trading-stream and market-data-stream are separate implementations, not a shared abstraction — don't try to unify them:

- `trading/stream.rs` (`TradingStream`) — auth via `{action: "auth", data: {key_id, secret_key}}`, listens on a single `trade_updates` stream, JSON only.
- `data/live/websocket.rs` (`DataStreamConnection`) — auth via `{action: "auth", key, secret}`, subscribes to multiple typed channels at once (trades/quotes/bars/updated_bars/daily_bars/statuses/orderbooks/news) via `SubscribeMsg`, and accepts both JSON text frames and msgpack binary frames (falling back to JSON if msgpack decoding fails) via `rmp-serde`. Per-asset-class wrappers in `data/live/{stock,crypto,option,news}.rs` build the appropriate `SubscribeMsg` and expose typed `subscribe_*` handler-registration methods over the raw `RawStreamEvent`.

### Historical data auto-pagination

Alpaca's historical endpoints (bars/quotes/trades) return a `next_page_token`; SDK methods like `get_stock_bars` loop internally — feeding `next_page_token` back in as `page_token` — until the token is empty, so callers always get the complete result set in one call rather than handling pagination themselves (see `data/historical/stock.rs`).

### Error handling

All fallible operations return `error::AlpacaError` (thiserror-based), with variants for `Http` (reqwest transport errors), `Api` (non-2xx responses, carrying `status_code`/`code`/`message` parsed from Alpaca's JSON error body), `Json`, `WebSocket`, `InvalidCredentials`, and `Msgpack`. Don't introduce a second error type for new endpoints — extend `AlpacaError` if a genuinely new failure mode appears.

### Testing convention

Tests live in `tests/*.rs` (one file per resource area, e.g. `trading_orders.rs`, `trading_positions.rs`, `data_stock.rs`) and use `wiremock` to mock HTTP responses inline — no real API credentials are ever needed to run the suite. Each test spins up a `MockServer`, mounts one or more `Mock::given(method(...)).and(path(...))` expectations with a canned JSON response body, then exercises the client built via the `tests/common` helpers.
