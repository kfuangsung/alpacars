# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/kfuangsung/alpacars/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/kfuangsung/alpacars/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/kfuangsung/alpacars/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/kfuangsung/alpacars/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/kfuangsung/alpacars/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/kfuangsung/alpacars/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/kfuangsung/alpacars/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/kfuangsung/alpacars/releases/tag/v0.1.0
