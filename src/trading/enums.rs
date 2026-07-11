use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Canceled,
    Expired,
    Replaced,
    PendingCancel,
    PendingReplace,
    PendingReview,
    Accepted,
    PendingNew,
    AcceptedForBidding,
    Stopped,
    Rejected,
    Suspended,
    Calculated,
    Held,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderClass {
    Simple,
    Mleg,
    Bracket,
    Oco,
    Oto,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    Day,
    Gtc,
    Opg,
    Cls,
    Ioc,
    Fok,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    UsEquity,
    UsOption,
    Crypto,
    CryptoPerp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetExchange {
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "ARCA")]
    Arca,
    #[serde(rename = "ASCX")]
    Ascx,
    #[serde(rename = "BATS")]
    Bats,
    #[serde(rename = "NYSE")]
    Nyse,
    #[serde(rename = "NASDAQ")]
    Nasdaq,
    #[serde(rename = "NYSEARCA")]
    Nysearca,
    #[serde(rename = "FTXU")]
    Ftxu,
    #[serde(rename = "CBSE")]
    Cbse,
    #[serde(rename = "GNSS")]
    Gnss,
    #[serde(rename = "ERSX")]
    Ersx,
    #[serde(rename = "OTC")]
    Otc,
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionSide {
    Long,
    Short,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorporateActionType {
    Dividend,
    Merger,
    Spinoff,
    Split,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorporateActionSubType {
    Cash,
    Stock,
    MergerUpdate,
    MergerCompletion,
    Spinoff,
    StockSplit,
    UnitSplit,
    ReverseSplit,
    Recapitalization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    AccountClosed,
    AccountUpdated,
    ActionRequired,
    Active,
    AmlReview,
    ApprovalPending,
    Approved,
    Disabled,
    DisablePending,
    Edited,
    Inactive,
    KycSubmitted,
    Limited,
    Onboarding,
    PaperOnly,
    ReapprovalPending,
    Rejected,
    Resubmitted,
    SignedUp,
    SubmissionFailed,
    Submitted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorporateActionDateType {
    DeclarationDate,
    ExDate,
    RecordDate,
    PayableDate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeEvent {
    Accepted,
    Canceled,
    Expired,
    Fill,
    New,
    PartialFill,
    PendingCancel,
    PendingNew,
    PendingReplace,
    Rejected,
    Replaced,
    Restated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryOrderStatus {
    Open,
    Closed,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TradeConfirmationEmail {
    All,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContractType {
    Call,
    Put,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseStyle {
    American,
    European,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    Fill,
    Acatc,
    Acats,
    Cfee,
    Cil,
    Csd,
    Csw,
    Div,
    Divcgl,
    Divcgs,
    Divnra,
    Divroc,
    Divtxex,
    Divwh,
    Extrd,
    Fee,
    Fxtrd,
    Int,
    Intpnl,
    Jnlc,
    Jnls,
    Ma,
    Mem,
    Nc,
    Oct,
    Opasn,
    Opcsh,
    Opexc,
    Opexp,
    Optrd,
    Ptc,
    Reorg,
    Spin,
    Split,
    Swp,
    Vof,
    Wh,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeActivityType {
    PartialFill,
    Fill,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NonTradeActivityStatus {
    Executed,
    Correct,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PositionIntent {
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
}
