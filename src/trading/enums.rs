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
    #[serde(rename = "BATS")]
    Bats,
    #[serde(rename = "NYSE")]
    Nyse,
    #[serde(rename = "NASDAQ")]
    Nasdaq,
    #[serde(rename = "NYSEARCA")]
    Nysearca,
    #[serde(rename = "OTC")]
    Otc,
    /// Texas Stock Exchange.
    #[serde(rename = "TXSE")]
    Txse,
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "ERSX")]
    Ersx,
    #[serde(rename = "CRXL")]
    Crxl,
    /// Crypto perpetuals.
    #[serde(rename = "ASCX")]
    Ascx,
    /// The artificial exchange of index symbols such as SPX. As a `GetAssetsRequest` filter
    /// it requires `asset_class: UsIndex`.
    #[serde(rename = "INDEX")]
    Index,
    /// Returned on positions whose exchange is unset.
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

/// Corporate action announcement type (`ca_type`, and the `ca_types` filter). The wire values
/// are capitalized (`Dividend`), per the announcements reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CorporateActionType {
    Dividend,
    Merger,
    Spinoff,
    Split,
    Reorg,
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
    /// Done executing for the day; no further updates until the next trading day.
    DoneForDay,
    /// Stopped: a trade is guaranteed, usually at a stated price or better, but has not
    /// occurred yet.
    Stopped,
    /// Completed for the day (filled or done for day), with settlement calculations pending.
    Calculated,
    /// Suspended and not eligible for trading.
    Suspended,
    OrderReplaceRejected,
    OrderCancelRejected,
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
///
/// One enum serves both directions, as in alpaca-py: the `activity_types` filter and an
/// activity's `activity_type`. A few values work in only one direction; their docs say so.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum ActivityType {
    /// Order fills, partial and full. The only trade activity type; every other value is a non-trade activity.
    Fill,
    /// ACATS in/out (cash).
    Acatc,
    /// ACATS in/out (securities).
    Acats,
    /// Allocation.
    Alloc,
    /// Crypto fee.
    Cfee,
    /// Capital gains distribution (sub-types `LTCG`, `STCG`).
    Cgd,
    /// Cash in lieu of stock.
    Cil,
    /// Correction. Returned only; not accepted as a filter.
    Corr,
    /// Cash deposit (+), including returned withdrawals.
    Csd,
    /// Cash receipt (-).
    Csr,
    /// Cash withdrawal (-), including returned deposits.
    Csw,
    /// Dividends.
    Div,
    /// Dividend (long-term capital gains).
    Divcgl,
    /// Dividend (short-term capital gains).
    Divcgs,
    /// Dividend fee.
    Divfee,
    /// Dividend adjusted (foreign tax withheld).
    Divft,
    /// Dividend adjusted (NRA withheld).
    Divnra,
    /// Dividend return of capital.
    Divroc,
    /// Dividend adjusted (Tefra withheld).
    Divtw,
    /// Dividend (tax exempt).
    Divtxex,
    /// Dividend withholding.
    Divwh,
    /// DTC security settlement. Returned only; not accepted as a filter.
    Dtcs,
    /// DVP flip.
    Dvf,
    /// Event contract expiration.
    Ecexp,
    /// External trade.
    Extrd,
    /// Fee denominated in USD; `activity_sub_type` names the fee.
    Fee,
    /// Fixed income maturity.
    Fimat,
    /// Free of payment transfer.
    Fopt,
    /// FX conversion.
    Fxcnv,
    /// FX trade.
    Fxtrd,
    /// Interest (credit / margin).
    Int,
    /// Interest adjusted (NRA withheld).
    Intnra,
    /// Interest adjusted (Tefra withheld).
    Inttw,
    /// Inventory profit and loss.
    Invpnl,
    /// Journal entry.
    Jnl,
    /// Journal entry (cash).
    Jnlc,
    /// Journal entry (stock).
    Jnls,
    /// Merger / acquisition.
    Ma,
    /// Memopost.
    Mem,
    /// Filter only: a fixed group of less common types (such as `ACATC`, `INT`, `JNLC`). Never returned.
    Misc,
    /// Name change.
    Nc,
    /// On-chain transaction (blockchain deposit or withdrawal).
    Oct,
    /// Option assignment.
    Opasn,
    /// Option corporate action.
    Opca,
    /// Option cash deliverable for non-standard contracts.
    Opcsh,
    /// Option exercise.
    Opexc,
    /// Option expiration.
    Opexp,
    /// Option trade.
    Optrd,
    /// Pass-through charge.
    Ptc,
    /// Pass-through rebate.
    Ptr,
    /// Reorganization (sub-types `REOS`, `REOC`, `REOSC`).
    Reo,
    /// Worthless removal (sub-type `WRM`); broader reorganizations are `REO`.
    Reorg,
    /// Symbol change.
    Sc,
    /// Segregation. Returned only; not accepted as a filter.
    Seg,
    /// Stock spin-off.
    Spin,
    /// Stock split.
    Split,
    /// Former stock spin-off code; current spin-offs use `SPIN`. Returned only.
    Sso,
    /// Former stock split code; current splits use `SPLIT`. Returned only.
    Ssp,
    /// Subscription.
    Sub,
    /// Filter only: accepted, but omitted from results.
    Swp,
    /// Filter only: cash transactions (`CSD`, `CSR`, `CSW`). Never returned.
    Trans,
    /// Filter only. Never returned.
    Trd,
    /// Voluntary offering (tender, warrant, rights or exchange offers).
    Vof,
    /// Withholding.
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

