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

Running an example against the real API (requires live credentials):
```bash
APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example stocks_trading_basic
```

### Reproducing CI locally

CI (`.github/workflows/ci.yml`) gates every PR on the following. Run them before pushing — all are expected to be clean, and `-D warnings` means a warning is a failure:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo build --all-targets --locked && cargo test --locked
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked   # catches docs.rs build breakage
cargo publish --dry-run --locked                          # packaging is a different code path from a normal build
cargo semver-checks check-release                         # breaking changes vs the published crate
cargo deny check                                          # advisories, licenses, bans, sources
```

Tests also run on macOS and Windows, not just Linux — `tokio-tungstenite` uses `native-tls`, which is a different backend per platform.

A caveat worth internalising: `cargo doc` and `cargo semver-checks` can **pass on a tree that does not compile**, because rustdoc does not type-check function bodies the way rustc does. Only `cargo build`/`cargo test` establish that the crate builds.

`cargo-deny` is configured by `deny.toml`. Its license allow-list is deliberately minimal and was derived from a real dependency-tree scan — if a new dependency introduces a license, add it deliberately rather than broadening the list to make CI green. `advisories.ignore` is empty on purpose: fix an advisory by upgrading, not by silencing it.

### Releasing

`.github/workflows/release.yml` publishes on a `v*` tag via crates.io Trusted Publishing (OIDC — there is no `CARGO_REGISTRY_TOKEN` secret, and none should be added). The whole release process is:

1. Bump `version` in `Cargo.toml`.
2. Add a `## [X.Y.Z] - DATE` section to `CHANGELOG.md` (Keep a Changelog format).
3. Tag `vX.Y.Z` and push the tag.

The `verify` job runs first and refuses to proceed if the tag does not match `Cargo.toml`, or if `CHANGELOG.md` has no section for that version. Publishing is irreversible — a crates.io version can be yanked but never replaced — so everything recoverable is checked before anything is uploaded.

Note that this crate is pre-1.0: a `0.x` **minor** bump is the breaking-change bump. `cargo-semver-checks` understands this, so breaking changes require `0.(x+1).0`, not a patch.

## Architecture

### Domain modules mirror Alpaca's API surface

The crate is split into four top-level modules under `src/`, each independent and each following the same internal file layout (`client.rs`, `enums.rs`, `models.rs`, `requests.rs`):

- `trading/` — `TradingClient` (orders, positions, assets, watchlists, options contracts, locates, tokenization) and `TradingStream` (WebSocket trade-update events) against the paper/live trading REST + stream APIs.
- `broker/` — `BrokerClient` for the Broker API (account management, ACH/bank transfers, journals, rebalancing, tokenization). Always uses HTTP Basic auth, unlike Trading/Data clients.
- `data/` — market data, split further into `historical/` (one file per asset class: `stock.rs`, `crypto.rs`, `option.rs`, `news.rs`, `corporate_actions.rs`, `screener.rs`) and `live/` (per-asset-class WebSocket stream wrappers over the shared `websocket.rs` connection primitive).
- `common/` — shared plumbing used by every domain: `RestClient` (auth, retry, request execution) and `base_url` (all REST/WS endpoint constants for paper/live/sandbox environments).

When adding a new endpoint, follow the existing domain's file split rather than inventing a new grouping.

### `RestClient` is the single HTTP chokepoint

Every domain client (`TradingClient`, `BrokerClient`, `StockHistoricalDataClient`, etc.) wraps a `common::client::RestClient` rather than talking to `reqwest` directly. `RestClient` centralizes:

