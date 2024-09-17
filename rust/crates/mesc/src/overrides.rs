use crate::{ChainId, Endpoint, MescError, Profile, RpcConfig, TryIntoChainId};
use std::collections::HashMap;

/// get active overrides
pub fn get_active_overrides() -> Vec<String> {
    let overrides = [
        "MESC_NETWORK_NAMES",
        "MESC_NETWORK_DEFAULTS",
        "MESC_ENDPOINTS",
        "MESC_DEFAULT_ENDPOINT",
        "MESC_GLOBAL_METADATA",
        "MESC_ENDPOINT_METADATA",
        "MESC_PROFILES",
    ];
    overrides.iter().filter_map(|v| std::env::var(v).ok()).collect()
}

/// apply overrides to config
pub fn apply_overrides(config: &mut RpcConfig) -> Result<(), MescError> {
    if let Some(default_endpoint) = get_default_endpoint_override() {
        if !default_endpoint.is_empty() {
            config.default_endpoint = Some(default_endpoint)
        }
    }
    if let Some(network_defaults) = get_network_defaults_override()? {
        config.network_defaults = network_defaults;
    }
    if let Some(network_names) = get_network_names_override()? {
        config.network_names = network_names
    }
    if let Some(endpoints) = get_endpoints_override()? {
        for (endpoint_name, endpoint) in endpoints.into_iter() {
            if let Some(current_endpoint) = config.endpoints.get_mut(&endpoint_name) {
                if endpoint.chain_id.is_some() {
                    current_endpoint.chain_id = endpoint.chain_id;
                }
                current_endpoint.url = endpoint.url;
            } else {
                config.endpoints.insert(endpoint_name, endpoint);
            }
        }
    }
    if let Some(profiles) = get_profiles_override()? {
        config.profiles = profiles
    }
    if let Some(global_metadata) = get_global_metadata_override()? {
        config.global_metadata.extend(global_metadata)
    }
    if let Some(endpoint_metadatas) = get_endpoint_metadata_override()? {
        for (name, metadata) in endpoint_metadatas.into_iter() {
            if let Some(endpoint) = config.endpoints.get_mut(&name) {
                endpoint.endpoint_metadata.extend(metadata)
            } else {
                return Err(MescError::OverrideError(format!("endpoint does not exist: {}", name)));
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
        if raw.is_empty() {
            return Ok(None);
        }
        let mut network_defaults = HashMap::new();
        for piece in raw.split(' ') {
            match piece.split('=').collect::<Vec<&str>>().as_slice() {
                [chain_id, endpoint] => {
                    network_defaults.insert(chain_id.try_into_chain_id()?, endpoint.to_string());
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
    if let Ok(raw) = std::env::var("MESC_NETWORK_NAMES") {
        if raw.is_empty() {
            return Ok(None);
        }
        let mut network_names = HashMap::new();
        for piece in raw.split(' ') {
            match piece.split('=').collect::<Vec<&str>>().as_slice() {
                [name, chain_id] => {
                    network_names.insert(name.to_string(), chain_id.try_into_chain_id()?);
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
        if raw.is_empty() {
            return Ok(None);
        }
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
        (Some(url), None) => ("", url),
        _ => return Err(MescError::OverrideError("invalid endpoint override".to_string())),
    };

    let mut name_chain_parts = name_chain.split(':');
    let name = name_chain_parts
        .next()
        .map(|s| s.to_string())
        .unwrap_or(
            get_default_endpoint_name(url, None)
                .ok_or(MescError::OverrideError("could not create endpoint name".to_string()))?,
        )
        .to_string();
    let name = if name.is_empty() {
        get_default_endpoint_name(url, None)
            .ok_or(MescError::OverrideError("could not create endpoint name".to_string()))?
    } else {
        name
    };
    let chain_id = match name_chain_parts.next() {
        Some(chain_id) => Some(chain_id.try_into_chain_id()?),
        None => None,
    };

    Ok(Endpoint { name, url: url.to_string(), chain_id, endpoint_metadata: HashMap::new() })
}

/// get default endpoint name for a url
pub fn get_default_endpoint_name(url: &str, chain_id: Option<ChainId>) -> Option<String> {
    // Find the start of the main part of the URL, skipping the protocol if present
    let start = if let Some(pos) = url.find("://") { pos + 3 } else { 0 };

    // Extract the main part of the URL, from the protocol (if present) to the end
    let main_part = &url[start..];

    // Find the end of the domain part, which could be before a slash (path) or the end of the
    // string
    let end = main_part.find('/').unwrap_or(main_part.len());

    // Extract the domain part
    let domain_part = &main_part[..end];

    // Check if the domain part contains a period, which is typical for a hostname
    let hostname = if domain_part.contains('.') {
        // Find the last period in the domain part
        if let Some(last_period_pos) = domain_part.rfind('.') {
            domain_part[..last_period_pos].to_string()
        } else {
            domain_part.to_string()
        }
    } else {
        domain_part.to_string()
    };
    let hostname =
        if let Some(split) = hostname.split('.').last() { split.to_string() } else { hostname };

    match chain_id {
        Some(chain_id) => Some(format!("{hostname}_{chain_id}")),
        None => Some(hostname),
    }
}

fn get_profiles_override() -> Result<Option<HashMap<String, Profile>>, MescError> {
    let raw = match std::env::var("MESC_PROFILES") {
        Ok(raw) => raw,
        Err(_) => return Ok(None),
    };
    if raw.is_empty() {
        return Ok(None);
    }

    let mut profiles: HashMap<String, Profile> = HashMap::new();

    // Splitting the string into entries
    let entries = raw.split_whitespace().collect::<Vec<&str>>();

    for entry in entries {
        let mut parts = entry.split('=');
        let (left_side, endpoint) = match (parts.next(), parts.next()) {
            (Some(l), Some(r)) => (l, r),
            _ => {
                return Err(MescError::OverrideError(format!(
                    "invalid profiles override, bad match: {}",
                    entry
                )))
            }
        };

        let mut left_parts = left_side.split('.');
        let (profile_name, key, chain_id) =
            match (left_parts.next(), left_parts.next(), left_parts.next()) {
                (Some(profile), Some(key), Some(chain_id)) => (profile, key, Some(chain_id)),
                (Some(profile), Some(key), None) => (profile, key, None),
                _ => {
                    return Err(MescError::OverrideError(format!(
                        "invalid profiles override, bad target: {}",
                        left_side
                    )))
                }
            };

        let profile = profiles
            .entry(profile_name.to_string())
            .or_insert_with(|| Profile::new(profile_name.to_string()));

        match key {
            "default_endpoint" => profile.default_endpoint = Some(endpoint.to_string()),
            "network_defaults" => {
                if let Some(cid) = chain_id {
                    profile.network_defaults.insert(cid.try_into_chain_id()?, endpoint.to_string());
                }
            }
            _ => {
                return Err(MescError::OverrideError(format!(
                    "invalid profile override, bad key: {}",
                    key
                )));
            }
        }
    }

    Ok(Some(profiles))
}

fn get_global_metadata_override() -> Result<Option<HashMap<String, serde_json::Value>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_GLOBAL_METADATA") {
        if raw.is_empty() {
            return Ok(None);
        }
        let parsed: Result<HashMap<String, serde_json::Value>, _> =
            serde_json::from_str(raw.as_str());
        Ok(Some(parsed?))
    } else {
        Ok(None)
    }
}

type Metadata = HashMap<String, serde_json::Value>;

fn get_endpoint_metadata_override() -> Result<Option<HashMap<String, Metadata>>, MescError> {
    if let Ok(raw) = std::env::var("MESC_ENDPOINT_METADATA") {
        if raw.is_empty() {
            return Ok(None);
        }
        let parsed: Result<HashMap<String, HashMap<String, serde_json::Value>>, _> =
            serde_json::from_str(raw.as_str());
        Ok(Some(parsed?))
    } else {
        Ok(None)
    }
}
