//! Stocks Trading Basic — mirrors alpaca-py/examples/stocks-trading-basic.ipynb
//!
//! Run with:
//!   APCA_API_KEY_ID=<key> APCA_API_SECRET_KEY=<secret> cargo run --example stocks_trading_basic

use alpaca_rs::data::enums::DataFeed;
use alpaca_rs::data::historical::stock::{
    StockBarsRequest, StockHistoricalDataClient, StockLatestRequest,
};
use alpaca_rs::trading::client::TradingClient;
use alpaca_rs::trading::enums::{AssetClass, AssetExchange, AssetStatus, OrderSide, TimeInForce};
use alpaca_rs::trading::models::AccountConfiguration;
use alpaca_rs::trading::requests::{
    GetAssetsRequest, GetOrdersRequest, OrderRequest, StopLossRequest, TakeProfitRequest,
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
    println!("Equity         : {:?}", account.equity);

    // ── Account configuration ────────────────────────────────────────────────
    let config = trading.get_account_configurations().await?;
    println!("Fractional trading: {:?}", config.fractional_trading);

    // Enable fractional trading
    let new_config = AccountConfiguration {
        fractional_trading: Some(true),
        ..config.clone()
    };
    trading.set_account_configurations(&new_config).await?;
    println!("Fractional trading enabled.");

    // ── List active NASDAQ stocks ────────────────────────────────────────────
    let assets = trading
        .get_all_assets(Some(&GetAssetsRequest {
            status: Some(AssetStatus::Active),
            asset_class: Some(AssetClass::UsEquity),
            exchange: Some(AssetExchange::Nasdaq),
            ..Default::default()
        }))
        .await?;
    println!("Active NASDAQ assets: {}", assets.len());

    // ── Place orders on SPY ──────────────────────────────────────────────────

    // Market order — 5 shares
    let market_order = trading
        .submit_order(&OrderRequest::market("SPY", OrderSide::Buy, "5"))
        .await?;
    println!("Market order id: {:?}", market_order.id);

    // Market order — $1.11 notional (fractional)
    let notional_order = trading
        .submit_order(&OrderRequest::market_notional("SPY", OrderSide::Buy, "1.11"))
        .await?;
    println!("Notional order id: {:?}", notional_order.id);

    // Limit order
    let limit_order = trading
        .submit_order(&OrderRequest::limit(
            "SPY",
            OrderSide::Buy,
            "1",
            "550.25",
            TimeInForce::Day,
        ))
        .await?;
    println!("Limit order id: {:?}", limit_order.id);

    // Stop order
    let stop_order = trading
        .submit_order(&OrderRequest::stop("SPY", OrderSide::Buy, "1", "600.00"))
        .await?;
    println!("Stop order id: {:?}", stop_order.id);

    // Stop-limit order
    let stop_limit_order = trading
        .submit_order(&OrderRequest::stop_limit(
            "SPY",
            OrderSide::Buy,
            "1",
            "600.00",
            "601.00",
        ))
        .await?;
    println!("Stop-limit order id: {:?}", stop_limit_order.id);

    // Bracket order (market + take-profit + stop-loss)
    let mut bracket_order = OrderRequest::market("SPY", OrderSide::Buy, "1");
    bracket_order.order_class = Some(alpaca_rs::trading::enums::OrderClass::Bracket);
    bracket_order.take_profit = Some(TakeProfitRequest { limit_price: "620.00".to_string() });
    bracket_order.stop_loss = Some(StopLossRequest {
        stop_price: "550.00".to_string(),
        limit_price: None,
    });
    let bracket = trading.submit_order(&bracket_order).await?;
    println!("Bracket order id: {:?}", bracket.id);

    // OTO order (limit + stop-loss leg)
    let mut oto_order = OrderRequest::limit("SPY", OrderSide::Buy, "1", "555.00", TimeInForce::Day);
    oto_order.order_class = Some(alpaca_rs::trading::enums::OrderClass::Oto);
    oto_order.stop_loss = Some(StopLossRequest {
        stop_price: "540.00".to_string(),
        limit_price: None,
    });
    let oto = trading.submit_order(&oto_order).await?;
    println!("OTO order id: {:?}", oto.id);

    // OCO order (limit buy with both take-profit and stop-loss already open)
    let mut oco_order = OrderRequest::limit("SPY", OrderSide::Sell, "1", "600.00", TimeInForce::Gtc);
    oco_order.order_class = Some(alpaca_rs::trading::enums::OrderClass::Oco);
    oco_order.stop_loss = Some(StopLossRequest {
        stop_price: "540.00".to_string(),
        limit_price: None,
    });
    let oco = trading.submit_order(&oco_order).await?;
    println!("OCO order id: {:?}", oco.id);

    // Trailing-stop order (20% trail)
    let trail_order = trading
        .submit_order(&OrderRequest::trailing_stop(
            "SPY",
            OrderSide::Sell,
            "1",
            None,
            Some("20".to_string()),
        ))
        .await?;
    println!("Trailing-stop order id: {:?}", trail_order.id);

    // ── Query open orders ────────────────────────────────────────────────────
    let open_orders = trading
        .get_orders(Some(&GetOrdersRequest {
            status: Some(alpaca_rs::trading::enums::QueryOrderStatus::Open),
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

    if let Some(pos) = positions.first() {
        let sym = pos.symbol.as_str();
        println!("First position: {} qty={}", sym, pos.qty);

        let pos_detail = trading.get_open_position(sym).await?;
        println!("  Market value: {:?}", pos_detail.market_value);

        // Close a portion (3 shares)
        trading
            .close_position(
                sym,
                Some(&alpaca_rs::trading::requests::ClosePositionRequest {
                    qty: Some("3".to_string()),
                    percentage: None,
                }),
            )
            .await?;
        println!("  Closed 3 shares of {}.", sym);
    }

    // ── Historical data ───────────────────────────────────────────────────────
    let data_client = StockHistoricalDataClient::new(Some(&api_key), Some(&secret_key), false)?;

    let bars = data_client
        .get_stock_bars(&StockBarsRequest {
            symbols: vec!["SPY".to_string(), "AAPL".to_string()],
            ..Default::default()
        })
        .await?;
    for (sym, sym_bars) in &bars {
        println!("{} has {} bar(s). Latest close: {}", sym, sym_bars.len(),
            sym_bars.last().map(|b| b.close).unwrap_or(0.0));
    }

    let latest_quote = data_client
        .get_stock_latest_quote(&StockLatestRequest {
            symbols: vec!["SPY".to_string()],
            feed: Some(DataFeed::Iex),
            ..Default::default()
        })
        .await?;
    if let Some(q) = latest_quote.get("SPY") {
        println!("SPY latest ask: {} bid: {}", q.ask_price, q.bid_price);
    }

    Ok(())
}
