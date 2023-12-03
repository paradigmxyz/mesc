use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub chain_id: ChainId,
    pub endpoint_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub default_endpoint: Option<String>,
    pub network_defaults: HashMap<ChainId, String>,
}

#[derive(Debug)]
pub enum ConfigMode {
    Path,
    Env,
    Disabled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcConfig {
    pub mesc_version: String,
    pub default_endpoint: Option<String>,
    pub network_defaults: HashMap<ChainId, String>,
    pub endpoints: HashMap<String, Endpoint>,
    pub network_names: HashMap<String, ChainId>,
    pub profiles: HashMap<String, Profile>,
    pub global_metadata: HashMap<String, serde_json::Value>,
}

impl RpcConfig {
    pub fn validate(&self) -> Result<(), MescError> {
        validate::validate_config(self)
    }
}

#[derive(Debug)]
pub enum MescError {
    MescNotEnabled,
    InvalidConfigMode,
    InvalidChainId(String),
    MissingEndpoint(String),
    FileReadError(std::io::Error),
    InvalidJson,
    EnvReadError,
    NotImplemented(String),
}

/// ChainId is a string representation of an integer chain id
/// - TryFrom conversions allow specifying as String, &str, uint, or binary data
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChainId(String);

impl std::fmt::Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_from_uint_for_chainid {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ChainId {
                fn from(value: $t) -> ChainId {
                    ChainId(value.to_string())
                }
            }
        )*
    };
}

impl_from_uint_for_chainid!(u8, u16, u32, u64, u128, usize);

/// use custom trait instead of TryInto so that Error type is always the same
pub trait TryIntoChainId {
    fn try_into_chain_id(self) -> Result<ChainId, MescError>;
}

impl TryIntoChainId for String {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        if self.chars().all(|c| c.is_ascii_digit()) {
            Ok(ChainId(self))
        } else {
            Err(MescError::InvalidChainId(self))
        }
    }
}

impl TryIntoChainId for &str {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        if self.chars().all(|c| c.is_ascii_digit()) {
            Ok(ChainId(self.to_string()))
        } else {
            Err(MescError::InvalidChainId(self.to_string()))
        }
    }
}

macro_rules! impl_try_into_chain_id_for_integer {
    ($($t:ty),*) => {
        $(
            impl TryIntoChainId for $t {
                fn try_into_chain_id(self) -> Result<ChainId, MescError> {
                    Ok(ChainId(self.to_string()))
                }
            }
        )*
    };
}

impl_try_into_chain_id_for_integer!(u8, u16, u32, u64, u128, usize);

impl TryIntoChainId for &[u8] {
    fn try_into_chain_id(self) -> Result<ChainId, MescError> {
        Err(MescError::NotImplemented("binary chain_id".to_string()))
    }
}

#[derive(Debug, Default, Clone)]
pub struct EndpointQuery {
    pub chain_id: Option<ChainId>,
}

impl EndpointQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chain_id<T: TryIntoChainId>(mut self, chain_id: T) -> Result<Self, MescError> {
        self.chain_id = Some(chain_id.try_into_chain_id()?);
        Ok(self)
    }
}
