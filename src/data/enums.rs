use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Exchange {
    Z,
    I,
    M,
    U,
    L,
    W,
    X,
    B,
    D,
    J,
    P,
    Q,
    S,
    V,
    A,
    E,
    N,
    T,
    Y,
    C,
    H,
    K,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DataFeed {
    Iex,
    Sip,
    #[serde(rename = "delayed_sip")]
    DelayedSip,
    Otc,
    Boats,
    Overnight,
    /// A value this version of the crate does not recognize yet. Alpaca adds values over
    /// time; this keeps such responses parseable. Not meaningful to send in a request.
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Adjustment {
    Raw,
    Split,
    Dividend,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum CryptoFeed {
    Us,
    /// Location values from the crypto market data reference (`crypto_historical_loc`).
    #[serde(rename = "us-1")]
    Us1,
    #[serde(rename = "us-2")]
    Us2,
    #[serde(rename = "eu-1")]
    Eu1,
    #[serde(rename = "bs-1")]
    Bs1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum OptionsFeed {
    Opra,
    Indicative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum MostActivesBy {
    Volume,
    Trades,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum MarketType {
    Stocks,
    Crypto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum NewsImageSize {
    Thumb,
    Small,
    Large,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CorporateActionsType {
    ReverseSplit,
    ForwardSplit,
    UnitSplit,
    CashDividend,
    StockDividend,
    SpinOff,
    CashMerger,
    StockMerger,
    StockAndCashMerger,
    Redemption,
    NameChange,
    WorthlessRemoval,
    RightsDistribution,
    CapitalGainsDistribution,
    PartialCall,
    Reorganization,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TimeFrameUnit {
    #[serde(rename = "Min")]
    Minute,
    #[serde(rename = "Hour")]
    Hour,
    #[serde(rename = "Day")]
    Day,
    #[serde(rename = "Week")]
    Week,
    #[serde(rename = "Month")]
    Month,
}

impl TimeFrameUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFrameUnit::Minute => "Min",
            TimeFrameUnit::Hour => "Hour",
            TimeFrameUnit::Day => "Day",
            TimeFrameUnit::Week => "Week",
            TimeFrameUnit::Month => "Month",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeFrame {
    pub amount: u32,
    pub unit: TimeFrameUnit,
}

impl TimeFrame {
    pub fn new(amount: u32, unit: TimeFrameUnit) -> Self {
        Self { amount, unit }
    }

    pub fn minute() -> Self {
        Self::new(1, TimeFrameUnit::Minute)
    }
    pub fn hour() -> Self {
        Self::new(1, TimeFrameUnit::Hour)
    }
    pub fn day() -> Self {
        Self::new(1, TimeFrameUnit::Day)
    }
    pub fn week() -> Self {
        Self::new(1, TimeFrameUnit::Week)
    }
    pub fn month() -> Self {
        Self::new(1, TimeFrameUnit::Month)
    }

    pub fn value(&self) -> String {
        format!("{}{}", self.amount, self.unit.as_str())
    }
}

impl serde::Serialize for TimeFrame {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.value())
    }
}

impl std::fmt::Display for TimeFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
