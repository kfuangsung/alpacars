# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- **Breaking:** `BrokerClient::create_watchlist_for_account` and `update_watchlist_for_account_by_id` take new Broker-specific `broker::requests::{CreateWatchlistRequest, UpdateWatchlistRequest}` with required `name: String` and `symbols: Vec<String>`. The Broker reference (`createwatchlistforaccount`, `replacewatchlistforaccountbyid`) requires both fields on both calls, while the Trading API's create needs only `name` and its update neither, so the Trading types the Broker methods used to share are unchanged. `broker::models` no longer re-exports the two Trading watchlist request types.
- **Breaking:** `BrokerClient::upload_cip_data_for_account_by_id` returns `()`. The reference documents its `200` without a body, so the old `CIPInfo` return reported a deserialization error on an empty body even though the upload succeeded.

### Removed
- **Breaking:** the methods deprecated in 0.6.1, none of which could succeed: `BrokerClient::delete_account` (use `close_account`), `get_trade_account_by_id` (use `get_trading_account`) and `get_trade_document_for_account_by_id` (use `get_trade_documents_for_account` and `download_trade_document_for_account`).
- **Breaking:** the invalid enum variants documented in 0.6.1: `DocumentType::{DriversLicense, NationalIdCard, Passport, PassportCard, VisaPermit, ResidencePermit, TaxReturn, FinancialStatement, BankStatement, FundSource, AccountTransferIn, LimitedPoa, Other}` (sub-type values, not document types) and `TradeDocumentType::TaxForm*` (not in the reference). Those values now parse as `Unknown`. The explicit discriminants pinned in 0.6.1 are gone.
- **Breaking:** `RestClient::delete_with_body`. Alpaca ignores DELETE bodies, which is how `close_position` closed whole positions before 0.6.1; use `delete` with a query.
- **Breaking:** `OptionChainRequest`, which no method took (`get_option_chain` takes its arguments directly).

## [0.6.2] - 2026-10-20

Patch release: every client is now testable and tested, which found three more broken paths and a `TradingStream` that dropped every trade update on paper accounts — plus the Trading and Broker endpoints the crate was missing. No breaking changes. **If you use `TradingStream` against paper trading, upgrade.**

### Added
- Endpoints the crate lacked, checked against the Trading spec and Broker references:
  - `TradingClient`: `get_account_activities_by_type` (`/v2/account/activities/{activity_type}`), `do_not_exercise_options_position`, `get_watchlist_by_name` / `add_asset_to_watchlist_by_name` / `update_watchlist_by_name` / `delete_watchlist_by_name` (`/v2/watchlists:by_name`), and `get_market_calendar(market, …)` (`GET /v3/calendar/{market}`: pre-, core- and post-market sessions; `market` is a MIC, BIC or acronym such as `XNYS`) with new `MarketCalendar`, `Market`, `MarketCalendarDay` and `GetMarketCalendarRequest`.
  - `BrokerClient`: `get_account_activities_by_type`, `do_not_exercise_options_position_for_account`, `get_order_for_account_by_client_order_id`, and `estimate_order_for_account` with `EstimateOrderRequest`.
- `RestClient::post_with_query` / `put_with_query`, for endpoints that take both a query string and a body.
- `CryptoFeed::{Us1, Us2, Eu1, Bs1}` — the other crypto market data locations in the reference (`us-1`, `us-2`, `eu-1`, `bs-1`).
- `#[doc(hidden)] new_with_url` on `CryptoHistoricalDataClient`, `NewsClient` and `ScreenerClient`, so every historical data client can now be tested; tests for all four of crypto, news, screener and options.
- `#[doc(hidden)] TradingStream::new_with_url`, and the first end-to-end test of the trading stream.

