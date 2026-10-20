# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Breaking**: `reqwest` 0.12 → 0.13. `AlpacaError::Http` wraps `reqwest::Error`, so it now carries the 0.13 error type; code that names `reqwest::Error` (e.g. matching on `AlpacaError::Http(e)` and calling reqwest-specific methods on `e`) must use reqwest 0.13. `cargo-semver-checks` does not detect this kind of change, so it is called out here.
- `reqwest` now uses `default-features = false` with `json`, `query`, `native-tls`, `charset`, `http2` and `system-proxy`. 0.13 made rustls/aws-lc-rs the default TLS backend; the crate stays on `native-tls`, the same backend `tokio-tungstenite` uses, so there is still one TLS stack and no new license or C-toolchain requirements.
- **Breaking**: `trading::GetAccountActivitiesRequest.activity_type: Option<ActivityType>` is now `activity_types: Option<Vec<ActivityType>>`, sent as the comma-separated `activity_types` query parameter the API actually reads. Its `page_token` is now `Option<String>`: activity IDs (and therefore page tokens) have the form `"<timestamp>::<uuid>"`, so the old `Option<Uuid>` made pagination impossible.
- **Breaking**: `BrokerClient::get_account_activities(account_id, filter)` is now `get_account_activities(filter)`; set `account_id` on the filter, or leave it unset to query across all accounts.
- **Breaking**: every public enum is now `#[non_exhaustive]`, so `match`es on them need a `_ =>` arm. In exchange, Alpaca adding a value is no longer a breaking change for this crate, so future additions can ship in minor releases.
- **Breaking**: the 44 enums that appear in API responses gained an `Unknown` variant (`#[serde(other)]`). A value this version does not recognize now parses as `Unknown` instead of failing the entire response — previously, for example, one record with a new tokenization issuer made `get_tokenization_requests` return an error. `Unknown` is not meaningful in requests; it serializes as `"unknown"`. Request-only enums (`QueryOrderStatus`, `OrderDestination`, `Adjustment`, …) are `#[non_exhaustive]` but have no `Unknown`.

### Added
- `order_id` filter on both `GetAccountActivitiesRequest`s; `activity_types` and `category` on the Broker one.
- `ActivityType` variants `Cgd`, `Divfee`, `Divft`, `Divtw`, `Fopt`, `Intnra`, `Inttw`, `Jnl`, `Misc`, `Opca`, `Ptr`, `Reo`, `Trans`, completing the set in the Trading and Broker API references. **Breaking** for exhaustive `match`es.
- Enum values Alpaca added since the last sync, each checked against the API reference:
  - `TokenizationNetwork`: `Cronos`, `Hyperevm`, `Robinhood`, `Hypercore`.
  - `TokenizationIssuer`: `Binance`, `Coinbase`, `Ondo`.
  - Broker `AccountType`: `Trust`, `OmnibusNonDisclosed`, `OmnibusSub`, `Joint` (returned on reads; not creatable through the API).
  - `JournalStatus`: `Queued`, `SentToClearing`, `Deleted`.

### Fixed
- **Breaking**: `TradeAccount.account_number` is now `Option<String>`. The Trading API reference types it `string | null`, and a `null` made `get_account` fail.
- `ActivityType` was serialized in snake_case (`fill`, `jnlc`); Alpaca's values are uppercase (`FILL`, `JNLC`). Activity filters were sent with values the API does not recognize, and `TradeActivity`/`NonTradeActivity` failed to deserialize any real activity.
- `BrokerClient::get_account_activities` called `/v1/accounts/{account_id}/account_activities`, which does not exist; the endpoint is `/v1/accounts/activities` with `account_id` as a query parameter.
- `AccountActivity` was `#[serde(untagged)]` with `Trade` tried first, and both shapes only require `id` and `activity_type`, so every activity — dividends, journals, fees — deserialized as `Trade`. It now selects the variant from `activity_type` (`FILL` is the only trade activity).
- **Breaking**: Broker rebalancing request bodies did not match the API, so `create_portfolio` and `create_manual_run` were rejected. Verified against the Create Portfolio / Create Run / Portfolio / List Runs references:
  - `CreatePortfolioRequest.assets: Vec<serde_json::Value>` → `weights: Vec<PortfolioWeightRequest>` (`type`, optional `symbol`, `percent`).
  - `CreateRunRequest { portfolio_id, account_id? }` → `{ account_id, run_type, amount?, weights? }`. The API has no `portfolio_id` on this request and requires `type` (`RunType::FullRebalance` / `InvestCash`). Omit `weights` for a subscription-driven run; pass them for a manual one-off run.
  - `Portfolio.assets` (and `PortfolioAsset`) → `weights: Option<Vec<PortfolioWeight>>`; the old field never matched the response and was always `None`. `Portfolio` also gains `rebalancing_conditions`.
  - `RebalancingRun` gains `account_id`, `run_type`, `initiated_from`, `weights`, `canceled_at`; `Subscription` gains `last_rebalanced_at`; `GetRunsRequest.run_type` is now `Option<RunType>`.
  - New enums `WeightType`, `RunType`, `RunInitiatedFrom`.
