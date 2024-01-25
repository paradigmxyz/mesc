use crate::{overrides::apply_overrides, ConfigMode, MescError, RpcConfig};
use std::{env, fs};

/// check whether mesc is enabled
pub fn is_mesc_enabled() -> bool {
    if let Ok("DISABLED") = std::env::var("MESC_MODE").as_deref() {
        return false;
    };
    let env_vars = [
        "MESC_MODE",
        "MESC_PATH",
        "MESC_ENV",
        "MESC_NETWORK_NAMES",
        "MESC_NETWORK_DEFAULTS",
        "MESC_ENDPOINTS",
        "MESC_DEFAULT_ENDPOINT",
        "MESC_GLOBAL_METADATA",
        "MESC_ENDPOINT_METADATA",
        "MESC_PROFILES",
    ];
    for env_var in env_vars.iter() {
        match std::env::var(env_var).as_deref() {
            Ok(value) if !value.is_empty() => return true,
            _ => {}
        }
    }
    false
}

/// get config mode
pub fn get_config_mode() -> Result<ConfigMode, MescError> {
    let mode = env::var("MESC_MODE").unwrap_or_default();
    if mode == "PATH" {
        return Ok(ConfigMode::Path);
    } else if mode == "ENV" {
        return Ok(ConfigMode::Env);
    } else if mode == "DISABLED" {
        return Ok(ConfigMode::Disabled);
    } else if !mode.is_empty() {
        return Err(MescError::InvalidConfigMode);
    }
    if let Ok(path) = env::var("MESC_PATH") {
        if !path.is_empty() {
            return Ok(ConfigMode::Path);
        }
    }
    if let Ok(env_config) = env::var("MESC_ENV") {
        if !env_config.is_empty() {
            return Ok(ConfigMode::Env);
        }
    }

    Ok(ConfigMode::Disabled)
}

/// load config data
pub fn load_config_data() -> Result<RpcConfig, MescError> {
    let config = match get_config_mode() {
        Ok(ConfigMode::Path) => load_file_config(None),
        Ok(ConfigMode::Env) => load_env_config(),
        Ok(ConfigMode::Disabled) => Err(MescError::MescNotEnabled),
        Err(e) => Err(e),
    };

    let mut config = config?;
    apply_overrides(&mut config)?;
    Ok(config)
}

/// load env config
pub fn load_env_config() -> Result<RpcConfig, MescError> {
    let config_json = env::var("MESC_ENV")?;
    serde_json::from_str(&config_json).map_err(|_| MescError::InvalidJson)
}

/// load file config
pub fn load_file_config(path: Option<String>) -> Result<RpcConfig, MescError> {
    let path = match path {
        Some(path) => path,
        None => get_config_path()?,
    };
    if !std::path::Path::new(path.as_str()).exists() {
        return Err(MescError::MissingConfigFile(path));
    };
    let config_str = fs::read_to_string(path).map_err(MescError::IOError)?;
    serde_json::from_str(&config_str).map_err(|_| MescError::InvalidJson)
}

/// get config path
pub fn get_config_path() -> Result<String, MescError> {
    let path = env::var("MESC_PATH")?;
    let path = expand_path(path)?;
    Ok(path)
}

/// expand tilde's in path
pub fn expand_path<P: AsRef<std::path::Path>>(path: P) -> Result<String, MescError> {
    let path_str =
        path.as_ref().to_str().ok_or(MescError::InvalidPath("Invalid path".to_string()))?;

    if let Some(subpath) = path_str.strip_prefix("~/") {
        Ok(format!("{}/{}", env::var("HOME")?, subpath))
    } else {
        Ok(path_str.to_string())
    }
}
