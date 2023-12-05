use mesc::MescError;

#[derive(Debug)]
pub enum MescCliError {
    MescError(MescError),
    NetworkError(reqwest::Error),
    InquireError(inquire::InquireError),
    SerdeError(serde_json::Error),
    IOError(std::io::Error),
    InvalidNetworkResponse,
    JoinError(tokio::task::JoinError),
    InvalidInput,
}

impl From<mesc::MescError> for MescCliError {
    fn from(value: mesc::MescError) -> Self {
        MescCliError::MescError(value)
    }
}

impl From<reqwest::Error> for MescCliError {
    fn from(value: reqwest::Error) -> Self {
        MescCliError::NetworkError(value)
    }
}

impl From<inquire::InquireError> for MescCliError {
    fn from(value: inquire::InquireError) -> Self {
        MescCliError::InquireError(value)
    }
}

impl From<serde_json::Error> for MescCliError {
    fn from(value: serde_json::Error) -> MescCliError {
        MescCliError::SerdeError(value)
    }
}
