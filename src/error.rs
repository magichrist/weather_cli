use thiserror::Error;

/// Errors that can occur in the weather CLI application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("cache error: {0}")]
    Cache(String),

    #[error("invalid latitude {lat} — must be between -90 and 90")]
    InvalidLatitude { lat: f64 },

    #[error("invalid longitude {lon} — must be between -180 and 180")]
    InvalidLongitude { lon: f64 },

    #[error("API returned unexpected response")]
    UnexpectedResponse,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("task join error: {0}")]
    Join(#[from] tokio::task::JoinError),
}

/// Convenience alias for Results using [`AppError`].
pub type AppResult<T> = Result<T, AppError>;