- **Auth mode selection**: API key headers (`APCA-API-KEY-ID`/`APCA-API-SECRET-KEY`), OAuth Bearer token, or HTTP Basic — chosen via constructor flags, not per-request.
- **Retry with exponential backoff**: automatic retry on HTTP 429/504, wait doubling each attempt (3s → 6s → 12s...), capped at 60s (`DEFAULT_RETRY_ATTEMPTS`, `DEFAULT_RETRY_WAIT_SECS`, `DEFAULT_RETRY_STATUS_CODES` in `common/client.rs`).
- **Typed verbs**: `get`/`post`/`patch`/`put`/`delete`/`delete_with_body` deserialize into a typed response; `get_raw` returns `serde_json::Value` for endpoints whose shape varies (e.g. account activities); `post_void`/`delete_void` discard the body; `post_with_headers` adds per-request headers on top of the auth headers (used for the `Idempotency-Key` on `create_locate`). Extra headers are applied inside the retry loop, so they replay on every attempt — which is what an idempotency key needs.

New client methods should be thin wrappers calling one of these verbs — don't reimplement request building or retry logic at the call site.

### Every client has a `new_with_url` test-only constructor

Alongside the public `new(api_key, secret_key, paper/sandbox)` constructor, clients expose a `#[doc(hidden)] new_with_url(...)` that points at an arbitrary base URL. This exists purely so tests can point the client at a `wiremock::MockServer`. `tests/common/mod.rs` has thin factory functions (`trading_client`, `broker_client`, `stock_client`, `corporate_actions_client`) built on top of it — extend that file rather than constructing clients ad hoc in each test.

A client without this constructor cannot be tested at all, and every time one was added it surfaced bugs: `BrokerClient` gained it in v0.5.0 (the rebalancing bugs below), `CorporateActionsClient` in v0.6.0 (it had been calling `/v1beta1/corporate-actions` instead of `/v1/corporate-actions`). **`CryptoHistoricalDataClient`, `NewsClient` and `ScreenerClient` still lack it** — add it before trusting or changing them.

### Two independent WebSocket stacks

Trading-stream and market-data-stream are separate implementations, not a shared abstraction — don't try to unify them:

- `trading/stream.rs` (`TradingStream`) — auth via `{action: "auth", key, secret}`, listens on a single `trade_updates` stream. Accepts text **and binary** frames: per Alpaca's "Websocket Streaming" doc the *paper* stream sends `trade_updates` as binary frames (JSON payload, or MessagePack), so ignoring binary silently drops every paper update. Tested end to end in `tests/trading_stream.rs` via the `#[doc(hidden)] new_with_url`.
- `data/live/websocket.rs` (`DataStreamConnection`) — auth via `{action: "auth", key, secret}`, subscribes to multiple typed channels at once (trades/quotes/bars/updated_bars/daily_bars/statuses/orderbooks/news) via `SubscribeMsg`, and accepts both JSON text frames and msgpack binary frames (falling back to JSON if msgpack decoding fails) via `rmp-serde`. Per-asset-class wrappers in `data/live/{stock,crypto,option,news}.rs` build the appropriate `SubscribeMsg` and expose typed `subscribe_*` handler-registration methods over the raw `RawStreamEvent`.

### Historical data auto-pagination

Alpaca's historical endpoints (bars/quotes/trades) return a `next_page_token`; SDK methods like `get_stock_bars` loop internally — feeding `next_page_token` back in as `page_token` — until the token is empty, so callers always get the complete result set in one call rather than handling pagination themselves (see `data/historical/stock.rs`).

### Error handling

