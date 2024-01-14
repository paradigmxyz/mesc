use mesc::MescError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MescCliError {
    #[error("MESC error: {0}")]
    MescError(#[from] MescError),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),

    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Invalid network response")]
    InvalidNetworkResponse,

    #[error("Join error: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Format error: {0}")]
    FormatError(#[from] toolstr::FormatError),

    #[error("Error: {0}")]
    Error(String),

    #[error("URL error: {0}")]
    UrlError(String),
}
