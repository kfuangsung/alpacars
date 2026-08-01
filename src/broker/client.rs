use crate::broker::models::*;
use crate::broker::requests::*;
use crate::common::client::{base_url, RestClient};
use crate::error::AlpacaError;
use crate::trading::models::{
    Asset, Calendar, Clock, ClosePositionResponse, Order, Position, TradeAccount, Watchlist,
};
use uuid::Uuid;

/// Async client for the Alpaca Broker API.
///
/// Supports both sandbox and production environments.
/// All methods are async and return `Result<T, AlpacaError>`.
#[derive(Clone)]
pub struct BrokerClient {
    client: RestClient,
}

impl BrokerClient {
    /// Create a new BrokerClient.
    ///
    /// Set `sandbox = true` for the sandbox environment.
    pub fn new(api_key: &str, secret_key: &str, sandbox: bool) -> Result<Self, AlpacaError> {
        let url = if sandbox {
            base_url::BROKER_SANDBOX
        } else {
            base_url::BROKER_PRODUCTION
        };
        Ok(Self {
            client: RestClient::new(
                Some(api_key.to_string()),
                Some(secret_key.to_string()),
                None,
                url.to_string(),
                "v1".to_string(),
                true, // basic auth
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
                true, // basic auth
            )?,
        })
    }

    // ── Accounts ──────────────────────────────────────────────────────────────

    pub async fn create_account(
        &self,
        data: &CreateAccountRequest,
    ) -> Result<Account, AlpacaError> {
        self.client.post("/accounts", Some(data)).await
    }

