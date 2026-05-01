//! Options Multi-Leg (MLEG) Trading — mirrors alpaca-py/examples/options-trading-mleg.ipynb
//!
//! Demonstrates two strategies:
//!   1. Straddle  — buy a call + buy a put at the same strike/expiry
//!   2. Iron Condor — sell OTM call + buy further OTM call + sell OTM put + buy further OTM put
//!
//! Run with:
//!   APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example options_trading_mleg

use alpaca_rs::trading::client::TradingClient;
use alpaca_rs::trading::models::OptionContract;

fn closest_strike(contracts: &[OptionContract], target: f64) -> Option<&OptionContract> {
    contracts.iter().min_by(|a, b| {
        let da = a.strike_price.as_deref().and_then(|s| s.parse::<f64>().ok())
            .map(|v| (v - target).abs()).unwrap_or(f64::MAX);
        let db = b.strike_price.as_deref().and_then(|s| s.parse::<f64>().ok())
            .map(|v| (v - target).abs()).unwrap_or(f64::MAX);
        da.partial_cmp(&db).unwrap()
    })
}
use alpaca_rs::trading::enums::{AssetStatus, ContractType, OrderClass, OrderSide, TimeInForce};
use alpaca_rs::trading::requests::{
    GetOptionContractsRequest, OptionLegRequest, OrderRequest,
};
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID not set");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY not set");

    let trading = TradingClient::new(&api_key, &secret_key, true)?;

    // ── Verify account can trade multi-leg options (level >= 3) ───────────────
    let account = trading.get_account().await?;
    let options_level = account.options_trading_level.unwrap_or(0);
    println!("Options trading level: {}", options_level);
    if options_level < 3 {
        eprintln!("Multi-leg options require trading level 3 (current: {}). Exiting.", options_level);
        return Ok(());
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Strategy 1: Straddle on TSLA
    //   Buy 1 call + buy 1 put with the same strike & expiry
    // ─────────────────────────────────────────────────────────────────────────
    let today = Utc::now().date_naive();
    let expiry_min = today + Duration::days(20);
    let expiry_max = today + Duration::days(50);

    // Fetch TSLA calls
    let calls_resp = trading
        .get_option_contracts(&GetOptionContractsRequest {
            underlying_symbols: Some("TSLA".to_string()),
            status: Some(AssetStatus::Active),
            expiration_date_gte: Some(expiry_min),
            expiration_date_lte: Some(expiry_max),
            contract_type: Some(ContractType::Call),
            limit: Some(50),
            ..Default::default()
        })
        .await?;

    // Fetch TSLA puts
    let puts_resp = trading
        .get_option_contracts(&GetOptionContractsRequest {
            underlying_symbols: Some("TSLA".to_string()),
            status: Some(AssetStatus::Active),
            expiration_date_gte: Some(expiry_min),
            expiration_date_lte: Some(expiry_max),
            contract_type: Some(ContractType::Put),
            limit: Some(50),
            ..Default::default()
        })
        .await?;

    let calls = calls_resp.option_contracts;
    let puts = puts_resp.option_contracts;

    // Find a matched call + put at the same strike closest to ATM (choose highest OI call)
    let straddle_call = calls.iter().max_by_key(|c| {
        c.open_interest.as_deref().and_then(|v| v.parse::<u64>().ok()).unwrap_or(0)
    });
    let straddle_put = straddle_call.as_ref().and_then(|call| {
        puts.iter().find(|p| p.strike_price == call.strike_price && p.expiration_date == call.expiration_date)
    });

    if let (Some(call), Some(put)) = (straddle_call, straddle_put) {
        println!(
            "Straddle: call={} put={} strike={:?} expiry={:?}",
            call.symbol, put.symbol, call.strike_price, call.expiration_date
        );

        // Build MLEG market order with 2 legs
        let mut straddle_order = OrderRequest::market("", OrderSide::Buy, "1");
        straddle_order.symbol = call.symbol.clone(); // symbol unused for MLEG but required
        straddle_order.order_class = Some(OrderClass::Mleg);
        straddle_order.legs = Some(vec![
            OptionLegRequest { symbol: call.symbol.clone(), side: OrderSide::Buy, ratio_qty: 1 },
            OptionLegRequest { symbol: put.symbol.clone(),  side: OrderSide::Buy, ratio_qty: 1 },
        ]);

        match trading.submit_order(&straddle_order).await {
            Ok(order) => println!("Straddle order id: {:?}", order.id),
            Err(e)    => println!("Straddle order error (expected in paper): {}", e),
        }
    } else {
        println!("Could not find matching call+put for straddle.");
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Strategy 2: Iron Condor on SPY (4-leg spread)
    //   Sell OTM put  (strike B)  + Buy further OTM put  (strike A, A < B)
    //   Sell OTM call (strike C)  + Buy further OTM call (strike D, D > C)
    // ─────────────────────────────────────────────────────────────────────────
    let spy_calls_resp = trading
        .get_option_contracts(&GetOptionContractsRequest {
            underlying_symbols: Some("SPY".to_string()),
            status: Some(AssetStatus::Active),
            expiration_date_gte: Some(expiry_min),
            expiration_date_lte: Some(expiry_max),
            contract_type: Some(ContractType::Call),
            limit: Some(100),
            ..Default::default()
        })
        .await?;

    let spy_puts_resp = trading
        .get_option_contracts(&GetOptionContractsRequest {
            underlying_symbols: Some("SPY".to_string()),
            status: Some(AssetStatus::Active),
            expiration_date_gte: Some(expiry_min),
            expiration_date_lte: Some(expiry_max),
            contract_type: Some(ContractType::Put),
            limit: Some(100),
            ..Default::default()
        })
        .await?;

    let spy_calls = spy_calls_resp.option_contracts;
    let spy_puts  = spy_puts_resp.option_contracts;

    // Assume SPY is around 550; adjust based on actual price in production
    let spy_price = 550.0_f64;
    let wing_width = 10.0_f64;

    let short_call = closest_strike(&spy_calls, spy_price + wing_width);       // sell C
    let long_call  = closest_strike(&spy_calls, spy_price + wing_width * 2.0); // buy  D
    let short_put  = closest_strike(&spy_puts,  spy_price - wing_width);       // sell B
    let long_put   = closest_strike(&spy_puts,  spy_price - wing_width * 2.0); // buy  A

    if let (Some(sc), Some(lc), Some(sp), Some(lp)) = (short_call, long_call, short_put, long_put) {
        println!(
            "Iron Condor: sell_call={} buy_call={} sell_put={} buy_put={}",
            sc.symbol, lc.symbol, sp.symbol, lp.symbol
        );

        // Build MLEG limit order at $0 credit (net zero for demo)
        let mut condor_order = OrderRequest::limit("", OrderSide::Sell, "1", "0.00", TimeInForce::Day);
        condor_order.order_class = Some(OrderClass::Mleg);
        condor_order.legs = Some(vec![
            OptionLegRequest { symbol: sc.symbol.clone(), side: OrderSide::Sell, ratio_qty: 1 },
            OptionLegRequest { symbol: lc.symbol.clone(), side: OrderSide::Buy,  ratio_qty: 1 },
            OptionLegRequest { symbol: sp.symbol.clone(), side: OrderSide::Sell, ratio_qty: 1 },
            OptionLegRequest { symbol: lp.symbol.clone(), side: OrderSide::Buy,  ratio_qty: 1 },
        ]);

        match trading.submit_order(&condor_order).await {
            Ok(order) => {
                println!("Iron Condor order id: {:?}", order.id);

                // Query the placed order
                let queried = trading.get_order_by_id(&order.id, None).await?;
                println!("Order status: {:?}", queried.status);

                // Cancel the whole order (individual legs cannot be cancelled)
                trading.cancel_order_by_id(&order.id).await?;
                println!("Iron Condor order cancelled.");
            }
            Err(e) => println!("Iron Condor order error (expected in paper): {}", e),
        }
    } else {
        println!("Could not find all four legs for iron condor.");
    }

    Ok(())
}
