/// MescError
#[derive(Debug)]
pub enum MescError {
    /// MescNotEnabled
    MescNotEnabled,
    /// InvalidConfigMode
    InvalidConfigMode,
    /// InvalidChainId
    InvalidChainId(String),
    /// IntegrityError
    IntegrityError(String),
    /// MissingEndpoint
    MissingEndpoint(String),
    /// IOError
    IOError(std::io::Error),
    /// InvalidJson
    InvalidJson,
    /// EnvReadError
    EnvReadError(std::env::VarError),
    /// NotImplemented
    NotImplemented(String),
    /// SerdeError
    SerdeError(serde_json::Error),
    /// InvalidInput
    InvalidInput,
    /// OverrideError
    OverrideError(String),
}

impl From<std::io::Error> for MescError {
    fn from(value: std::io::Error) -> MescError {
        MescError::IOError(value)
    }
}

impl From<serde_json::Error> for MescError {
    fn from(value: serde_json::Error) -> MescError {
        MescError::SerdeError(value)
    }
}

impl From<std::env::VarError> for MescError {
    fn from(value: std::env::VarError) -> MescError {
        MescError::EnvReadError(value)
    }
}
