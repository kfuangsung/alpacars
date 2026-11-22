use crate::common::enums::Sort;
use crate::trading::enums::*;
use jiff::{civil::Date, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One leg of a multi-leg (`mleg`) options order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptionLegRequest {
    /// Contract symbol or asset ID.
    pub symbol: String,
    /// Quantity of this leg relative to the order's `qty`, as a decimal string (`"1"`).
    pub ratio_qty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_intent: Option<PositionIntent>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdvancedInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<AdvancedAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<OrderDestination>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_qty: Option<String>,
}

/// Body of `submit_order` (and Broker `submit_order_for_account`).
///
/// The constructors cover single-asset orders; [`OrderRequest::mleg`] builds a multi-leg
/// options order, which has no top-level `symbol` or `side`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Symbol, asset ID or currency pair. Required for every order class except `mleg`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
    /// Required except on `mleg` orders, whose legs carry their own sides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_instructions: Option<AdvancedInstructions>,
    /// Broker API only: the commission to collect from the end user, interpreted per
    /// `commission_type`. On an `mleg` order it is a per-contract rate set on the parent
    /// order (leg commissions are ignored) and needs `commission_type: Qty` when non-zero.
    /// Commission fields cannot be changed on a replace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission: Option<String>,
    /// Broker API only. The API defaults to `Notional`, except on `mleg` orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_type: Option<CommissionType>,
    /// Broker API only. Deprecated by Alpaca: set `commission_type: Bps` and put the bps value
    /// in `commission` instead. Not supported on `mleg` orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_bps: Option<String>,
}

impl OrderRequest {
    pub fn market(symbol: impl Into<String>, side: OrderSide, qty: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            qty: Some(qty.into()),
            notional: None,
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
        }
    }

    pub fn market_notional(
        symbol: impl Into<String>,
        side: OrderSide,
        notional: impl Into<String>,
    ) -> Self {
        Self {
            symbol: Some(symbol.into()),
            qty: None,
            notional: Some(notional.into()),
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
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
            symbol: Some(symbol.into()),
            qty: Some(qty.into()),
            notional: None,
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
        }
    }

    pub fn stop(
        symbol: impl Into<String>,
        side: OrderSide,
        qty: impl Into<String>,
        stop_price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: Some(symbol.into()),
            qty: Some(qty.into()),
            notional: None,
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
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
            symbol: Some(symbol.into()),
            qty: Some(qty.into()),
            notional: None,
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
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
            symbol: Some(symbol.into()),
            qty: Some(qty.into()),
            notional: None,
            side: Some(side),
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
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
        }
    }

    /// A multi-leg options market order of `qty` strategy units. For a limit order, set
    /// `order_type` to `OrderType::Limit` and `limit_price` to the net price.
    pub fn mleg(qty: impl Into<String>, legs: Vec<OptionLegRequest>) -> Self {
        Self {
            symbol: None,
            qty: Some(qty.into()),
            notional: None,
            side: None,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            limit_price: None,
            stop_price: None,
            trail_price: None,
            trail_percent: None,
            extended_hours: None,
            client_order_id: None,
            order_class: Some(OrderClass::Mleg),
            take_profit: None,
            stop_loss: None,
            position_intent: None,
            legs: Some(legs),
            advanced_instructions: None,
            commission: None,
            commission_type: None,
            commission_bps: None,
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
    pub after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    /// Only orders in these asset classes, sent comma-separated. Omit for all classes. With
    /// `us_option`, `symbols` filters option orders by underlying.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<Vec<AssetClass>>,
    /// Orders submitted before this order (exclusive). Do not combine with `after_order_id`,
    /// `after` or `until`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_order_id: Option<Uuid>,
    /// Orders submitted after this order (exclusive). Do not combine with `before_order_id`,
    /// `after` or `until`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_order_id: Option<Uuid>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_instructions: Option<AdvancedInstructions>,
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
    pub timeframe: Option<PortfolioHistoryTimeframe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intraday_reporting: Option<IntradayReporting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pnl_reset: Option<PnlReset>,
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
    /// Assets with any of these attributes, sent comma-separated.
    pub attributes: Option<Vec<AssetAttribute>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetCalendarRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_type: Option<CalendarDateType>,
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
    /// Sent comma-separated.
    pub ca_types: Vec<CorporateActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<Date>,
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
    pub expiration_date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_gte: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date_lte: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
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
pub struct CreateLocateRequest {
    pub symbol: String,
    pub qty: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_or_none: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetLocatesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LocateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetLocateQuotesRequest {
    pub symbols: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddSymbolToWatchlistRequest {
    pub symbol: String,
}

/// Date range and display timezone for [`get_market_calendar`](crate::trading::client::TradingClient::get_market_calendar).
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMarketCalendarRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Date>,
    /// IANA timezone the session times are expressed in (they are parsed to UTC either way).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetAccountActivitiesRequest {
    /// Sent as a comma-separated `activity_types` query parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<Vec<ActivityType>>,
    /// Only return activities (such as fills) belonging to this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// The `id` of the last activity on the previous page (`"<timestamp>::<uuid>"` format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// `trade_activity` or `non_trade_activity`. Mutually exclusive with `activity_types`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<ActivityCategory>,
}

/// Request to convert underlying equity into a tokenized asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationMintRequest {
    pub underlying_symbol: String,
    /// Underlying quantity to convert. May be fractional.
    pub qty: String,
    pub issuer: TokenizationIssuer,
    pub network: TokenizationNetwork,
    /// Wallet address that receives the tokenized asset.
    pub wallet_address: String,
    /// Optional correlation label, at most 128 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct GetTokenizationRequestsRequest {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub request_type: Option<TokenizationRequestType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TokenizationRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<TokenizationIssuer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<TokenizationNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Timestamp>,
}
