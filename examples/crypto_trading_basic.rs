//! Crypto Trading Basic — mirrors alpaca-py/examples/crypto-trading-basic.ipynb
//!
//! Run with:
//!   APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example crypto_trading_basic

use alpacars::data::historical::crypto::{CryptoBarsRequest, CryptoHistoricalDataClient, CryptoLatestRequest};
use alpacars::trading::client::TradingClient;
use alpacars::trading::enums::{AssetClass, AssetStatus, OrderSide, TimeInForce};
use alpacars::trading::requests::{
    ClosePositionRequest, GetAssetsRequest, GetOrdersRequest, OrderRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID not set");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY not set");

    // ── Trading client (paper) ────────────────────────────────────────────────
    let trading = TradingClient::new(&api_key, &secret_key, true)?;

    // ── Account info ─────────────────────────────────────────────────────────
    let account = trading.get_account().await?;
    println!("Account status : {:?}", account.status);
    println!("Buying power   : {:?}", account.buying_power);

    // ── List active crypto assets ────────────────────────────────────────────
    let assets = trading
        .get_all_assets(Some(&GetAssetsRequest {
            status: Some(AssetStatus::Active),
            asset_class: Some(AssetClass::Crypto),
            ..Default::default()
        }))
        .await?;
    println!("Active crypto assets: {}", assets.len());

    // ── Place orders on BTC/USD ──────────────────────────────────────────────

    // Market order — 0.01 BTC
    let market_order = trading
        .submit_order(&OrderRequest::market("BTC/USD", OrderSide::Buy, "0.01"))
        .await?;
    println!("Market order id: {:?}", market_order.id);

    // Market order — $1.11 notional
    let notional_order = trading
        .submit_order(&OrderRequest::market_notional("BTC/USD", OrderSide::Buy, "1.11"))
        .await?;
    println!("Notional order id: {:?}", notional_order.id);

    // Limit order
    let limit_order = trading
        .submit_order(&OrderRequest::limit(
            "BTC/USD",
            OrderSide::Buy,
            "0.01",
            "60000.00",
            TimeInForce::Gtc,
        ))
        .await?;
    println!("Limit order id: {:?}", limit_order.id);

    // Stop-limit order
    let stop_limit_order = trading
        .submit_order(&OrderRequest::stop_limit(
            "BTC/USD",
            OrderSide::Sell,
            "0.01",
            "58000.00",
            "57900.00",
        ))
        .await?;
    println!("Stop-limit order id: {:?}", stop_limit_order.id);

    // ── Query orders ─────────────────────────────────────────────────────────
    let all_orders = trading
        .get_orders(Some(&GetOrdersRequest {
            status: Some(alpacars::trading::enums::QueryOrderStatus::All),
            ..Default::default()
        }))
        .await?;
    println!("Total orders: {}", all_orders.len());

    let open_orders = trading
        .get_orders(Some(&GetOrdersRequest {
            status: Some(alpacars::trading::enums::QueryOrderStatus::Open),
            ..Default::default()
        }))
        .await?;
    println!("Open orders: {}", open_orders.len());

    // ── Cancel all open orders ───────────────────────────────────────────────
    let cancelled = trading.cancel_orders().await?;
    println!("Cancelled {} order(s).", cancelled.len());

    // ── Positions ────────────────────────────────────────────────────────────
    let positions = trading.get_all_positions().await?;
    println!("Open positions: {}", positions.len());

    // Note: Alpaca uses "BTCUSD" (no slash) for position queries
    match trading.get_open_position("BTCUSD").await {
        Ok(pos) => {
            println!("BTC/USD position: qty={} market_value={:?}", pos.qty, pos.market_value);

            // Close the BTC position
            trading
                .close_position(
                    "BTCUSD",
                    Some(&ClosePositionRequest {
                        percentage: Some("100".to_string()),
                        qty: None,
                    }),
                )
                .await?;
            println!("BTC/USD position closed.");
        }
        Err(e) => println!("No BTC/USD position: {}", e),
    }

    // ── Historical data ───────────────────────────────────────────────────────
    // CryptoHistoricalDataClient does not require auth for free data
    let data_client = CryptoHistoricalDataClient::new(Some(&api_key), Some(&secret_key))?;

    let bars = data_client
        .get_crypto_bars(&CryptoBarsRequest {
            symbols: vec!["BTC/USD".to_string(), "ETH/USD".to_string()],
            ..Default::default()
        })
        .await?;
    for (sym, sym_bars) in &bars {
        if let Some(last) = sym_bars.last() {
            println!("{} latest bar: open={} close={}", sym, last.open, last.close);
        }
    }

    let latest_quote = data_client
        .get_crypto_latest_quote(&CryptoLatestRequest {
            symbols: vec!["BTC/USD".to_string()],
            ..Default::default()
        })
        .await?;
    if let Some(q) = latest_quote.get("BTC/USD") {
        println!("BTC/USD latest ask: {} bid: {}", q.ask_price, q.bid_price);
    }

    Ok(())
}
