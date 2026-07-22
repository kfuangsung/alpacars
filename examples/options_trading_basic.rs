//! Options Trading Basic — mirrors alpaca-py/examples/options-trading-basic.ipynb
//!
//! Run with:
//!   APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example options_trading_basic

use alpacars::data::historical::option::{
    OptionBarsRequest, OptionHistoricalDataClient, OptionLatestRequest, OptionSnapshotRequest,
};
use alpacars::trading::client::TradingClient;
use alpacars::trading::enums::{AssetStatus, ContractType, ExerciseStyle, OrderSide};
use alpacars::trading::requests::{ClosePositionRequest, GetOptionContractsRequest, OrderRequest};
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID not set");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY not set");

    // ── Trading client (paper) ────────────────────────────────────────────────
    let trading = TradingClient::new(&api_key, &secret_key, true)?;

    // ── Check options capability ──────────────────────────────────────────────
    let account = trading.get_account().await?;
    println!(
        "Options buying power   : {:?}",
        account.options_buying_power
    );
    println!(
        "Options approved level : {:?}",
        account.options_approved_level
    );
    println!(
        "Options trading level  : {:?}",
        account.options_trading_level
    );

    let config = trading.get_account_configurations().await?;
    println!(
        "Max options trading level: {:?}",
        config.max_options_trading_level
    );

    // ── Discover SPY option contracts ─────────────────────────────────────────
    let today = Utc::now().date_naive();
    let expiry_min = today + Duration::days(7);
    let expiry_max = today + Duration::days(60);

    let contracts_resp = trading
        .get_option_contracts(&GetOptionContractsRequest {
            underlying_symbols: Some("SPY".to_string()),
            status: Some(AssetStatus::Active),
            expiration_date_gte: Some(expiry_min),
            expiration_date_lte: Some(expiry_max),
            contract_type: Some(ContractType::Call),
            style: Some(ExerciseStyle::American),
            limit: Some(20),
            ..Default::default()
        })
        .await?;
    let contracts = contracts_resp.option_contracts;
    println!("Found {} SPY call contracts.", contracts.len());

    // Pick the contract with the highest open interest
    let best = contracts.iter().max_by_key(|c| {
        c.open_interest
            .as_deref()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    });

    let Some(contract) = best else {
        println!("No contracts found — exiting.");
        return Ok(());
    };
    println!(
        "Selected contract: {} expiry={:?} strike={:?} OI={:?}",
        contract.symbol, contract.expiration_date, contract.strike_price, contract.open_interest
    );

    // ── Fetch a single contract by symbol ─────────────────────────────────────
    let c = trading.get_option_contract(&contract.symbol).await?;
    println!("Contract details: {:?}", c.contract_type);

    // ── Place a market order for 1 contract ───────────────────────────────────
    let order = trading
        .submit_order(&OrderRequest::market(&contract.symbol, OrderSide::Buy, "1"))
        .await?;
    println!("Options market order id: {:?}", order.id);

    // ── Query positions ───────────────────────────────────────────────────────
    let positions = trading.get_all_positions().await?;
    let options_positions: Vec<_> = positions
        .iter()
        .filter(|p| p.symbol == contract.symbol)
        .collect();
    println!(
        "Options positions for {}: {}",
        contract.symbol,
        options_positions.len()
    );

    if let Some(pos) = options_positions.first() {
        // Close the options position
        trading
            .close_position(
                &pos.symbol,
                Some(&ClosePositionRequest {
                    qty: Some(pos.qty.clone()),
                    percentage: None,
                }),
            )
            .await?;
        println!("Closed options position for {}.", pos.symbol);

        // Alternatively: exercise the position
        // trading.exercise_options_position(&pos.symbol).await?;
    }

    // ── Options historical data ───────────────────────────────────────────────
    let data_client = OptionHistoricalDataClient::new(Some(&api_key), Some(&secret_key))?;

    // Exchange codes
    let codes = data_client.get_option_exchange_codes().await?;
    println!("Option exchanges: {}", codes);

    if !contracts.is_empty() {
        let sym = &contracts[0].symbol;

        // Latest quote and trade
        let latest_quote = data_client
            .get_option_latest_quote(&OptionLatestRequest {
                symbols: vec![sym.clone()],
                ..Default::default()
            })
            .await?;
        if let Some(q) = latest_quote.get(sym.as_str()) {
            println!("{} latest ask={} bid={}", sym, q.ask_price, q.bid_price);
        }

        let latest_trade = data_client
            .get_option_latest_trade(&OptionLatestRequest {
                symbols: vec![sym.clone()],
                ..Default::default()
            })
            .await?;
        if let Some(t) = latest_trade.get(sym.as_str()) {
            println!("{} latest trade price={} size={}", sym, t.price, t.size);
        }

        // Historical bars
        let bars = data_client
            .get_option_bars(&OptionBarsRequest {
                symbols: vec![sym.clone()],
                ..Default::default()
            })
            .await?;
        if let Some(sym_bars) = bars.get(sym.as_str()) {
            println!("{} has {} bar(s).", sym, sym_bars.len());
        }

        // Snapshot
        let snapshot = data_client
            .get_option_snapshot(&OptionSnapshotRequest {
                symbols: vec![sym.clone()],
                ..Default::default()
            })
            .await?;
        if let Some(s) = snapshot.get(sym.as_str()) {
            println!(
                "{} snapshot implied vol: {:?}",
                sym,
                s.greeks.as_ref().map(|g| g.vega)
            );
        }
    }

    // Option chain for SPY (all contracts for the underlying)
    let chain = data_client.get_option_chain("SPY", None).await?;
    let chain_count = chain.as_object().map(|m| m.len()).unwrap_or(0);
    println!("SPY option chain entries: {}", chain_count);

    Ok(())
}