All fallible operations return `error::AlpacaError` (thiserror-based), with variants for `Http` (reqwest transport errors), `Api` (non-2xx responses, carrying `status_code`/`code`/`message` parsed from Alpaca's JSON error body), `Json`, `WebSocket`, `InvalidCredentials`, and `Msgpack`. Don't introduce a second error type for new endpoints — extend `AlpacaError` if a genuinely new failure mode appears.

### Testing convention

Tests live in `tests/*.rs` (one file per resource area, e.g. `trading_orders.rs`, `trading_positions.rs`, `data_stock.rs`) and use `wiremock` to mock HTTP responses inline — no real API credentials are ever needed to run the suite. Each test spins up a `MockServer`, mounts one or more `Mock::given(method(...)).and(path(...))` expectations with a canned JSON response body, then exercises the client built via the `tests/common` helpers.

**Always assert the full path and verb, not just the response body.** A mock mounted on the wrong path silently returns 404 and the test fails for the wrong reason — or worse, a test that only checks deserialization passes against an endpoint the client never actually hits correctly. Adding `.expect(1)` makes an unmatched request fail loudly rather than pass vacuously.

When a test is meant to prove a *negative* (a header is absent, a field is omitted), verify it actually fails against deliberately broken code before trusting it. Several tests here were written that way, and one of them caught a real bug that reading the docs had not.

**Build fixtures from the API reference, never from the crate's own types.** A fixture copied from the crate encodes the crate's bugs and passes against them: `tests/broker_rebalancing.rs` used the crate's `assets` key, so `create_portfolio` shipped sending a field the API does not accept. For request bodies, assert the whole body with `body_json` against the reference's named request examples where they exist.

### Enums: `#[non_exhaustive]` everywhere, `Unknown` on response enums

Every public enum is `#[non_exhaustive]`. Every enum that appears in a response type (any `models.rs`, the trading stream, the live data streams) also ends with a `#[serde(other)] Unknown` unit variant, so a value Alpaca adds later parses instead of failing the whole response. Request-only enums get no `Unknown`. When adding an enum, follow the same rule, add it to `tests/enum_forward_compat.rs` if it has `Unknown`, and add its reference value list to `tests/enum_wire_values.rs`. When values don't survive `rename_all` case conversion (`TYPE_401K`, `DIVCGL`-style acronyms, empty strings), rename every variant explicitly.

**Adding a variant to an existing enum in a patch release:** serde requires `#[serde(other)] Unknown` to stay the *last* variant, so a new variant inserted before it shifts `Unknown`'s implicit discriminant — which `cargo semver-checks` reports as major (`enum_no_repr_variant_discriminant_changed`). Pin `Unknown` to its previous value and number the new variants after it explicitly, e.g. `NewValue = 14, … Unknown = 13` (see `DocumentType`). In a minor (0.x) release this does not matter.

### Broker API endpoint paths are the least trustworthy part of the crate

The Trading API has an official OpenAPI spec to check against; the Broker API does not, so its paths were transcribed by hand and were never cross-checked. That gap hid four bugs that made **all 13 rebalancing methods fail** — wrong path prefix (`/v1/portfolios` rather than `/v1/rebalancing/portfolios`), wrong verb for archive, wrong response shape for List All Runs, and a `PortfolioStatus` enum with both wrong casing and wrong variants.

The 0.6.0 audit found the same pattern in every Broker area it checked: all journal methods (wrong `JNLC`/`JNLS` and status casing, wrong batch paths), the rebalancing *request bodies* (`assets` vs `weights`; `create_run` missing its required `type`), account activities (a non-existent path), and `create_account` (unable to express non-trading account types). The 0.6.1 audit of the rest found per-account orders, positions, watchlists and the trading-account read all on `/v1/accounts/{id}/…` instead of `/v1/trading/accounts/{id}/…`, recipient banks on `/banks`, CIP on `/cip_data`, and the upload `DocumentType` holding sub-type values — plus `close_position` (Trading too) sending `qty`/`percentage` as an ignored DELETE body.

Before relying on or modifying an untested `BrokerClient` method, verify its path, request body and enum encoding against the docs reference page — not against the crate, and not against the Trading spec. Each `docs.alpaca.markets/us/reference/<operation>.md` page embeds that operation's full OpenAPI JSON in a fenced block, so enums, required fields and examples can be parsed exactly. As of v0.6.1 every Broker area has tests. The methods still without a test call (paths match the reference, but are unexercised): `get_all_assets`, `get_asset`, `get_clock`, `get_calendar`, `get_journal_by_id`, `cancel_journal_by_id`, `update_portfolio_by_id`, `get_all_subscriptions`, `get_subscription_by_id`, `get_run_by_id`.