/// `side` of a trade activity. Unlike an order's [`OrderSide`], it reports short sales as
/// `sell_short`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TradeActivitySide {
    Buy,
    Sell,
    SellShort,
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
    /// Trading API only; the Broker API accepts only `Dma`.
    Twap,
    /// Trading API only; the Broker API accepts only `Dma`.
    Vwap,
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

/// An order's role within its `bracket`, `oto` or `oco` group (see `Order::group_id`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OrderGroupRole {
    /// The entry order of a `bracket` or `oto` order.
    Entry,
    /// The take-profit limit order; for an `oco` order, the top-level order.
    TakeProfit,
    /// The stop-loss stop or stop-limit order.
    StopLoss,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
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
    Cronos,
    Hyperevm,
    Robinhood,
    Hypercore,
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
    Binance,
    Coinbase,
    Ondo,
    Bitget,
    Robinhood,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// Account activity `category` filter: mutually exclusive with `activity_types`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ActivityCategory {
    TradeActivity,
    NonTradeActivity,
}

/// Portfolio history `intraday_reporting`: which timestamps intraday data points cover.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum IntradayReporting {
    MarketHours,
    ExtendedHours,
    Continuous,
}

/// Portfolio history `pnl_reset`: the baseline for profit and loss.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PnlReset {
    NoReset,
    PerDay,
}

/// Asset `attributes`, also usable as a `GetAssetsRequest` filter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AssetAttribute {
    PtpNoException,
    PtpWithException,
    Ipo,
    HasOptions,
    OptionsLateClose,
    FractionalEhEnabled,
    OvernightTradable,
    OvernightHalted,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// What a calendar request's `start` and `end` refer to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[non_exhaustive]
pub enum CalendarDateType {
    Trading,
    Settlement,
}

/// Portfolio history `timeframe`: the resolution of each data point. Values other than these
/// are rejected by the API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PortfolioHistoryTimeframe {
    #[serde(rename = "1Min")]
    OneMin,
    #[serde(rename = "5Min")]
    FiveMin,
    #[serde(rename = "15Min")]
    FifteenMin,
    #[serde(rename = "1H")]
    OneHour,
    #[serde(rename = "1D")]
    OneDay,
}

/// How a Broker order's `commission` is interpreted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CommissionType {
    /// Per order. The default, except on `mleg` orders, which do not support it.
    Notional,
    /// Per share or contract, pro rated. The only type `mleg` orders support, and required on
    /// them whenever `commission` is non-zero.
    Qty,
    /// In basis points of the order's notional value (up to two decimal places). Not
    /// supported on `mleg` orders.
    Bps,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

/// State of the share transfer behind a tokenization request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TokenizationTransferStatus {
    /// The shares have not moved yet.
    Pending,
    /// The shares have moved.
    Completed,
    /// The share transfer failed.
    Rejected,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}
