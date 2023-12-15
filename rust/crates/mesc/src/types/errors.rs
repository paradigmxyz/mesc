#[derive(Debug)]
pub enum MescError {
    MescNotEnabled,
    InvalidConfigMode,
    InvalidChainId(String),
    IntegrityError(String),
    MissingEndpoint(String),
    IOError(std::io::Error),
    InvalidJson,
    EnvReadError(std::env::VarError),
    NotImplemented(String),
    SerdeError(serde_json::Error),
    InvalidInput,
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
