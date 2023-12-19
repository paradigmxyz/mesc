use crate::{validate, ChainId, MescError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub chain_id: Option<ChainId>,
    pub endpoint_metadata: HashMap<String, serde_json::Value>,
}

impl Endpoint {
    pub fn chain_id_string(&self) -> String {
        self.chain_id
            .clone()
            .map(|x| x.to_string())
            .unwrap_or("-".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub default_endpoint: Option<String>,
    pub network_defaults: HashMap<ChainId, String>,
}

#[derive(Debug)]
pub enum ConfigMode {
    Path,
    Env,
    Disabled,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcConfig {
    pub mesc_version: String,
    pub default_endpoint: Option<String>,
    pub network_defaults: HashMap<ChainId, String>,
    pub network_names: HashMap<String, ChainId>,
    pub endpoints: HashMap<String, Endpoint>,
    pub profiles: HashMap<String, Profile>,
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
    pub fn serialize(&self) -> Result<String, MescError> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn validate(&self) -> Result<(), MescError> {
        validate::validate_config(self)
    }
}
