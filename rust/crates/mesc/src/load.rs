use crate::types::{MescError, RpcConfig};
use std::env;
use std::fs;

pub fn is_mesc_enabled() -> bool {
    matches!(
        get_config_mode(),
        Ok(ConfigMode::Path) | Ok(ConfigMode::Env)
    )
}
use crate::ConfigMode;

pub fn get_config_mode() -> Result<ConfigMode, MescError> {
    let mode = env::var("MESC_CONFIG_MODE").unwrap_or_default();
    if mode == "PATH" {
        return Ok(ConfigMode::Path);
    } else if mode == "ENV" {
        return Ok(ConfigMode::Env);
    } else if mode == "DISABLED" {
        return Ok(ConfigMode::Disabled);
    } else if !mode.is_empty() {
        return Err(MescError::InvalidConfigMode);
    }
    if let Ok(path) = env::var("MESC_CONFIG_PATH") {
        if !path.is_empty() {
            return Ok(ConfigMode::Path);
        }
    }
    if let Ok(env_config) = env::var("MESC_CONFIG_ENV") {
        if !env_config.is_empty() {
            return Ok(ConfigMode::Env);
        }
    }

    Ok(ConfigMode::Disabled)
}

pub fn load_config_data() -> Result<RpcConfig, MescError> {
    match get_config_mode() {
        Ok(ConfigMode::Path) => load_file_config(),
        Ok(ConfigMode::Env) => load_env_config(),
        Ok(ConfigMode::Disabled) => Err(MescError::MescNotEnabled),
        Err(e) => Err(e),
    }
}

pub fn load_env_config() -> Result<RpcConfig, MescError> {
    let config_json = env::var("MESC_CONFIG_ENV")?;
    serde_json::from_str(&config_json).map_err(|_| MescError::InvalidJson)
}

pub fn load_file_config() -> Result<RpcConfig, MescError> {
    let path = get_config_path()?;
    let config_str = fs::read_to_string(path).map_err(MescError::IOError)?;
    serde_json::from_str(&config_str).map_err(|_| MescError::InvalidJson)
}

pub fn get_config_path() -> Result<String, MescError> {
    let path = env::var("MESC_CONFIG_PATH")?;
    let path = expand_path(path)?;
    Ok(path)
}

fn expand_path(path: String) -> Result<String, MescError> {
    if let Some(subpath) = path.strip_prefix("~/") {
        Ok(format!("{}/{}", env::var("HOME")?, subpath))
    } else {
        Ok(path)
    }
}