### Fixed
- **`TradingStream` delivered no trade updates on paper accounts.** Per Alpaca's "Websocket Streaming" doc, the paper stream (`wss://paper-api.alpaca.markets/stream`) sends `trade_updates` in binary frames; `TradingStream` handled only text frames and silently dropped binary ones. Binary frames are now decoded (JSON, with a MessagePack fallback) and handled like text frames.
- `TradingStream` authenticated with the legacy `{"action": "authenticate", "data": {"key_id", "secret_key"}}` message; it now sends the documented `{"action": "auth", "key", "secret"}`.
- `ScreenerClient::get_market_movers` always called `/v1beta1/screener/stocks/movers` and sent `market_type` as a query parameter the API does not read, so `MarketType::Crypto` returned stock movers. `market_type` now selects the path (`/screener/{market_type}/movers`).
- `OptionHistoricalDataClient::get_option_chain` called `/v1beta1/options/snapshots?underlying_symbols=…`, an endpoint that requires contract `symbols`; the chain endpoint is `/v1beta1/options/snapshots/{underlying_symbol}`.
- Crypto bars, latest bars / quotes / trades / orderbooks and snapshots always used the `us` location: `loc` was sent as an ignored query parameter while the path was hardcoded to `/crypto/us/…`. `loc` now selects the path segment. (Quotes and trades still use `us`: their request type has no `loc` field, and adding one is breaking.)

## [0.6.1] - 2026-10-20

Patch release: a reference audit of every remaining untested Broker area found 29 methods calling paths that do not exist or sending values the API rejects — all fixed, with no breaking changes. **If you close positions partially, upgrade**: `close_position` and `close_position_for_account` ignored `qty` / `percentage` and closed the whole position.

### Added
- `BrokerClient::get_trading_account(account_id)` (`GET /v1/trading/accounts/{account_id}/account`): balances, buying power and trading configuration for an account.
- `BrokerClient::download_trade_document_for_account(account_id, document_id) -> Vec<u8>` (`GET …/documents/{document_id}/download`). The API redirects to a pre-signed link, which is followed; the Basic-auth `Authorization` header is not forwarded to the file host (covered by a test).
- `RestClient::get_bytes`, for endpoints that serve files. It shares the JSON verbs' auth, retry and error handling (the request loop is now one `send_with_retry` path).
- `DocumentType` variants for the API's actual upload `document_type` values (`IdentityVerification`, `AddressVerification`, `TaxIdVerification`, `W8ben`, `W9`, `CipResult`, … — 14 in all), and `TradeDocumentType::{AccountApplication, CryptoAccountStatement, OptionsApplication, TaxStatement, TradeConfirmationJson}`.

### Deprecated
- `BrokerClient::get_trade_account_by_id(account_id, trading_account_id)`: the endpoint takes no trading account id. It now calls `get_trading_account` and ignores the second argument.
- `BrokerClient::delete_account`: the Broker API has no `DELETE /v1/accounts/{account_id}` endpoint, so it can never succeed. Use `close_account`.
- `BrokerClient::get_trade_document_for_account_by_id`: there is no JSON endpoint for a single document. Use `get_trade_documents_for_account` and `download_trade_document_for_account`.
- The pre-existing `DocumentType` variants (`DriversLicense`, `Passport`, …) are not valid `document_type` values — they are `document_sub_type` values — and the `TradeDocumentType::TaxForm*` variants are not in the API reference. They are documented as such.

All of these will be removed in 0.7.0.

