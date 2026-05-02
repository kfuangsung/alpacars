use crate::trading::enums::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionLegRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub ratio_qty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakeProfitRequest {
    pub limit_price: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopLossRequest {
    pub stop_price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_percent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_hours: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_class: Option<OrderClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<TakeProfitRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<StopLossRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_intent: Option<PositionIntent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<OptionLegRequest>>,
}

impl OrderRequest {
    pub fn market(symbol: impl Into<String>, side: OrderSide, qty: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            notional: None,
            side,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }

    pub fn market_notional(symbol: impl Into<String>, side: OrderSide, notional: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            qty: None,
            notional: Some(notional.into()),
            side,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }

    pub fn limit(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        limit_price: impl Into<String>,
        time_in_force: TimeInForce,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            notional: None,
            side,
            order_type: OrderType::Limit,
            time_in_force,
            limit_price: Some(limit_price.into()),
            stop_price: None,
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }

    pub fn stop(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        stop_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            notional: None,
            side,
            order_type: OrderType::Stop,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: Some(stop_price.into()),
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }

    pub fn stop_limit(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        stop_price: impl Into<String>,
        limit_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            notional: None,
            side,
            order_type: OrderType::StopLimit,
            time_in_force: TimeInForce::Day,
            limit_price: Some(limit_price.into()),
            stop_price: Some(stop_price.into()),
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }

    pub fn trailing_stop(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        trail_price: Option<String>,
        trail_percent: Option<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            qty: Some(qty.into()),
            notional: None,
            side,
            order_type: OrderType::TrailingStop,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            trail_price,
            trail_percent,
            extended_hours: None,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOrderByIdRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ReplaceOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ClosePositionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetPortfolioHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intraday_reporting: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl_reset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAssetsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssetStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<AssetClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange: Option<AssetExchange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCalendarRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWatchlistRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWatchlistRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCorporateAnnouncementsRequest {
    pub ca_types: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_type: Option<CorporateActionDateType>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetOptionContractsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AssetStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_gte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_lte: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<ContractType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ExerciseStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price_lte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct CancelAllOrdersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_orders: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSymbolToWatchlistRequest {
    pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountActivitiesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<ActivityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
