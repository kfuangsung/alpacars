use crate::error::AlpacaError;
use reqwest::{header, Client, Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
pub const DEFAULT_RETRY_WAIT_SECS: u64 = 3;
pub const DEFAULT_RETRY_STATUS_CODES: &[u16] = &[429, 504];

pub const DATA_V2_MAX_LIMIT: u32 = 10_000;
pub const ACCOUNT_ACTIVITIES_DEFAULT_PAGE_SIZE: u32 = 100;

pub mod base_url {
    pub const TRADING_PAPER: &str = "https://paper-api.alpaca.markets";
    pub const TRADING_LIVE: &str = "https://api.alpaca.markets";
    pub const BROKER_SANDBOX: &str = "https://broker-api.sandbox.alpaca.markets";
    pub const BROKER_PRODUCTION: &str = "https://broker-api.alpaca.markets";
    pub const DATA: &str = "https://data.alpaca.markets";
    pub const DATA_SANDBOX: &str = "https://data.sandbox.alpaca.markets";
    pub const TRADING_STREAM_PAPER: &str = "wss://paper-api.alpaca.markets/stream";
    pub const TRADING_STREAM_LIVE: &str = "wss://api.alpaca.markets/stream";
    pub const MARKET_DATA_STREAM: &str = "wss://stream.data.alpaca.markets";
}

#[derive(Clone)]
pub struct RestClient {
    http: Client,
    api_key: Option<String>,
    secret_key: Option<String>,
    oauth_token: Option<String>,
    pub(crate) base_url: String,
    pub(crate) api_version: String,
    use_basic_auth: bool,
    retry_attempts: u32,
    retry_wait_secs: u64,
    retry_status_codes: Vec<u16>,
}

impl RestClient {
    pub fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        oauth_token: Option<String>,
        base_url: String,
        api_version: String,
        use_basic_auth: bool,
    ) -> Result<Self, AlpacaError> {
        if oauth_token.is_none() {
            match (&api_key, &secret_key) {
                (None, _) => {
                    return Err(AlpacaError::InvalidCredentials(
                        "Either oauth_token or api_key+secret_key must be provided".into(),
                    ))
                }
                (Some(_), None) => {
                    return Err(AlpacaError::InvalidCredentials(
                        "secret_key is required when api_key is provided".into(),
                    ))
                }
                _ => {}
            }
        }
        Ok(Self {
            http: Client::new(),
            api_key,
            secret_key,
            oauth_token,
            base_url,
            api_version,
            use_basic_auth,
            retry_attempts: DEFAULT_RETRY_ATTEMPTS,
            retry_wait_secs: DEFAULT_RETRY_WAIT_SECS,
            retry_status_codes: DEFAULT_RETRY_STATUS_CODES.to_vec(),
        })
    }

    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    pub fn secret_key(&self) -> Option<&str> {
        self.secret_key.as_deref()
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Clone this client with a different API version prefix (e.g. "v1"
    /// for endpoints not served under the client's default version).
    pub(crate) fn with_api_version(&self, api_version: &str) -> RestClient {
        let mut client = self.clone();
        client.api_version = api_version.to_string();
        client
    }

    pub fn build_url(&self, path: &str) -> String {
        format!("{}/{}{}", self.base_url, self.api_version, path)
    }

    fn auth_headers(&self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        if let Some(token) = &self.oauth_token {
            if let Ok(v) = format!("Bearer {}", token).parse() {
                headers.insert(header::AUTHORIZATION, v);
            }
        } else if self.use_basic_auth {
            if let (Some(key), Some(secret)) = (&self.api_key, &self.secret_key) {
                use base64::Engine;
                let encoded =
                    base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", key, secret));
                if let Ok(v) = format!("Basic {}", encoded).parse() {
                    headers.insert(header::AUTHORIZATION, v);
                }
            }
        } else {
            if let Some(key) = &self.api_key {
                if let Ok(v) = key.parse() {
                    headers.insert("APCA-API-KEY-ID", v);
                }
            }
            if let Some(secret) = &self.secret_key {
                if let Ok(v) = secret.parse() {
                    headers.insert("APCA-API-SECRET-KEY", v);
                }
            }
        }
        headers
    }

    fn query_pairs(val: &serde_json::Value) -> Vec<(String, String)> {
        let obj = match val.as_object() {
            Some(o) => o,
            None => return vec![],
        };
        obj.iter()
            .filter_map(|(k, v)| match v {
                serde_json::Value::Null => None,
                serde_json::Value::Bool(b) => Some((k.clone(), b.to_string())),
                serde_json::Value::Number(n) => Some((k.clone(), n.to_string())),
                serde_json::Value::String(s) if s.is_empty() => None,
                serde_json::Value::String(s) => Some((k.clone(), s.clone())),
                serde_json::Value::Array(arr) => {
                    let joined = arr
                        .iter()
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    if joined.is_empty() {
                        None
                    } else {
                        Some((k.clone(), joined))
                    }
                }
                other => Some((k.clone(), other.to_string())),
            })
            .collect()
    }

    async fn execute_raw(
        &self,
        method: Method,
        url: &str,
        query: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, AlpacaError> {
        self.execute_raw_with_headers(method, url, query, body, None)
            .await
    }

    /// Like [`Self::execute_raw`], but merges `extra_headers` on top of the auth
    /// headers. The extra headers are replayed on every retry attempt, which is
    /// what idempotency keys need.
    async fn execute_raw_with_headers(
        &self,
        method: Method,
        url: &str,
        query: Option<serde_json::Value>,
        body: Option<serde_json::Value>,
        extra_headers: Option<&header::HeaderMap>,
    ) -> Result<serde_json::Value, AlpacaError> {
        let mut attempts = 0u32;
        loop {
            debug!(method = %method, url, attempt = attempts, "sending request");
            let mut req = self.http.request(method.clone(), url);
            req = req.headers(self.auth_headers());
            if let Some(extra) = extra_headers {
                req = req.headers(extra.clone());
            }

            if let Some(ref q) = query {
                req = req.query(&Self::query_pairs(q));
            }
            if let Some(ref b) = body {
                req = req.json(b);
            }

            let resp = req.send().await?;
            let status = resp.status();
            let status_code = status.as_u16();

            if self.retry_status_codes.contains(&status_code) && attempts < self.retry_attempts {
                attempts += 1;
                let exp = attempts.saturating_sub(1).min(6);
                let backoff = self.retry_wait_secs.saturating_mul(1u64 << exp).min(60);
                warn!(
                    status = status_code,
                    attempt = attempts,
                    wait_secs = backoff,
                    "retryable error, backing off"
                );
                sleep(Duration::from_secs(backoff)).await;
                continue;
            }

            if status == StatusCode::NO_CONTENT {
                return Ok(serde_json::Value::Null);
            }

            let text = resp.text().await?;

            if !status.is_success() {
                let parsed: serde_json::Value =
                    serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                let message = parsed["message"].as_str().unwrap_or(&text).to_string();
                let code = parsed["code"].as_u64().unwrap_or(0) as u32;
                warn!(status = status_code, code, message, body = %text, "API error");
                return Err(AlpacaError::Api {
                    status_code,
                    code,
                    message,
                });
            }

            if text.is_empty() {
                return Ok(serde_json::Value::Null);
            }

            return Ok(serde_json::from_str(&text)?);
        }
    }

    pub async fn get<Q, R>(&self, path: &str, query: Option<&Q>) -> Result<R, AlpacaError>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let q = query.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::GET, &url, q, None).await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn get_raw<Q>(
        &self,
        path: &str,
        query: Option<&Q>,
    ) -> Result<serde_json::Value, AlpacaError>
    where
        Q: Serialize,
    {
        let url = self.build_url(path);
        let q = query.map(serde_json::to_value).transpose()?;
        self.execute_raw(Method::GET, &url, q, None).await
    }

    pub async fn post<B, R>(&self, path: &str, body: Option<&B>) -> Result<R, AlpacaError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::POST, &url, None, b).await?;
        Ok(serde_json::from_value(val)?)
    }

    /// POST with additional request headers merged on top of the auth headers.
    ///
    /// Header names or values that are not valid HTTP headers are skipped.
    pub async fn post_with_headers<B, R>(
        &self,
        path: &str,
        body: Option<&B>,
        headers: &[(&str, &str)],
    ) -> Result<R, AlpacaError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        let mut map = header::HeaderMap::new();
        for (name, value) in headers {
            if let (Ok(n), Ok(v)) = (
                name.parse::<header::HeaderName>(),
                value.parse::<header::HeaderValue>(),
            ) {
                map.insert(n, v);
            }
        }
        let val = self
            .execute_raw_with_headers(Method::POST, &url, None, b, Some(&map))
            .await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn post_void<B>(&self, path: &str, body: Option<&B>) -> Result<(), AlpacaError>
    where
        B: Serialize + ?Sized,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        self.execute_raw(Method::POST, &url, None, b).await?;
        Ok(())
    }

    pub async fn patch<B, R>(&self, path: &str, body: Option<&B>) -> Result<R, AlpacaError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::PATCH, &url, None, b).await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn put<B, R>(&self, path: &str, body: Option<&B>) -> Result<R, AlpacaError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::PUT, &url, None, b).await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn delete_void<Q>(&self, path: &str, query: Option<&Q>) -> Result<(), AlpacaError>
    where
        Q: Serialize,
    {
        let url = self.build_url(path);
        let q = query.map(serde_json::to_value).transpose()?;
        self.execute_raw(Method::DELETE, &url, q, None).await?;
        Ok(())
    }

    pub async fn delete<Q, R>(&self, path: &str, query: Option<&Q>) -> Result<R, AlpacaError>
    where
        Q: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let q = query.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::DELETE, &url, q, None).await?;
        Ok(serde_json::from_value(val)?)
    }

    pub async fn delete_with_body<B, R>(
        &self,
        path: &str,
        body: Option<&B>,
    ) -> Result<R, AlpacaError>
    where
        B: Serialize,
        R: DeserializeOwned,
    {
        let url = self.build_url(path);
        let b = body.map(serde_json::to_value).transpose()?;
        let val = self.execute_raw(Method::DELETE, &url, None, b).await?;
        Ok(serde_json::from_value(val)?)
    }
}
