use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortoError {
    #[error("Failed to create account: {0}")]
    AccountCreation(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Dialog interaction error: {0}")]
    Dialogue(#[from] dialoguer::Error),
}

pub type Result<T> = std::result::Result<T, PortoError>;
