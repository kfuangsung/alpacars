use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaginationType {
    None,
    Full,
    Iterator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SupportedCurrencies {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CNH")]
    Cnh,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "NZD")]
    Nzd,
}
