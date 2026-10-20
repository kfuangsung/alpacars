use crate::common::client::{base_url, RestClient};
use crate::common::enums::Sort;
use crate::data::models::CorporateActionsSet;
use crate::error::AlpacaError;
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CorporateActionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<String>,
    /// Comma-separated CUSIPs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cusips: Option<String>,
    /// Comma-separated corporate action types, e.g. `cash_dividend,capital_gains_distribution`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// `us`, `non_us` or `all`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Comma-separated corporate action IDs. Mutually exclusive with every other filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// `complete` (the API default) excludes corporate actions that are still incomplete and
    /// unprocessed; `all` also returns those early records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Sort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// Async client for the Alpaca Corporate Actions API.
#[derive(Clone)]
pub struct CorporateActionsClient {
    client: RestClient,
}

impl CorporateActionsClient {
    pub fn new(api_key: Option<&str>, secret_key: Option<&str>) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                api_key.map(str::to_string),
                secret_key.map(str::to_string),
                None,
                base_url::DATA.to_string(),
                "v1".to_string(),
                false,
            )?,
        })
    }

    /// Create a client pointed at a custom base URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(
        api_key: &str,
        secret_key: &str,
        base_url: &str,
    ) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                base_url.to_string(),
                "v1".to_string(),
                false,
            )?,
        })
    }

    /// Fetch corporate actions (auto-paginated).
    pub async fn get_corporate_actions(
        &self,
        req: &CorporateActionsRequest,
    ) -> Result<CorporateActionsSet, AlpacaError> {
        let mut params = req.clone();
        let mut result = CorporateActionsSet {
            reverse_splits: None,
            forward_splits: None,
            unit_splits: None,
            cash_dividends: None,
            stock_dividends: None,
            spin_offs: None,
            cash_mergers: None,
            stock_mergers: None,
            stock_and_cash_mergers: None,
            redemptions: None,
            name_changes: None,
            worthless_removals: None,
            rights_distributions: None,
            capital_gains_distributions: None,
            partial_calls: None,
            reorganizations: None,
            next_page_token: None,
        };

        #[derive(serde::Deserialize)]
        struct Wrapper {
            corporate_actions: Option<CorporateActionsSet>,
            next_page_token: Option<String>,
        }

        loop {
            let resp: Wrapper = self.client.get("/corporate-actions", Some(&params)).await?;

            if let Some(data) = resp.corporate_actions {
                macro_rules! merge_vec {
                    ($field:ident) => {
                        if let Some(items) = data.$field {
                            result.$field.get_or_insert_with(Vec::new).extend(items);
                        }
                    };
                }

                merge_vec!(reverse_splits);
                merge_vec!(forward_splits);
                merge_vec!(unit_splits);
                merge_vec!(cash_dividends);
                merge_vec!(stock_dividends);
                merge_vec!(spin_offs);
                merge_vec!(cash_mergers);
                merge_vec!(stock_mergers);
                merge_vec!(stock_and_cash_mergers);
                merge_vec!(redemptions);
                merge_vec!(name_changes);
                merge_vec!(worthless_removals);
                merge_vec!(rights_distributions);
                merge_vec!(capital_gains_distributions);
                merge_vec!(partial_calls);
                merge_vec!(reorganizations);
            }

            match resp.next_page_token {
                Some(t) if !t.is_empty() => params.page_token = Some(t),
                _ => break,
            }
        }

        Ok(result)
    }
}
