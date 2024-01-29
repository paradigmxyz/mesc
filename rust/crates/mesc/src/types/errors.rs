use thiserror::Error;

/// Errors related to MESC operations.
#[derive(Error, Debug)]
pub enum MescError {
    /// Error indicating that MESC is not enabled.
    #[error("Mesc is not enabled")]
    MescNotEnabled,

    /// Error for invalid configuration modes.
    #[error("Invalid configuration mode")]
    InvalidConfigMode,

    /// Error for invalid chain ID, with the invalid ID provided.
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),

    /// Error representing an integrity issue, with a description.
    #[error("Integrity error: {0}")]
    IntegrityError(String),

    /// Error for missing endpoint, specifying which endpoint is missing.
    #[error("Missing endpoint: {0}")]
    MissingEndpoint(String),

    /// Error for missing invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Error for missing config file
    #[error("Missing config file: {0}")]
    MissingConfigFile(String),

    /// Error wrapper for standard IO errors.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    /// Error indicating an issue with JSON formatting.
    #[error("Invalid JSON format")]
    InvalidJson,

    /// Error wrapper for environment variable read errors.
    #[error(transparent)]
    EnvReadError(#[from] std::env::VarError),

    /// Error indicating a feature or function is not implemented, with a description.
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Error wrapper for errors from the `serde_json` crate.
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    /// General error for invalid input.
    #[error("Invalid input")]
    InvalidInput,

    /// Error for override conflicts, with a description of the conflict.
    #[error("Override error: {0}")]
    OverrideError(String),
}
