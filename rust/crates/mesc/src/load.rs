use crate::types::{RpcConfig, MescError};
use std::env;
use std::fs;

pub fn is_mesc_enabled() -> bool {
    if let Ok(value) = env::var("RPC_CONFIG_MODE") {
        if !value.is_empty() {
            return true
        }
    }
    if let Ok(value) = env::var("RPC_CONFIG_PATH") {
        if !value.is_empty() {
            return true
        }
    }
    if let Ok(value) = env::var("RPC_CONFIG_ENV") {
        if !value.is_empty() {
            return true
        }
    }
    false
}

pub fn load_env_profile() -> Option<String> {
    match env::var("RPC_PROFILE") {
        Ok(s) => Some(s),
        _ => None,
    }
}

pub fn load_config_data() -> Result<RpcConfig, MescError> {
    let mode = env::var("RPC_CONFIG_MODE").unwrap_or_default();
    let mode = mode.as_str();
    if mode == "PATH" {
        load_file_config()
    } else if mode == "ENV" {
        load_env_config()
    } else if !mode.is_empty() {
        Err(MescError::InvalidConfigMode)
    } else if let Ok(path) = env::var("RPC_CONFIG_PATH") {
        load_file_config_from_path(&path)
    } else if let Ok(env_config) = env::var("RPC_CONFIG_ENV") {
        serde_json::from_str(&env_config).map_err(|_| MescError::EnvRead)
    } else {
        Err(MescError::ConfigNotSpecified)
    }
}

pub fn load_env_config() -> Result<RpcConfig, MescError> {
    let config_json = env::var("RPC_CONFIG_ENV").map_err(|_| MescError::EnvRead)?;
    serde_json::from_str(&config_json).map_err(|_| MescError::EnvRead)
}

pub fn load_file_config() -> Result<RpcConfig, MescError> {
    let path = env::var("RPC_CONFIG_PATH").map_err(|_| MescError::EnvRead)?;
    load_file_config_from_path(&path)
}

pub fn load_file_config_from_path(path: &str) -> Result<RpcConfig, MescError> {
    let config_str = fs::read_to_string(path).map_err(|_| MescError::EnvRead)?;
    serde_json::from_str(&config_str).map_err(|_| MescError::EnvRead)
}
