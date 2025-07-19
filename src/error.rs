use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortoError {
    #[error("Failed to connect to wallet: {0}")]
    WalletConnectionError(String),

    #[error("Failed to create account: {0}")]
    AccountCreationError(String),

    #[error("Failed to onramp funds: {0}")]
    OnrampError(String),

    #[error("Failed to initialize account: {0}")]
    InitializationError(String),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Crypto error: {0}")]
    CryptoError(String),

    #[error("Dialog communication error: {0}")]
    DialogError(String),

    #[error("Dialog interaction error: {0}")]
    DialogueError(#[from] dialoguer::Error),
}

pub type Result<T> = std::result::Result<T, PortoError>;