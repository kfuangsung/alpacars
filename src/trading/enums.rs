use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum OrderSide {
    Buy,
    Sell,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum OrderClass {
    Simple,
    Mleg,
    Bracket,
    Oco,
    Oto,
    #[serde(rename = "")]
    Empty,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TimeInForce {
    Day,
    Gtc,
    Opg,
    Cls,
    Ioc,
    Fok,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AssetClass {
    UsEquity,
    UsOption,
    Crypto,
    CryptoPerp,
    UsIndex,
    Treasury,
    Corporate,
    GlobalEquity,
    UsEquityChain,
    Ipo,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AssetStatus {
    Active,
    Inactive,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BorrowStatus {
    EasyToBorrow,
    HardToBorrow,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PositionSide {
    Long,
    Short,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CorporateActionType {
    Dividend,
    Merger,
    Spinoff,
    Split,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum AccountStatus {
    AccountClosed,
    AccountClosedPending,
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
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CorporateActionDateType {
    DeclarationDate,
    ExDate,
    RecordDate,
    PayableDate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
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
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum QueryOrderStatus {
    Open,
    Closed,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TradeConfirmationEmail {
    All,
    None,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ContractType {
    Call,
    Put,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ExerciseStyle {
    American,
    European,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Account activity type. Wire values are uppercase (`FILL`, `JNLC`, `DIVCGL`, ...).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum ActivityType {
    /// Order fill — the only trade activity type; every other value is a non-trade activity.
    Fill,
    Acatc,
    Acats,
    Cfee,
    /// Capital gains distribution (sub-types `LTCG`, `STCG`).
    Cgd,
    Cil,
    Csd,
    Csw,
    Div,
    Divcgl,
    Divcgs,
    Divfee,
    Divft,
    Divnra,
    Divroc,
    Divtw,
    Divtxex,
    Divwh,
    Extrd,
    Fee,
    Fopt,
    Fxtrd,
    Int,
    Intnra,
    Intpnl,
    Inttw,
    Jnl,
    Jnlc,
    Jnls,
    Ma,
    Mem,
    Misc,
    Nc,
    /// On-chain transaction (blockchain deposit or withdrawal).
    Oct,
    Opasn,
    Opca,
    Opcsh,
    Opexc,
    Opexp,
    Optrd,
    Ptc,
    Ptr,
    /// Reorganization (sub-types `REOS`, `REOC`, `REOSC`).
    Reo,
    /// Worthless removal (sub-type `WRM`); broader reorganizations are `REO`.
    Reorg,
    Spin,
    Split,
    Swp,
    Trans,
    Vof,
    Wh,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TradeActivityType {
    PartialFill,
    Fill,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum NonTradeActivityStatus {
    Executed,
    Correct,
    Canceled,
    Pending,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum LocateStatus {
    Active,
    Expired,
    Rejected,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum AdvancedAlgorithm {
    Dma,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum OrderDestination {
    Nyse,
    Nasdaq,
    Arca,
    Iex,
    Memx,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PositionIntent {
    BuyToOpen,
    BuyToClose,
    SellToOpen,
    SellToClose,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TokenizationRequestStatus {
    Pending,
    Rejected,
    Completed,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TokenizationRequestType {
    Mint,
    Redeem,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Blockchain network a tokenized asset is issued on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TokenizationNetwork {
    Solana,
    Arbitrum,
    Ethereum,
    Binance,
    Base,
    Ton,
    Tron,
    Mantle,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Issuer of a tokenized asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TokenizationIssuer {
    Xstocks,
    St0x,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}
