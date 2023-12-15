use crate::{ChainId, Endpoint, MescError, Profile, RpcConfig, TryIntoChainId};
use std::collections::HashMap;

pub fn apply_overrides(config: &mut RpcConfig) -> Result<(), MescError> {
    if let Some(default_endpoint) = get_default_endpoint_override() {
        config.default_endpoint = Some(default_endpoint)
    }
    if let Some(network_defaults) = get_network_defaults_override()? {
        config.network_defaults = network_defaults
    }
    if let Some(network_names) = get_network_names_override()? {
        config.network_names = network_names
    }
    if let Some(endpoints) = get_endpoints_override()? {
        config.endpoints = endpoints
    }
    if let Some(profiles) = get_profiles_override()? {
        config.profiles = profiles
    }
    if let Some(global_metadata) = get_global_metadata_override()? {
        config.global_metadata = global_metadata
    }
    if let Some(endpoint_metadatas) = get_endpoint_metadata_override()? {
        for (name, metadata) in endpoint_metadatas.into_iter() {
            if let Some(endpoint) = config.endpoints.get_mut(&name) {
                endpoint.endpoint_metadata.extend(metadata)
            } else {
                return Err(MescError::OverrideError(format!(
                    "endpoint does not exist: {}",
                    name
                )));
            }
        }
    }

    Ok(())
}

fn get_default_endpoint_override() -> Option<String> {
    std::env::var("MESC_DEFAULT_ENDPOINT").ok()
}

fn get_network_defaults_override() -> Result<Option<HashMap<ChainId, String>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_NETWORK_DEFAULTS") {
        let mut network_defaults = HashMap::new();
        for piece in raw.split(' ') {
            match piece.split('=').collect::<Vec<&str>>().as_slice() {
                [chain_id, endpoint] => {
                    network_defaults.insert(
                        (chain_id.to_string()).try_into_chain_id()?,
                        endpoint.to_string(),
                    );
                }
                _ => {
                    return Err(MescError::OverrideError(
                        "invalid network default override".to_string(),
                    ))
                }
            }
        }
        Ok(Some(network_defaults))
    } else {
        Ok(None)
    }
}

fn get_network_names_override() -> Result<Option<HashMap<String, ChainId>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_NETWORK_DEFAULTS") {
        let mut network_names = HashMap::new();
        for piece in raw.split(' ') {
            match piece.split('=').collect::<Vec<&str>>().as_slice() {
                [name, chain_id] => {
                    network_names.insert(
                        name.to_string(),
                        (chain_id.to_string()).try_into_chain_id()?,
                    );
                }
                _ => {
                    return Err(MescError::OverrideError(
                        "invalid network name override".to_string(),
                    ))
                }
            }
        }
        Ok(Some(network_names))
    } else {
        Ok(None)
    }
}

fn get_endpoints_override() -> Result<Option<HashMap<String, Endpoint>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_ENDPOINTS") {
        let mut endpoints = HashMap::new();
        for piece in raw.split(' ') {
            let endpoint = parse_endpoint(piece)?;
            endpoints.insert(endpoint.name.clone(), endpoint);
        }
        Ok(Some(endpoints))
    } else {
        Ok(None)
    }
}

fn parse_endpoint(input: &str) -> Result<Endpoint, MescError> {
    let mut parts = input.split('=');
    let (name_chain, url) = match (parts.next(), parts.next()) {
        (Some(name), Some(url)) => (name, url),
        (None, Some(url)) => ("", url),
        _ => {
            return Err(MescError::OverrideError(
                "invalid endpoint override".to_string(),
            ))
        }
    };

    let mut name_chain_parts = name_chain.split(':');
    let name = name_chain_parts
        .next()
        .map(|s| s.to_string())
        .unwrap_or_else(|| get_default_name(url))
        .to_string();
    let chain_id = match name_chain_parts.next() {
        Some(chain_id) => Some(chain_id.try_into_chain_id()?),
        None => None,
    };

    Ok(Endpoint {
        name,
        url: url.to_string(),
        chain_id,
        endpoint_metadata: HashMap::new(),
    })
}

fn get_default_name(url: &str) -> String {
    url.to_string()
}

fn get_profiles_override() -> Result<Option<HashMap<String, Profile>>, MescError> {
    let raw = match std::env::var("MESC_NETWORK_DEFAULTS") {
        Ok(raw) => raw,
        Err(_) => return Ok(None),
    };

    let mut profiles: HashMap<String, Profile> = HashMap::new();

    // Splitting the string into entries
    let entries = raw.split_whitespace().collect::<Vec<&str>>();

    for entry in entries {
        let mut parts = entry.split('=');
        let (left_side, endpoint) = match (parts.next(), parts.next()) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                return Err(MescError::OverrideError(
                    "invalid profiles override".to_string(),
                ))
            }
        };

        let mut left_parts = left_side.split('.');
        let (profile_name, key, chain_id) =
            match (left_parts.next(), left_parts.next(), left_parts.next()) {
                (Some(p), Some(k), cid) => (p, k, cid),
                _ => {
                    return Err(MescError::OverrideError(
                        "invalid profiles override".to_string(),
                    ))
                }
            };

        let profile = profiles
            .entry(profile_name.to_string())
            .or_insert_with(|| Profile {
                default_endpoint: None,
                network_defaults: HashMap::new(),
            });

        match key {
            "default" => profile.default_endpoint = Some(endpoint.to_string()),
            "network" => {
                if let Some(cid) = chain_id {
                    profile
                        .network_defaults
                        .insert(cid.try_into_chain_id()?, endpoint.to_string());
                }
            }
            _ => {
                return Err(MescError::OverrideError(
                    "invalid profile override".to_string(),
                ))
            }
        }
    }

    Ok(Some(profiles))
}

fn get_global_metadata_override() -> Result<Option<HashMap<String, serde_json::Value>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_GLOBAL_METADATA") {
        let parsed: Result<HashMap<String, serde_json::Value>, _> =
            serde_json::from_str(raw.as_str());
        Ok(Some(parsed?))
    } else {
        Ok(None)
    }
}

type Metadata = HashMap<String, serde_json::Value>;

fn get_endpoint_metadata_override() -> Result<Option<HashMap<String, Metadata>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_GLOBAL_METADATA") {
        let parsed: Result<HashMap<String, HashMap<String, serde_json::Value>>, _> =
            serde_json::from_str(raw.as_str());
        Ok(Some(parsed?))
    } else {
        Ok(None)
    }
}