    pub async fn get_account_by_id(&self, account_id: &Uuid) -> Result<Account, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}", account_id), None::<&()>)
            .await
    }

    pub async fn update_account(
        &self,
        account_id: &Uuid,
        data: &UpdateAccountRequest,
    ) -> Result<Account, AlpacaError> {
        self.client
            .patch(&format!("/accounts/{}", account_id), Some(data))
            .await
    }

    pub async fn delete_account(&self, account_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(&format!("/accounts/{}", account_id), None::<&()>)
            .await
    }

    pub async fn close_account(&self, account_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .post_void(&format!("/accounts/{}:close", account_id), None::<&()>)
            .await
    }

    pub async fn list_accounts(
        &self,
        filter: Option<&ListAccountsRequest>,
    ) -> Result<Vec<Account>, AlpacaError> {
        self.client.get("/accounts", filter).await
    }

    pub async fn get_trade_account_by_id(
        &self,
        account_id: &Uuid,
        trading_account_id: &Uuid,
    ) -> Result<TradeAccount, AlpacaError> {
        self.client
            .get(
                &format!(
                    "/accounts/{}/trading_accounts/{}",
                    account_id, trading_account_id
                ),
                None::<&()>,
            )
            .await
    }

    // ── Documents ─────────────────────────────────────────────────────────────

    pub async fn upload_documents_to_account(
        &self,
        account_id: &Uuid,
        documents: &[UploadDocumentRequest],
    ) -> Result<(), AlpacaError> {
        self.client
            .post_void(
                &format!("/accounts/{}/documents/upload", account_id),
                Some(documents),
            )
            .await
    }

    pub async fn get_trade_documents_for_account(
        &self,
        account_id: &Uuid,
        filter: Option<&GetTradeDocumentsRequest>,
    ) -> Result<Vec<TradeDocument>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/documents", account_id), filter)
            .await
    }

    pub async fn get_trade_document_for_account_by_id(
        &self,
        account_id: &Uuid,
        document_id: &Uuid,
    ) -> Result<TradeDocument, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/documents/{}", account_id, document_id),
                None::<&()>,
            )
            .await
    }

    // ── CIP Data ──────────────────────────────────────────────────────────────

    pub async fn get_cip_data_for_account_by_id(
        &self,
        account_id: &Uuid,
    ) -> Result<CIPInfo, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/cip_data", account_id), None::<&()>)
            .await
    }

    pub async fn upload_cip_data_for_account_by_id(
        &self,
        account_id: &Uuid,
        data: &serde_json::Value,
    ) -> Result<CIPInfo, AlpacaError> {
        self.client
            .post(&format!("/accounts/{}/cip_data", account_id), Some(data))
            .await
    }

    // ── Activities ────────────────────────────────────────────────────────────

    pub async fn get_account_activities(
        &self,
        account_id: &Uuid,
        filter: Option<&GetAccountActivitiesRequest>,
    ) -> Result<Vec<serde_json::Value>, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/account_activities", account_id),
                filter,
            )
            .await
    }

    // ── ACH Relationships ─────────────────────────────────────────────────────

    pub async fn create_ach_relationship_for_account(
        &self,
        account_id: &Uuid,
        data: &CreateACHRelationshipRequest,
    ) -> Result<ACHRelationship, AlpacaError> {
        self.client
            .post(
                &format!("/accounts/{}/ach_relationships", account_id),
                Some(data),
            )
            .await
    }

    pub async fn get_ach_relationships_for_account(
        &self,
        account_id: &Uuid,
    ) -> Result<Vec<ACHRelationship>, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/ach_relationships", account_id),
                None::<&()>,
            )
            .await
    }

    pub async fn delete_ach_relationship_for_account(
        &self,
        account_id: &Uuid,
        relationship_id: &Uuid,
    ) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!(
                    "/accounts/{}/ach_relationships/{}",
                    account_id, relationship_id
                ),
                None::<&()>,
            )
            .await
    }

    // ── Banks ─────────────────────────────────────────────────────────────────

    pub async fn create_bank_for_account(
        &self,
        account_id: &Uuid,
        data: &CreateBankRequest,
    ) -> Result<Bank, AlpacaError> {
        self.client
            .post(&format!("/accounts/{}/banks", account_id), Some(data))
            .await
    }

    pub async fn get_banks_for_account(&self, account_id: &Uuid) -> Result<Vec<Bank>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/banks", account_id), None::<&()>)
            .await
    }

    pub async fn delete_bank_for_account(
        &self,
        account_id: &Uuid,
        bank_id: &Uuid,
    ) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!("/accounts/{}/banks/{}", account_id, bank_id),
                None::<&()>,
            )
            .await
    }

    // ── Transfers ─────────────────────────────────────────────────────────────

    pub async fn create_transfer_for_account(
        &self,
        account_id: &Uuid,
        data: &CreateTransferRequest,
    ) -> Result<Transfer, AlpacaError> {
        self.client
            .post(&format!("/accounts/{}/transfers", account_id), Some(data))
            .await
    }

    pub async fn get_transfers_for_account(
        &self,
        account_id: &Uuid,
        filter: Option<&GetTransfersRequest>,
    ) -> Result<Vec<Transfer>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/transfers", account_id), filter)
            .await
    }

    pub async fn cancel_transfer_for_account(
        &self,
        account_id: &Uuid,
        transfer_id: &Uuid,
    ) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!("/accounts/{}/transfers/{}", account_id, transfer_id),
                None::<&()>,
            )
            .await
    }

    // ── Positions (per account) ───────────────────────────────────────────────

    pub async fn get_all_positions_for_account(
        &self,
        account_id: &Uuid,
    ) -> Result<Vec<Position>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/positions", account_id), None::<&()>)
            .await
    }

    pub async fn get_open_position_for_account(
        &self,
        account_id: &Uuid,
        symbol_or_asset_id: &str,
    ) -> Result<Position, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/positions/{}", account_id, symbol_or_asset_id),
                None::<&()>,
            )
            .await
    }

    pub async fn close_all_positions_for_account(
        &self,
        account_id: &Uuid,
        cancel_orders: Option<bool>,
    ) -> Result<Vec<ClosePositionResponse>, AlpacaError> {
        #[derive(serde::Serialize)]
        struct Params {
            #[serde(skip_serializing_if = "Option::is_none")]
            cancel_orders: Option<bool>,
        }
        self.client
            .delete(
                &format!("/accounts/{}/positions", account_id),
                Some(&Params { cancel_orders }),
            )
            .await
    }

    pub async fn close_position_for_account(
        &self,
        account_id: &Uuid,
        symbol_or_asset_id: &str,
        close_options: Option<&ClosePositionRequest>,
    ) -> Result<Order, AlpacaError> {
        self.client
            .delete_with_body(
                &format!("/accounts/{}/positions/{}", account_id, symbol_or_asset_id),
                close_options,
            )
            .await
    }

    // ── Orders (per account) ──────────────────────────────────────────────────

    pub async fn submit_order_for_account(
        &self,
        account_id: &Uuid,
        order: &OrderRequest,
    ) -> Result<Order, AlpacaError> {
        self.client
            .post(&format!("/accounts/{}/orders", account_id), Some(order))
            .await
    }

    pub async fn get_orders_for_account(
        &self,
        account_id: &Uuid,
        filter: Option<&GetOrdersForAccountRequest>,
    ) -> Result<Vec<Order>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/orders", account_id), filter)
            .await
    }

    pub async fn get_order_for_account_by_id(
        &self,
        account_id: &Uuid,
        order_id: &Uuid,
    ) -> Result<Order, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/orders/{}", account_id, order_id),
                None::<&()>,
            )
            .await
    }

    pub async fn replace_order_for_account_by_id(
        &self,
        account_id: &Uuid,
        order_id: &Uuid,
        data: &ReplaceOrderRequest,
    ) -> Result<Order, AlpacaError> {
        self.client
            .patch(
                &format!("/accounts/{}/orders/{}", account_id, order_id),
                Some(data),
            )
            .await
    }

    pub async fn cancel_orders_for_account(
        &self,
        account_id: &Uuid,
    ) -> Result<Vec<serde_json::Value>, AlpacaError> {
        self.client
            .delete(&format!("/accounts/{}/orders", account_id), None::<&()>)
            .await
    }

    pub async fn cancel_order_for_account_by_id(
        &self,
        account_id: &Uuid,
        order_id: &Uuid,
    ) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!("/accounts/{}/orders/{}", account_id, order_id),
                None::<&()>,
            )
            .await
    }

    // ── Journals ──────────────────────────────────────────────────────────────

    pub async fn create_journal(
        &self,
        data: &CreateJournalRequest,
    ) -> Result<Journal, AlpacaError> {
        self.client.post("/journals", Some(data)).await
    }

    pub async fn create_batch_journal(
        &self,
        data: &CreateBatchJournalRequest,
    ) -> Result<Vec<Journal>, AlpacaError> {
        self.client.post("/journals:batch", Some(data)).await
    }

    pub async fn create_reverse_batch_journal(
        &self,
        data: &CreateReverseBatchJournalRequest,
    ) -> Result<Vec<Journal>, AlpacaError> {
        self.client
            .post("/journals:batch_reverse", Some(data))
            .await
    }

    pub async fn get_journals(
        &self,
        filter: Option<&GetJournalsRequest>,
    ) -> Result<Vec<Journal>, AlpacaError> {
        self.client.get("/journals", filter).await
    }

    pub async fn get_journal_by_id(&self, journal_id: &Uuid) -> Result<Journal, AlpacaError> {
        self.client
            .get(&format!("/journals/{}", journal_id), None::<&()>)
            .await
    }

    pub async fn cancel_journal_by_id(&self, journal_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(&format!("/journals/{}", journal_id), None::<&()>)
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

    // ── Watchlists (per account) ──────────────────────────────────────────────

    pub async fn get_watchlists_for_account(
        &self,
        account_id: &Uuid,
    ) -> Result<Vec<Watchlist>, AlpacaError> {
        self.client
            .get(&format!("/accounts/{}/watchlists", account_id), None::<&()>)
            .await
    }

    pub async fn get_watchlist_for_account_by_id(
        &self,
        account_id: &Uuid,
        watchlist_id: &Uuid,
    ) -> Result<Watchlist, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/watchlists/{}", account_id, watchlist_id),
                None::<&()>,
            )
            .await
    }

    pub async fn create_watchlist_for_account(
        &self,
        account_id: &Uuid,
        data: &CreateWatchlistRequest,
    ) -> Result<Watchlist, AlpacaError> {
        self.client
            .post(&format!("/accounts/{}/watchlists", account_id), Some(data))
            .await
    }

    pub async fn update_watchlist_for_account_by_id(
        &self,
        account_id: &Uuid,
        watchlist_id: &Uuid,
        data: &UpdateWatchlistRequest,
    ) -> Result<Watchlist, AlpacaError> {
        self.client
            .put(
                &format!("/accounts/{}/watchlists/{}", account_id, watchlist_id),
                Some(data),
            )
            .await
    }

    pub async fn delete_watchlist_from_account_by_id(
        &self,
        account_id: &Uuid,
        watchlist_id: &Uuid,
    ) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!("/accounts/{}/watchlists/{}", account_id, watchlist_id),
                None::<&()>,
            )
            .await
    }

    // ── Portfolios (Rebalancing) ──────────────────────────────────────────────

    pub async fn create_portfolio(
        &self,
        data: &CreatePortfolioRequest,
    ) -> Result<Portfolio, AlpacaError> {
        self.client
            .post("/rebalancing/portfolios", Some(data))
            .await
    }

    pub async fn get_all_portfolios(
        &self,
        filter: Option<&GetPortfoliosRequest>,
    ) -> Result<Vec<Portfolio>, AlpacaError> {
        self.client.get("/rebalancing/portfolios", filter).await
    }

    pub async fn get_portfolio_by_id(&self, portfolio_id: &Uuid) -> Result<Portfolio, AlpacaError> {
        self.client
            .get(
                &format!("/rebalancing/portfolios/{}", portfolio_id),
                None::<&()>,
            )
            .await
    }

    pub async fn update_portfolio_by_id(
        &self,
        portfolio_id: &Uuid,
        data: &serde_json::Value,
    ) -> Result<Portfolio, AlpacaError> {
        self.client
            .patch(
                &format!("/rebalancing/portfolios/{}", portfolio_id),
                Some(data),
            )
            .await
    }

    pub async fn inactivate_portfolio_by_id(&self, portfolio_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .post_void(
                &format!("/rebalancing/portfolios/{}:inactivate", portfolio_id),
                None::<&()>,
            )
            .await
    }

    // ── Subscriptions ─────────────────────────────────────────────────────────

    pub async fn create_subscription(
        &self,
        data: &CreateSubscriptionRequest,
    ) -> Result<Subscription, AlpacaError> {
        self.client
            .post("/rebalancing/subscriptions", Some(data))
            .await
    }

    pub async fn get_all_subscriptions(
        &self,
        filter: Option<&GetSubscriptionsRequest>,
    ) -> Result<Vec<Subscription>, AlpacaError> {
        self.client.get("/rebalancing/subscriptions", filter).await
    }

    pub async fn get_subscription_by_id(
        &self,
        subscription_id: &Uuid,
    ) -> Result<Subscription, AlpacaError> {
        self.client
            .get(
                &format!("/rebalancing/subscriptions/{}", subscription_id),
                None::<&()>,
            )
            .await
    }

    pub async fn unsubscribe_account(&self, subscription_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(
                &format!("/rebalancing/subscriptions/{}", subscription_id),
                None::<&()>,
            )
            .await
    }

    // ── Rebalancing Runs ──────────────────────────────────────────────────────

    pub async fn create_manual_run(
        &self,
        data: &CreateRunRequest,
    ) -> Result<RebalancingRun, AlpacaError> {
        self.client.post("/rebalancing/runs", Some(data)).await
    }

    pub async fn get_all_runs(
        &self,
        filter: Option<&GetRunsRequest>,
    ) -> Result<Vec<RebalancingRun>, AlpacaError> {
        self.client.get("/rebalancing/runs", filter).await
    }

    pub async fn get_run_by_id(&self, run_id: &Uuid) -> Result<RebalancingRun, AlpacaError> {
        self.client
            .get(&format!("/rebalancing/runs/{}", run_id), None::<&()>)
            .await
    }

    pub async fn cancel_run_by_id(&self, run_id: &Uuid) -> Result<(), AlpacaError> {
        self.client
            .delete_void(&format!("/rebalancing/runs/{}", run_id), None::<&()>)
            .await
    }

    // ── Options ───────────────────────────────────────────────────────────────

    pub async fn exercise_options_position_for_account_by_id(
        &self,
        account_id: &Uuid,
        symbol_or_contract_id: &str,
    ) -> Result<(), AlpacaError> {
        self.client
            .post_void(
                &format!(
                    "/accounts/{}/positions/{}/exercise",
                    account_id, symbol_or_contract_id
                ),
                None::<&()>,
            )
            .await
    }

    // ── Tokenization ──────────────────────────────────────────────────────────

    /// Convert an account's underlying equity securities into a tokenized asset.
    pub async fn mint_tokenized_asset_for_account(
        &self,
        account_id: &Uuid,
        data: &TokenizationMintRequest,
    ) -> Result<TokenizationMintResponse, AlpacaError> {
        self.client
            .post(
                &format!("/accounts/{}/tokenization/mint", account_id),
                Some(data),
            )
            .await
    }

    pub async fn get_tokenization_requests_for_account(
        &self,
        account_id: &Uuid,
        filter: Option<&GetTokenizationRequestsRequest>,
    ) -> Result<Vec<TokenizationRequest>, AlpacaError> {
        self.client
            .get(
                &format!("/accounts/{}/tokenization/requests", account_id),
                filter,
            )
            .await
    }

    pub async fn get_tokenization_request_for_account(
        &self,
        account_id: &Uuid,
        tokenization_request_id: &str,
    ) -> Result<TokenizationRequest, AlpacaError> {
        self.client
            .get(
                &format!(
                    "/accounts/{}/tokenization/requests/{}",
                    account_id, tokenization_request_id
                ),
                None::<&()>,
            )
            .await
    }

    /// Look up a tokenization request by the `client_request_id` supplied at mint time.
    ///
    /// When several requests share a `client_request_id`, the most recently created wins.
    pub async fn get_tokenization_request_for_account_by_client_request_id(
        &self,
        account_id: &Uuid,
        client_request_id: &str,
    ) -> Result<TokenizationRequest, AlpacaError> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            client_request_id: &'a str,
        }
        self.client
            .get(
                &format!(
                    "/accounts/{}/tokenization/requests:by_client_request_id",
                    account_id
                ),
                Some(&Params { client_request_id }),
            )
            .await
    }

    /// Look up a tokenization request by the issuer-assigned `issuer_request_id`.
    ///
    /// Broker API only; the Trading API has no equivalent.
    pub async fn get_tokenization_request_for_account_by_issuer_request_id(
        &self,
        account_id: &Uuid,
        issuer_request_id: &str,
    ) -> Result<TokenizationRequest, AlpacaError> {
        #[derive(serde::Serialize)]
        struct Params<'a> {
            issuer_request_id: &'a str,
        }
        self.client
            .get(
                &format!(
                    "/accounts/{}/tokenization/requests:by_issuer_request_id",
                    account_id
                ),
                Some(&Params { issuer_request_id }),
            )
            .await
    }
}
