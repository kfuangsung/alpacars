use crate::common::client::{base_url, RestClient};
use crate::error::AlpacaError;
use crate::trading::models::*;
use crate::trading::requests::*;
use uuid::Uuid;

/// Async client for the Alpaca Trading API.
///
/// Supports both paper and live trading environments.
/// All methods are async and return `Result<T, AlpacaError>`.
#[derive(Clone)]
pub struct TradingClient {
    client: RestClient,
}

impl TradingClient {
    /// Create a new TradingClient with API key authentication.
    ///
    /// Set `paper = true` for paper trading, `false` for live trading.
    pub fn new(api_key: &str, secret_key: &str, paper: bool) -> Result<Self, AlpacaError> {
        let url = if paper {
            base_url::TRADING_PAPER
        } else {
            base_url::TRADING_LIVE
        };
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                url.to_string(),
                "v2".to_string(),
                false,
            )?,
        })
    }

    /// Create a client pointed at a custom base URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(api_key: &str, secret_key: &str, base_url: &str) -> Result<Self, AlpacaError> {
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                base_url.to_string(),
                "v2".to_string(),
                false,
            )?,
        })
    }

    /// Create a new TradingClient with OAuth token authentication.
    pub fn with_oauth(oauth_token: &str, paper: bool) -> Result<Self, AlpacaError> {
        let url = if paper {
            base_url::TRADING_PAPER
        } else {
            base_url::TRADING_LIVE
        };
        Ok(Self {
            client: RestClient::new(
                None,
                None,
                Some(oauth_token.to_string()),
                url.to_string(),
                "v2".to_string(),
                false,
            )?,
        })
    }

    // ── Account ──────────────────────────────────────────────────────────────

    pub async fn get_account(&self) -> Result<TradeAccount, AlpacaError> {
        self.client.get("/account", None::<&()>).await
    }

    pub async fn get_account_configurations(&self) -> Result<AccountConfiguration, AlpacaError> {
        self.client.get("/account/configurations", None::<&()>).await
    }

    pub async fn set_account_configurations(
        &self,
        config: &AccountConfiguration,
    ) -> Result<AccountConfiguration, AlpacaError> {
        self.client.patch("/account/configurations", Some(config)).await
    }

    pub async fn get_portfolio_history(
        &self,
        filter: Option<&GetPortfolioHistoryRequest>,
    ) -> Result<PortfolioHistory, AlpacaError> {
        self.client.get("/account/portfolio/history", filter).await
    }

    pub async fn get_account_activities(
        &self,
        filter: Option<&GetAccountActivitiesRequest>,
    ) -> Result<Vec<serde_json::Value>, AlpacaError> {
        self.client.get("/account/activities", filter).await
    }

    // ── Orders ────────────────────────────────────────────────────────────────

    pub async fn submit_order(&self, order: &OrderRequest) -> Result<Order, AlpacaError> {
        self.client.post("/orders", Some(order)).await
    }

    pub async fn get_orders(
        &self,
        filter: Option<&GetOrdersRequest>,
    ) -> Result<Vec<Order>, AlpacaError> {
        self.client.get("/orders", filter).await
    }

    pub async fn get_order_by_id(
        &self,
        order_id: &Uuid,
        filter: Option<&GetOrderByIdRequest>,
    ) -> Result<Order, AlpacaError> {
        self.client
            .get(&format!("/orders/{}", order_id), filter)
            .await
    }

    pub async fn get_order_by_client_id(&self, client_id: &str) -> Result<Order, AlpacaError> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            client_order_id: &'a str,
        }
        self.client
            .get("/orders:by_client_order_id", Some(&Params { client_order_id: client_id }))
            .await
    }

    pub async fn replace_order_by_id(
        &self,
        order_id: &Uuid,
        data: &ReplaceOrderRequest,
    ) -> Result<Order, AlpacaError> {
        self.client
            .patch(&format!("/orders/{}", order_id), Some(data))
            .await
    }

    pub async fn cancel_orders(&self) -> Result<Vec<CancelOrderResponse>, AlpacaError> {
        self.client.delete("/orders", None::<&()>).await
    }

    pub async fn cancel_order_by_id(&self, order_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(&format!("/orders/{}", order_id), None::<&()>)
            .await
    }

    // ── Positions ─────────────────────────────────────────────────────────────

    pub async fn get_all_positions(&self) -> Result<Vec<Position>, AlpacaError> {
        self.client.get("/positions", None::<&()>).await
    }

    pub async fn get_open_position(
        &self,
        symbol_or_asset_id: &str,
    ) -> Result<Position, AlpacaError> {
        self.client
            .get(&format!("/positions/{}", symbol_or_asset_id), None::<&()>)
            .await
    }

    pub async fn close_all_positions(
        &self,
        cancel_orders: Option<bool>,
    ) -> Result<Vec<ClosePositionResponse>, AlpacaError> {
        #[derive(serde::Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            cancel_orders: Option<bool>,
        }
        self.client
            .delete("/positions", Some(&Params { cancel_orders }))
            .await
    }

    pub async fn close_position(
        &self,
        symbol_or_asset_id: &str,
        close_options: Option<&ClosePositionRequest>,
    ) -> Result<Order, AlpacaError> {
        self.client
            .delete_with_body(&format!("/positions/{}", symbol_or_asset_id), close_options)
            .await
    }

    pub async fn exercise_options_position(
        &self,
        symbol_or_contract_id: &str,
    ) -> Result<(), AlpacaError> {
        self.client
            .post_void(
                &format!("/positions/{}/exercise", symbol_or_contract_id),
                None::<&()>,
            )
            .await
    }

    // ── Assets ────────────────────────────────────────────────────────────────

    pub async fn get_all_assets(
        &self,
        filter: Option<&GetAssetsRequest>,
    ) -> Result<Vec<Asset>, AlpacaError> {
        self.client.get("/assets", filter).await
    }

    pub async fn get_asset(&self, symbol_or_asset_id: &str) -> Result<Asset, AlpacaError> {
        self.client
            .get(&format!("/assets/{}", symbol_or_asset_id), None::<&()>)
            .await
    }

    // ── Clock & Calendar ──────────────────────────────────────────────────────

    pub async fn get_clock(&self) -> Result<Clock, AlpacaError> {
        self.client.get("/clock", None::<&()>).await
    }

    pub async fn get_calendar(
        &self,
        filters: Option<&GetCalendarRequest>,
    ) -> Result<Vec<Calendar>, AlpacaError> {
        self.client.get("/calendar", filters).await
    }

    // ── Watchlists ────────────────────────────────────────────────────────────

    pub async fn get_watchlists(&self) -> Result<Vec<Watchlist>, AlpacaError> {
        self.client.get("/watchlists", None::<&()>).await
    }

    pub async fn get_watchlist_by_id(&self, watchlist_id: &Uuid) -> Result<Watchlist, AlpacaError> {
        self.client
            .get(&format!("/watchlists/{}", watchlist_id), None::<&()>)
            .await
    }

    pub async fn create_watchlist(
        &self,
        data: &CreateWatchlistRequest,
    ) -> Result<Watchlist, AlpacaError> {
        self.client.post("/watchlists", Some(data)).await
    }

    pub async fn update_watchlist_by_id(
        &self,
        watchlist_id: &Uuid,
        data: &UpdateWatchlistRequest,
    ) -> Result<Watchlist, AlpacaError> {
        self.client
            .put(&format!("/watchlists/{}", watchlist_id), Some(data))
            .await
    }

    pub async fn add_asset_to_watchlist_by_id(
        &self,
        watchlist_id: &Uuid,
        symbol: &str,
    ) -> Result<Watchlist, AlpacaError> {
        let body = AddSymbolToWatchlistRequest { symbol: symbol.to_string() };
        self.client
            .post(&format!("/watchlists/{}", watchlist_id), Some(&body))
            .await
    }

    pub async fn delete_watchlist_by_id(&self, watchlist_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(&format!("/watchlists/{}", watchlist_id), None::<&()>)
            .await
    }

    pub async fn remove_asset_from_watchlist_by_id(
        &self,
        watchlist_id: &Uuid,
        symbol: &str,
    ) -> Result<Watchlist, AlpacaError> {
        self.client
            .delete(&format!("/watchlists/{}/{}", watchlist_id, symbol), None::<&()>)
            .await
    }

    // ── Corporate Actions (deprecated) ────────────────────────────────────────

    pub async fn get_corporate_announcements(
        &self,
        filter: &GetCorporateAnnouncementsRequest,
    ) -> Result<Vec<CorporateActionAnnouncement>, AlpacaError> {
        self.client
            .get("/corporate_actions/announcements", Some(filter))
            .await
    }

    pub async fn get_corporate_announcement_by_id(
        &self,
        id: &str,
    ) -> Result<CorporateActionAnnouncement, AlpacaError> {
        self.client
            .get(&format!("/corporate_actions/announcements/{}", id), None::<&()>)
            .await
    }

    // ── Options ───────────────────────────────────────────────────────────────

    pub async fn get_option_contracts(
        &self,
        request: &GetOptionContractsRequest,
    ) -> Result<OptionContractsResponse, AlpacaError> {
        self.client.get("/options/contracts", Some(request)).await
    }

    pub async fn get_option_contract(
        &self,
        symbol_or_id: &str,
    ) -> Result<OptionContract, AlpacaError> {
        self.client
            .get(&format!("/options/contracts/{}", symbol_or_id), None::<&()>)
            .await
    }

    // ── Locates (hard-to-borrow) ──────────────────────────────────────────────
    // Served under /v1, unlike the rest of the Trading API.

    pub async fn create_locate(&self, request: &CreateLocateRequest) -> Result<Locate, AlpacaError> {
        self.client
            .with_api_version("v1")
            .post("/locates", Some(request))
            .await
    }

    pub async fn get_locates(
        &self,
        request: Option<&GetLocatesRequest>,
    ) -> Result<ListLocatesResponse, AlpacaError> {
        self.client
            .with_api_version("v1")
            .get("/locates", request)
            .await
    }

    pub async fn get_locate(&self, locate_id: &str) -> Result<Locate, AlpacaError> {
        self.client
            .with_api_version("v1")
            .get(&format!("/locates/{}", locate_id), None::<&()>)
            .await
    }

    pub async fn get_locate_quotes(
        &self,
        request: &GetLocateQuotesRequest,
    ) -> Result<ListLocateQuotesResponse, AlpacaError> {
        self.client
            .with_api_version("v1")
            .get("/locates/quotes", Some(request))
            .await
    }
}
