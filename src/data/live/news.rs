use crate::common::client::base_url;
use crate::data::live::websocket::{DataStreamConnection, RawStreamEvent, SubscribeMsg};
use crate::data::models::News;
use crate::error::AlpacaError;
use std::sync::Arc;
use tracing::warn;

pub type NewsHandler = Arc<dyn Fn(News) + Send + Sync + 'static>;

/// Real-time WebSocket stream for news events.
pub struct NewsDataStream {
    api_key: String,
    secret_key: String,
    url_override: Option<String>,
    symbols: Vec<String>,
    news_handler: Option<NewsHandler>,
}

impl NewsDataStream {
    pub fn new(api_key: &str, secret_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            secret_key: secret_key.to_string(),
            url_override: None,
            symbols: Vec::new(),
            news_handler: None,
        }
    }

    /// Create a stream pointed at a custom WebSocket URL (for testing / mocking).
    #[doc(hidden)]
    pub fn new_with_url(api_key: &str, secret_key: &str, url: &str) -> Self {
        Self {
            url_override: Some(url.to_string()),
            ..Self::new(api_key, secret_key)
        }
    }

    /// Subscribe to news for the given symbols. Use `["*"]` for all symbols.
    pub fn subscribe_news<F>(
        &mut self,
        symbols: impl IntoIterator<Item = impl Into<String>>,
        handler: F,
    ) where
        F: Fn(News) + Send + Sync + 'static,
    {
        self.symbols.extend(symbols.into_iter().map(Into::into));
        self.news_handler = Some(Arc::new(handler));
    }

    pub async fn run(&self) -> Result<(), AlpacaError> {
        let sub = SubscribeMsg::subscribe(
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            self.symbols.clone(),
        );

        let url = self
            .url_override
            .clone()
            .unwrap_or_else(|| format!("{}/v1beta1/news", base_url::MARKET_DATA_STREAM));
        let conn =
            DataStreamConnection::new(url, self.api_key.clone(), self.secret_key.clone(), sub);

        let handler = self.news_handler.clone();

        conn.run(move |event: RawStreamEvent| {
            if event.msg_type.as_deref() != Some("n") {
                return;
            }
            let raw = event.into_payload();
            match serde_json::from_value::<News>(raw) {
                Ok(v) => {
                    if let Some(h) = &handler {
                        h(v);
                    }
                }
                Err(e) => warn!(error = %e, "failed to deserialize News event"),
            }
        })
        .await
    }
}
