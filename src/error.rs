use thiserror::Error;

#[derive(Debug, Error)]
pub enum AlpacaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error (status={status_code}, code={code}): {message}")]
    Api {
        status_code: u16,
        code: u32,
        message: String,
    },

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    #[error("Msgpack error: {0}")]
    Msgpack(String),
}
