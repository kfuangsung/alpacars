//! Response fields Alpaca added between 2026-08 and 2026-10, in the shapes the API references
//! document.

use alpacars::trading::enums::OrderGroupRole;
use alpacars::trading::models::{Asset, OptionContract, Order};
use serde_json::json;

#[test]
fn test_order_group_fields() {
    // A bracket take-profit leg (changelog 2026-10-08; Order.group_id / OrderGroupRole).
    let order: Order = serde_json::from_value(json!({
        "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
        "client_order_id": "eb9e2aaa-f71a-4f51-b5b4-52a6c565dad4",
        "order_class": "bracket",
        "time_in_force": "gtc",
        "status": "new",
        "extended_hours": false,
        "group_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
        "group_role": "take_profit"
    }))
    .unwrap();
    assert_eq!(
        order.group_id,
        Some("3fa85f64-5717-4562-b3fc-2c963f66afa6".parse().unwrap())
    );
    assert_eq!(order.group_role, Some(OrderGroupRole::TakeProfit));

    // Both keys are omitted for orders outside a bracket / oto / oco group.
    let simple: Order = serde_json::from_value(json!({
        "id": "61e69015-8549-4bfd-b9c3-01e75843f47d",
        "client_order_id": "c1",
        "order_class": "simple",
        "time_in_force": "day",
        "status": "new",
        "extended_hours": false
    }))
    .unwrap();
    assert_eq!(simple.group_id, None);
    assert_eq!(simple.group_role, None);
}

#[test]
fn test_option_contract_ppind() {
    let contract: OptionContract = serde_json::from_value(json!({
        "id": "6e58f870-fe73-4583-81e4-b9a37892c36f",
        "symbol": "AAPL250620C00100000",
        "ppind": true
    }))
    .unwrap();
    assert_eq!(contract.ppind, Some(true));
}

#[test]
fn test_asset_margin_requirements_are_decimal_strings() {
    // Examples from the Assets schema: "100" long, "30" short.
    let asset: Asset = serde_json::from_value(json!({
        "id": "b0b6dd9d-8b9b-48a9-ba46-b9d54906e415",
        "class": "us_equity",
        "exchange": "NASDAQ",
        "symbol": "AAPL",
        "status": "active",
        "tradable": true,
        "marginable": true,
        "shortable": true,
        "fractionable": true,
        "margin_requirement_long": "100",
        "margin_requirement_short": "30"
    }))
    .unwrap();
    assert_eq!(asset.margin_requirement_long.as_deref(), Some("100"));
    assert_eq!(asset.margin_requirement_short.as_deref(), Some("30"));
}