- **Breaking**: Broker batch journal request shapes, per the Create Batch Journal / Reverse Batch Journal references:
  - `CreateReverseBatchJournalRequest` takes a single top-level `to_account`, and `ReverseBatchJournalEntry` is now `{ from_account, amount, description? }` — previously each entry carried a `to_account`, which the many-to-one endpoint does not accept.
  - `BatchJournalEntryRequest.amount` is `Option<String>` (the API allows `null`), and the entry gains `currency`, `symbol`, `qty` and the `transmitter_*` travel-rule fields. `CreateBatchJournalRequest` gains `correspondent`.

## [0.5.1] - 2026-10-20

Patch release: unblocks the dependency updates, fixes the Broker journal endpoints, and patches an h2 advisory. No breaking API changes; the MSRV rises to 1.89.

### Added
- `tests/data_stream.rs`, the first runtime test of a WebSocket stack. It drives `DataStreamConnection` against a local server through auth, subscribe, JSON text frames, msgpack binary frames, ping/pong and close.

### Changed
- MSRV raised from 1.88 to 1.89, required by `uuid` ≥ 1.27.
- Dependency updates: `tokio` 1.53.3, `serde_json` 1.0.154, `thiserror` 2.0.21, `uuid` 1.28.0, `base64` 0.23.1, `futures-util` 0.3.34.
- `tokio-tungstenite` 0.24 → 0.30. No public API change: WebSocket errors were already surfaced as `AlpacaError::WebSocket(String)`.

### Fixed
- Broker journals were non-functional. All of these were verified against the `docs.alpaca.markets/us/reference` pages, and none of them changes the Rust API:
  - `JournalEntryType` was serialized as `CASH`/`SECURITY`; Alpaca uses `JNLC`/`JNLS`. Every journal response failed to deserialize and every journal request sent an invalid `entry_type`.
  - `JournalStatus` was serialized in SCREAMING_SNAKE_CASE (`PENDING`); Alpaca uses lowercase (`pending`), and `Corrected` is `correct` on the wire.
  - `create_batch_journal` posted to `/v1/journals:batch` instead of `/v1/journals/batch`.
  - `create_reverse_batch_journal` posted to `/v1/journals:batch_reverse` instead of `/v1/journals/reverse_batch`.