### Fixed
- `upload_documents_to_account` could never send a valid `document_type`: `DocumentType`'s variants were sub-type values in SCREAMING_SNAKE_CASE (`PASSPORT`), while the API takes `identity_verification`, `address_verification`, …. Use the new variants (see Added).
- `get_trade_documents_for_account` returned every document with `document_type` and `document_sub_type` set to `None`: the API's keys are `type` and `sub_type`, and its type values are lowercase (`account_statement`). `TradeDocument` now reads both key spellings, and `TradeDocumentType::AccountStatement` / `TradeConfirmation` use the lowercase wire values.
- CIP get / upload called `/v1/accounts/{account_id}/cip_data`; the endpoint is `/v1/accounts/{account_id}/cip`. (The reference documents the upload's `200` without a body; if Alpaca returns none, `upload_cip_data_for_account_by_id` reports a deserialization error despite succeeding — its return type is unchanged until 0.7.0.)
- Broker recipient banks called `/v1/accounts/{account_id}/banks…`; the endpoints are `/v1/accounts/{account_id}/recipient_banks…`. `create_bank_for_account`, `get_banks_for_account` and `delete_bank_for_account` were all failing.
- `TransferType` was serialized as `ACH` / `WIRE`; the wire values are `ach` / `wire`. `create_transfer_for_account` sent an unrecognized `transfer_type`, and transfers read back parsed as `TransferType::Unknown`.
- `CreateTransferRequest` sent unset `relationship_id` / `bank_id` as `null`, and `CreateBankRequest` did the same for its optional fields; the API types these as plain strings. Unset fields are now omitted.
- `CreateBankRequest` docs now state that the API requires `bank_code` and `bank_code_type` (`ABA` / `BIC`), and that `routing_number` is not part of the request.
- `BrokerClient::close_account` called `POST /v1/accounts/{account_id}:close`; the endpoint is `POST /v1/accounts/{account_id}/actions/close`.
- `BrokerClient::get_trade_account_by_id` called `/v1/accounts/{account_id}/trading_accounts/{id}`, which does not exist (see Deprecated).
- **`close_position` / `close_position_for_account` closed the entire position when asked for a partial close.** `ClosePositionRequest.qty` / `percentage` were sent as a `DELETE` request body, which the API ignores; they are query parameters (Trading spec; Broker `closepositionforaccountbysymbol` reference). They are now sent as query parameters. Affects both `TradingClient` and `BrokerClient`.
- Broker per-account orders, positions and options exercise called `/v1/accounts/{account_id}/…`, which does not exist; the endpoints are `/v1/trading/accounts/{account_id}/…`. All eleven methods were failing: `submit_order_for_account`, `get_orders_for_account`, `get_order_for_account_by_id`, `replace_order_for_account_by_id`, `cancel_orders_for_account`, `cancel_order_for_account_by_id`, `get_all_positions_for_account`, `get_open_position_for_account`, `close_all_positions_for_account`, `close_position_for_account`, `exercise_options_position_for_account_by_id`.
- Broker per-account watchlists called `/v1/accounts/{account_id}/watchlists…`, which does not exist; the endpoints are `/v1/trading/accounts/{account_id}/watchlists…`. All five methods were failing: `get_watchlists_for_account`, `get_watchlist_for_account_by_id`, `create_watchlist_for_account`, `update_watchlist_for_account_by_id`, `delete_watchlist_from_account_by_id`. Their docs now note that the Broker API requires `symbols` on create and both `name` and `symbols` on update (a full replace).

## [0.6.0] - 2026-10-20

Catch-up with the Alpaca changelog for 2026-07-29 through 2026-10-19, forward-compatible enums, and an audit of the Broker API against its reference pages that found several call paths broken outright — account activities, rebalancing request bodies, batch journals and corporate actions now work. Breaking: see *Upgrading from 0.5* first.

### Upgrading from 0.5

- **`match` on any crate enum needs a `_ =>` arm**: every public enum is now `#[non_exhaustive]`, and response enums have a new `Unknown` variant for values Alpaca adds later.
- **`reqwest` 0.13**: only matters if your code names `reqwest::Error` from `AlpacaError::Http`.
- **Mint methods** take a new trailing `idempotency_key` argument: `mint_tokenized_asset(&req, None)` / `mint_tokenized_asset_for_account(&id, &req, None)`, or pass an idempotency key.
- **Account activities**: `activity_type: Some(t)` → `activity_types: Some(vec![t])`; `page_token` is a `String`; `BrokerClient::get_account_activities(&id, filter)` → `get_account_activities(filter)` with `account_id` set on the filter.
- **Rebalancing**: `CreatePortfolioRequest.assets` → `weights` (`PortfolioWeightRequest`); `CreateRunRequest` is `{ account_id, run_type, amount, weights }` (no `portfolio_id`); read `Portfolio.weights` instead of `assets`; `GetRunsRequest.run_type` takes `RunType`.
- **Batch journals**: `CreateReverseBatchJournalRequest` has a top-level `to_account` and entries without one; `BatchJournalEntryRequest.amount` is `Option<String>` and the struct has more optional fields; `CreateBatchJournalRequest` has `correspondent`.
- **`CreateAccountRequest`**: wrap `contact` / `identity` / `disclosures` / `agreements` in `Some(..)`, and use `..Default::default()` for the rest.
- **`TradeAccount.account_number`** is `Option<String>`.
- **Removed fields**: `Asset.easy_to_borrow` → `borrow_status`; `TokenizationRequest.account` / `issuer_account` → `client_account_id` / `client_external_account_id`.
- New fields on structs you build with literal syntax (request types, `Order`, `Asset`, …) need adding, or use `..Default::default()` where the type implements `Default`.

### Changed
- **Breaking**: `reqwest` 0.12 → 0.13. `AlpacaError::Http` wraps `reqwest::Error`, so it now carries the 0.13 error type; code that names `reqwest::Error` (e.g. matching on `AlpacaError::Http(e)` and calling reqwest-specific methods on `e`) must use reqwest 0.13. `cargo-semver-checks` does not detect this kind of change, so it is called out here.
- `reqwest` now uses `default-features = false` with `json`, `query`, `native-tls`, `charset`, `http2` and `system-proxy`. 0.13 made rustls/aws-lc-rs the default TLS backend; the crate stays on `native-tls`, the same backend `tokio-tungstenite` uses, so there is still one TLS stack and no new license or C-toolchain requirements.
- **Breaking**: `trading::GetAccountActivitiesRequest.activity_type: Option<ActivityType>` is now `activity_types: Option<Vec<ActivityType>>`, sent as the comma-separated `activity_types` query parameter the API actually reads. Its `page_token` is now `Option<String>`: activity IDs (and therefore page tokens) have the form `"<timestamp>::<uuid>"`, so the old `Option<Uuid>` made pagination impossible.
- **Breaking**: `BrokerClient::get_account_activities(account_id, filter)` is now `get_account_activities(filter)`; set `account_id` on the filter, or leave it unset to query across all accounts.
- **Breaking**: every public enum is now `#[non_exhaustive]`, so `match`es on them need a `_ =>` arm. In exchange, Alpaca adding a value is no longer a breaking change for this crate, so future additions can ship in minor releases.
- **Breaking**: the 44 enums that appear in API responses gained an `Unknown` variant (`#[serde(other)]`). A value this version does not recognize now parses as `Unknown` instead of failing the entire response — previously, for example, one record with a new tokenization issuer made `get_tokenization_requests` return an error. `Unknown` is not meaningful in requests; it serializes as `"unknown"`. Request-only enums (`QueryOrderStatus`, `OrderDestination`, `Adjustment`, …) are `#[non_exhaustive]` but have no `Unknown`.
- **Breaking**: `TradingClient::mint_tokenized_asset` and `BrokerClient::mint_tokenized_asset_for_account` take a new `idempotency_key: Option<&str>`, sent as the `Idempotency-Key` header (and replayed on retries). Pass `None` for the previous behavior.
- **Breaking**: `CreateAccountRequest` fields `contact`, `identity`, `disclosures` and `agreements` are now `Option`, because requirements vary by account type (an existing-holder IRA or donor-advised account sends none of them). The struct now implements `Default`.

### Added
- ACATS transfers (Beta, `/v1beta1/acats`) on `BrokerClient`: `initiate_acats_transfer`, `list_acats_transfers` (firm-wide, with status / type / direction / contra-broker / time filters and sorting), `list_acats_transfers_for_account`, `get_acats_transfer`, `list_acats_transfer_assets`, `list_acats_contra_brokers`. Lists are paginated by `page_token_next`. `AcatsTransferDetails::{Full, Partial}` makes a full transfer carrying assets — which the API rejects — unrepresentable, and `AcatsSecurityId::{Cusip, Symbol}` enforces the one-identifier rule for equities (prefer CUSIP). Twelve new `Acats*` enums.
- `BrokerClient::create_agreement` (`POST /v1/agreements`) to sign party-level or per-account agreements for an existing party, with `CreateAgreementRequest::{Party, Account}`, `SignedAgreement`, and the enums `PartyAgreementType`, `PerAccountAgreementType`, `SignedAgreementType`.
- `Order.group_id` / `Order.group_role` (new enum `OrderGroupRole`: `Entry`, `TakeProfit`, `StopLoss`), identifying the members of a `bracket`, `oto` or `oco` order — useful on the trade-updates stream, where `legs` is not populated.
- `OptionContract.ppind` (Penny Program Indicator).
- `Asset.margin_requirement_long` / `margin_requirement_short` (decimal strings), which replace the deprecated `maintenance_margin_requirement`.
- `CreateAccountRequest.account_type`, `account_sub_type`, `primary_account_holder_id`, `entity_id`, `minor_identity`, covering custodial, IRA, donor-advised and existing-holder account creation.
- Corporate actions: `CorporateActionsRequest.data_quality` (`complete` / `all`), `cusips`, `region`, `ids`; `CorporateActionsSet.capital_gains_distributions`, `partial_calls`, `reorganizations` (merged across pages like the other types); `CorporateActionsType::CapitalGainsDistribution`, `PartialCall`, `Reorganization`. `CorporateActionsClient::new_with_url` (`#[doc(hidden)]`) for tests.
- `GetEventsRequest.since_ulid` / `until_ulid`; `since_id` / `until_id` are deprecated by Alpaca and removed on 2027-02-15.
- `order_id` filter on both `GetAccountActivitiesRequest`s; `activity_types` and `category` on the Broker one.
- `ActivityType` variants `Cgd`, `Divfee`, `Divft`, `Divtw`, `Fopt`, `Intnra`, `Inttw`, `Jnl`, `Misc`, `Opca`, `Ptr`, `Reo`, `Trans`, completing the set in the Trading and Broker API references. **Breaking** for exhaustive `match`es.
- Enum values Alpaca added since the last sync, each checked against the API reference:
  - `TokenizationNetwork`: `Cronos`, `Hyperevm`, `Robinhood`, `Hypercore`.
  - `TokenizationIssuer`: `Binance`, `Coinbase`, `Ondo`.
  - Broker `AccountType`: `Trust`, `OmnibusNonDisclosed`, `OmnibusSub`, `Joint` (returned on reads; not creatable through the API).
  - `JournalStatus`: `Queued`, `SentToClearing`, `Deleted`.

### Removed
- **Breaking**: fields Alpaca has stopped returning, which were therefore always `None`:
  - `Asset.easy_to_borrow` (removed by Alpaca 2026-09-22) — use `borrow_status`.
  - `TokenizationRequest.account` and `TokenizationRequest.issuer_account` (removed 2026-10-15) — use `client_account_id` and `client_external_account_id`.

  Responses that still include these keys continue to parse.

### Fixed
- `CorporateActionsClient` called `/v1beta1/corporate-actions`; the endpoint is `/v1/corporate-actions`.
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

[Unreleased]: https://github.com/kfuangsung/alpacars/compare/v0.6.2...HEAD
[0.6.2]: https://github.com/kfuangsung/alpacars/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/kfuangsung/alpacars/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/kfuangsung/alpacars/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/kfuangsung/alpacars/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/kfuangsung/alpacars/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/kfuangsung/alpacars/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/kfuangsung/alpacars/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/kfuangsung/alpacars/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kfuangsung/alpacars/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/kfuangsung/alpacars/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/kfuangsung/alpacars/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/kfuangsung/alpacars/releases/tag/v0.1.0
