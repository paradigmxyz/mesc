use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub url: String,
    pub chain_id: u64,
    pub endpoint_extras: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub default_network: Option<u64>,
    pub default_endpoints: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcConfig {
    pub schema: String,
    pub default_network: Option<u64>,
    pub default_endpoints: HashMap<String, String>,
    pub endpoints: HashMap<String, Endpoint>,
    pub profiles: HashMap<String, Profile>,
    pub global_extras: HashMap<String, serde_json::Value>,
}

pub fn get_default_network(profile: Option<&str>, config: &RpcConfig) -> Option<u64> {
    match profile {
        Some(profile_name) => config
            .profiles
            .get(profile_name)
            .and_then(|p| p.default_network),
        None => config.default_network,
    }
}

pub fn get_default_endpoint(
    chain_id: u64,
    profile: Option<&str>,
    config: &RpcConfig,
) -> Option<Endpoint> {
    let endpoint_name = match profile {
        Some(profile_name) => config
            .profiles
            .get(profile_name)
            .and_then(|p| p.default_endpoints.get(&chain_id.to_string())),
        None => config.default_endpoints.get(&chain_id.to_string()),
    }?;

    config.endpoints.get(endpoint_name).cloned()
}

pub fn get_endpoint_by_name(name: &str, config: &RpcConfig) -> Option<Endpoint> {
    config.endpoints.get(name).cloned()
}

pub fn read_config_data() -> Result<RpcConfig, Box<dyn std::error::Error>> {
    let mode = env::var("RPC_CONFIG_MODE").unwrap_or_default();
    match mode.as_str() {
        "PATH" => read_file_config(),
        "ENV" => read_env_config(),
        _ if !mode.is_empty() => Err("Invalid RPC_CONFIG_MODE value".into()),
        _ => {
            if let Ok(path) = env::var("RPC_CONFIG_PATH") {
                read_file_config_from_path(&path)
            } else if let Ok(env_config) = env::var("RPC_CONFIG_ENV") {
                serde_json::from_str(&env_config).map_err(Into::into)
            } else {
                Err("MESC configuration not specified".into())
            }
        }
    }
}

pub fn read_env_config() -> Result<RpcConfig, Box<dyn std::error::Error>> {
    let config_json = env::var("RPC_CONFIG_ENV")?;
    serde_json::from_str(&config_json).map_err(Into::into)
}

pub fn read_file_config() -> Result<RpcConfig, Box<dyn std::error::Error>> {
    let path = env::var("RPC_CONFIG_PATH")?;
    read_file_config_from_path(&path)
}

pub fn read_file_config_from_path(path: &str) -> Result<RpcConfig, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    serde_json::from_str(&config_str).map_err(Into::into)
}
