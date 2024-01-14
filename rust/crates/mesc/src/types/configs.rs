use crate::{validate, ChainId, MescError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Endpoint
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Endpoint {
    /// name
    pub name: String,
    /// url
    pub url: String,
    /// chain_id
    pub chain_id: Option<ChainId>,
    /// endpoint_metadata
    pub endpoint_metadata: HashMap<String, serde_json::Value>,
}

impl Endpoint {
    /// chain_id as String
    pub fn chain_id_string(&self) -> String {
        self.chain_id.clone().map(|x| x.to_string()).unwrap_or("-".to_string())
    }
}

/// Profile
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Profile {
    /// name
    pub name: String,
    /// default_endpoint
    pub default_endpoint: Option<String>,
    /// network_defaults
    pub network_defaults: HashMap<ChainId, String>,
    /// profile metadata
    pub profile_metadata: HashMap<String, serde_json::Value>,
    /// use mesc
    pub use_mesc: bool,
}

impl Profile {
    /// create new profile
    pub fn new<T: Into<String>>(name: T) -> Profile {
        Profile {
            name: name.into(),
            default_endpoint: None,
            network_defaults: HashMap::new(),
            profile_metadata: HashMap::new(),
            use_mesc: true,
        }
    }
}

/// ConfigMode
#[derive(Debug)]
pub enum ConfigMode {
    /// Path
    Path,
    /// Env
    Env,
    /// Disabled
    Disabled,
}

/// RpcConfig
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct RpcConfig {
    /// mesc version
    pub mesc_version: String,
    /// default endpoint
    pub default_endpoint: Option<String>,
    /// endpoints
    pub endpoints: HashMap<String, Endpoint>,
    /// network defaults
    pub network_defaults: HashMap<ChainId, String>,
    /// network names
    pub network_names: HashMap<String, ChainId>,
    /// profile
    pub profiles: HashMap<String, Profile>,
    /// global metadata
    pub global_metadata: HashMap<String, serde_json::Value>,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            mesc_version: env!("CARGO_PKG_VERSION").to_string(),
            default_endpoint: None,
            network_defaults: HashMap::new(),
            network_names: HashMap::new(),
            endpoints: HashMap::new(),
            profiles: HashMap::new(),
            global_metadata: HashMap::new(),
        }
    }
}

impl RpcConfig {
    /// serialize config
    pub fn serialize(&self) -> Result<String, MescError> {
        Ok(serde_json::to_string(self)?)
    }

    /// validate config
    pub fn validate(&self) -> Result<(), MescError> {
        validate::validate_config(self)
    }
}
