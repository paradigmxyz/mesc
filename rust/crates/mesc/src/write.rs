use crate::{MescError, RpcConfig, TryIntoChainId};
use std::{fs::File, path::Path};

/// write config to file
pub fn write_config<P: AsRef<Path>>(config: RpcConfig, path: P) -> Result<(), MescError> {
    let path_ref = path.as_ref();

    if let Some(parent) = path_ref.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?
        };
    }

    let file = File::create(path_ref)?;
    Ok(serde_json::to_writer_pretty(file, &config)?)
}

/// update name of endpoint
pub fn update_endpoint_name(
    config: &mut RpcConfig,
    old_name: &str,
    new_name: &str,
) -> Result<(), MescError> {
    // check that new name is valid
    if !config.endpoints.contains_key(old_name) {
        return Err(MescError::IntegrityError(format!("endpoint not in config: {}", old_name)));
    }
    if config.endpoints.contains_key(new_name) {
        return Err(MescError::IntegrityError(format!("endpoint already in config: {}", new_name)));
    }

    // update endpoint
    match config.endpoints.get_mut(old_name) {
        Some(endpoint) => endpoint.name = new_name.to_string(),
        None => {
            return Err(MescError::IntegrityError(format!("endpoint not in config: {}", old_name)))
        }
    }

    // update defaults
    if config.default_endpoint == Some(old_name.to_string()) {
        config.default_endpoint = Some(new_name.to_string());
    }
    for value in config.network_defaults.values_mut() {
        if value == old_name {
            *value = new_name.to_string();
        }
    }

    // update profiles
    for profile in config.profiles.values_mut() {
        if profile.default_endpoint == Some(old_name.to_string()) {
            profile.default_endpoint = Some(new_name.to_string());
        }
        for value in profile.network_defaults.values_mut() {
            if value == old_name {
                *value = new_name.to_string();
            }
        }
    }

    Ok(())
}

/// update endpoint chain_id
pub fn update_endpoint_chain_id<T: TryIntoChainId>(
    config: &mut RpcConfig,
    endpoint_name: &str,
    chain_id: T,
) -> Result<(), MescError> {
    let chain_id = chain_id.try_into_chain_id()?;

    match config.endpoints.get_mut(endpoint_name) {
        Some(endpoint) => endpoint.chain_id = Some(chain_id.try_into_chain_id()?),
        None => {
            return Err(MescError::IntegrityError(format!(
                "endpoint not present: {}",
                endpoint_name
            )))
        }
    };

    // update defaults
    for (chain_id, value) in config.network_defaults.clone().iter() {
        if value == endpoint_name {
            config.network_defaults.remove(chain_id);
        }
    }

    // update profiles
    for profile in config.profiles.values_mut() {
        for (chain_id, value) in profile.network_defaults.clone().iter() {
            if value == endpoint_name {
                config.network_defaults.remove(chain_id);
            }
        }
    }

    Ok(())
}

/// delete endpoint from config
pub fn delete_endpoint(config: &mut RpcConfig, endpoint: &str) -> Result<(), MescError> {
    // remove from endpoints
    config.endpoints.remove(endpoint);

    // remove from defaults
    if config.default_endpoint.as_deref() == Some(endpoint) {
        config.default_endpoint = None;
    }
    for (chain_id, name) in config.network_defaults.clone().iter() {
        if name == endpoint {
            config.network_defaults.remove(chain_id);
        }
    }

    // remove from profiles
    for profile in config.profiles.values_mut() {
        if profile.default_endpoint.as_deref() == Some(endpoint) {
            profile.default_endpoint = None;
        }
        for (chain_id, name) in profile.network_defaults.clone().iter() {
            if name == endpoint {
                profile.network_defaults.remove(chain_id);
            }
        }
    }

    Ok(())
}