### Security
- Bumped `h2` to 0.4.20 in `Cargo.lock` to fix [RUSTSEC-2026-0258](https://rustsec.org/advisories/RUSTSEC-2026-0258) (unbounded queueing of empty DATA frames). This only changes the lockfile — downstream crates pick up the fix with their own `cargo update -p h2`.

## [0.5.0] - 2026-08-01

Catch-up with the Alpaca changelog for 2026-07-16 through 2026-07-29.

### Added
- Tokenization API on `TradingClient`: `mint_tokenized_asset`, `get_tokenization_requests`, `get_tokenization_request`, `get_tokenization_request_by_client_request_id` (`/v2/tokenization/*`).
- Tokenization API on `BrokerClient`: the same four plus `get_tokenization_request_for_account_by_issuer_request_id`, which has no Trading API equivalent (`/v1/accounts/{account_id}/tokenization/*`).
- `Idempotency-Key` support on `create_locate`, so retries return the original locate instead of creating a second one.
- `RestClient::post_with_headers` for endpoints needing per-request headers. The headers replay on every retry attempt.
- `BrokerClient::new_with_url`, the `#[doc(hidden)]` test constructor every other client already had. The Broker API previously had no test coverage at all; adding it is what surfaced the rebalancing bugs below.
- `NonTradeActivityStatus::Pending`.

### Changed
- **Breaking**: `create_locate` takes a new `idempotency_key: Option<&str>` parameter. Pass `None` to keep the previous behavior.
- **Breaking**: `PortfolioHistory.base_value` is now `Option<f64>`. Alpaca marked `equity`, `profit_loss`, `profit_loss_pct` and `base_value` nullable on 2026-07-20; the three array fields were already optional, `base_value` was not and would fail to deserialize on a null.
- **Breaking**: the new `NonTradeActivityStatus` variant may require a new arm in exhaustive `match`es.
- Documented the known error-code values on `Locate.rejection_reason` and `LocateQuoteError.code`, including `quote_unavailable` (added 2026-07-23) and `idempotency_key_conflict`. Both stay `String` rather than becoming enums, since Alpaca keeps adding values.

### Fixed
- **Broker rebalancing endpoints were entirely non-functional.** All 13 methods targeted `/v1/portfolios`, `/v1/subscriptions` and `/v1/runs`; the API serves them under a `/v1/rebalancing` prefix, so every call 404'd.
- **Breaking**: `inactivate_portfolio_by_id` renamed to `archive_portfolio_by_id` and switched from `POST /v1/portfolios/{id}:inactivate` to the real `DELETE /v1/rebalancing/portfolios/{portfolio_id}`.
- **Breaking**: `get_all_runs` returns `ListRunsResponse { runs, next_page_token }` instead of `Vec<RebalancingRun>` — the endpoint returns a paginated object, so the previous return type failed to deserialize every response.
- **Breaking**: `GetRunsRequest` carried `portfolio_id`/`after`/`before`, none of which the endpoint accepts. Replaced with the documented `account_id`, `status`, `type` and `page_token`.
- **Breaking**: `PortfolioStatus` was `SCREAMING_SNAKE_CASE` with an `Active`/`Suspended`/`Inactive` variant set. The API uses lowercase `active`/`inactive`/`needs_adjustment` — there is no `suspended`, and `needs_adjustment` was missing — so every portfolio response failed to deserialize.

### Deprecated
- `TokenizationRequest.account` and `.issuer_account` are modeled as `Option` and superseded by `client_account_id` / `client_external_account_id`. Alpaca removes them on 2026-10-15.

## [0.4.1] - 2026-07-22

### Changed
- Relicensed under dual `MIT OR Apache-2.0` (previously MIT-only), matching the Rust ecosystem convention and adding Apache-2.0's explicit patent grant. Applies to releases from this version onward; already-published versions (0.1.0–0.4.0) remain MIT-only as published.

## [0.4.0] - 2026-07-22

### Added
- GitHub Actions CI: build, test, `cargo fmt --check`, and `cargo clippy -D warnings` on stable, plus a dedicated MSRV job.
- `CLAUDE.md` with architecture and command reference for AI-assisted development.

### Changed
- Declared `rust-version = "1.88"` in `Cargo.toml`. The README's previous `rust-1.75+` badge was never enforced and was wrong — verified locally that 1.88 is the real minimum.
- Applied `cargo fmt` across the whole tree and fixed 3 pre-existing clippy lints.
- Bumped `actions/checkout` v4 → v7 in CI to resolve a Node.js 20 deprecation warning.

## [0.3.0] - 2026-07-17

### Added
- Locates API for hard-to-borrow trading: `TradingClient::create_locate`, `get_locates`, `get_locate`, `get_locate_quotes` (`/v1/locates`).
- DMA order routing: opt-in `advanced_instructions` on order create/replace (`NYSE`/`NASDAQ`/`ARCA`/`IEX`/`MEMX` destinations, `display_qty`).
- New fields: `Order.ratio_qty`, `Position.prev_swap_rate`, `NonTradeActivity.currency`, `ReverseSplit.new_symbol`, broker `Account.allow_instant_ach` / `instant_ach_blocked`.
- New enum variants: `AssetClass` (`us_index`, `treasury`, `corporate`, `global_equity`, `us_equity_chain`, `ipo`), `AccountStatus::AccountClosedPending`, broker `JournalStatus::ActivityCreated`.

### Changed
- **Breaking**: new enum variants on `AssetClass`/`AccountStatus`/`JournalStatus` may require new arms in exhaustive `match`es.

### Deprecated
- `Asset.easy_to_borrow` is now `Option<bool>` and deprecated in favor of `Asset.borrow_status`. Alpaca removes the field from the API on 2026-09-22.

## [0.2.0] - 2026-07-11

### Changed
- **Breaking**: removed PDT (Pattern Day Trader) fields, matching their removal from the Alpaca API on 2026-07-06.
- Trading stream auth/subscription events now log at info level.

### Fixed
- Trading stream auth protocol corrected — connections were silently unauthorized.

## [0.1.2] - 2026-05-09

### Added
- `get_option_exchange_codes` on `OptionHistoricalDataClient`.

## [0.1.1] - 2026-05-06

### Fixed
- Corrected Alpaca API endpoints, serde field mappings, and WebSocket URLs.

### Added
- crates.io badge in README.

## [0.1.0] - 2026-05-03

### Added
- Initial release: async Rust SDK mirroring alpaca-py, covering Trading, Broker, and Market Data (historical + live streaming) APIs.
- Structured tracing across the REST client, WebSocket connections, and `TradingStream`.
- Release profile optimized for production builds (LTO, strip, `panic = "abort"`).

### Changed
- REST client retry switched to exponential backoff.

### Fixed
- Silent failures in the WebSocket connection, `TradingStream`, and live stream handlers now surface via tracing instead of failing silently.

[Unreleased]: https://github.com/kfuangsung/alpacars/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/kfuangsung/alpacars/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/kfuangsung/alpacars/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/kfuangsung/alpacars/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/kfuangsung/alpacars/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/kfuangsung/alpacars/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kfuangsung/alpacars/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/kfuangsung/alpacars/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/kfuangsung/alpacars/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/kfuangsung/alpacars/releases/tag/v0.1.0
