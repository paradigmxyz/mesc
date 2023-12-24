use mesc::MescError;

#[derive(Debug)]
pub(crate) enum MescCliError {
    MescError(MescError),
    NetworkError(reqwest::Error),
    InquireError(inquire::InquireError),
    SerdeError(serde_json::Error),
    IOError(std::io::Error),
    InvalidNetworkResponse,
    JoinError(tokio::task::JoinError),
    InvalidInput(String),
    FormatError(toolstr::FormatError),
    Error(String),
    UrlError(String),
}

impl From<mesc::MescError> for MescCliError {
    fn from(value: mesc::MescError) -> Self {
        MescCliError::MescError(value)
    }
}

impl From<std::io::Error> for MescCliError {
    fn from(value: std::io::Error) -> Self {
        MescCliError::IOError(value)
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

impl From<toolstr::FormatError> for MescCliError {
    fn from(value: toolstr::FormatError) -> MescCliError {
        MescCliError::FormatError(value)
    }
}
