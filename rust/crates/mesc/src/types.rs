use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub chain_id: u64,
    pub endpoint_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub default_network: Option<u64>,
    pub default_network_endpoints: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcConfig {
    pub schema: String,
    pub default_network: Option<u64>,
    pub default_network_endpoints: HashMap<String, String>,
    pub endpoints: HashMap<String, Endpoint>,
    pub network_names: HashMap<String, u64>,
    pub profiles: HashMap<String, Profile>,
    pub global_metadata: HashMap<String, serde_json::Value>,
}

pub enum ConfigError {
    ConfigNotSpecified,
    InvalidConfigMode,
    MissingEndpoint,
    FileDoesNotExist,
    EnvRead,
}

