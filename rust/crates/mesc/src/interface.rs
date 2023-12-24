use crate::{
    directory,
    load::{get_config_mode, load_config_data},
    types::{Endpoint, MescError},
    ConfigMode, MultiEndpointQuery, TryIntoChainId,
};
use std::collections::HashMap;

/// check whether mesc is enabled
pub fn is_mesc_enabled() -> bool {
    matches!(get_config_mode(), Ok(ConfigMode::Path) | Ok(ConfigMode::Env))
}

/// get default endpoint
pub fn get_default_endpoint(profile: Option<&str>) -> Result<Option<Endpoint>, MescError> {
    let config = load_config_data()?;

    // if using a profile, check if that profile has a default endpoint for chain_id
    if let Some(profile) = profile {
        let name = config.profiles.get(profile).and_then(|p| p.default_endpoint.clone());
        if let Some(name) = name {
            return get_endpoint_by_name(name.as_str()).map(Some);
        }
    };

    match config.default_endpoint {
        Some(name) => get_endpoint_by_name(name.as_str()).map(Some),
        None => Ok(None),
    }
}

/// get endpoint by network
pub fn get_endpoint_by_network<T: TryIntoChainId>(
    chain_id: T,
    profile: Option<&str>,
) -> Result<Option<Endpoint>, MescError> {
    let chain_id = chain_id.try_into_chain_id()?;
    let config = load_config_data()?;

    // if using a profile, check if that profile has a default endpoint for chain_id
    if let Some(profile) = profile {
        let name = config.profiles.get(profile).and_then(|p| p.network_defaults.get(&chain_id));
        if let Some(name) = name {
            return get_endpoint_by_name(name).map(Some);
        }
    };

    // check if base configuration has a default endpoint for that chain_id
    match config.network_defaults.get(&chain_id) {
        Some(name) => get_endpoint_by_name(name).map(Some),
        None => Ok(None),
    }
}

/// get endpoint by name
pub fn get_endpoint_by_name(name: &str) -> Result<Endpoint, MescError> {
    let config = load_config_data()?;
    if let Some(endpoint) = config.endpoints.get(name) {
        Ok(endpoint.clone())
    } else {
        Err(MescError::MissingEndpoint(name.to_string()))
    }
}

/// parse user query
pub fn parse_user_query(query: &str, profile: Option<&str>) -> Result<Option<Endpoint>, MescError> {
    let mut config = load_config_data()?;

    // by endpoint name
    if let Some(endpoint) = config.endpoints.remove(query) {
        return Ok(Some(endpoint));
    }

    // by chain_id
    if let Ok(chain_id) = query.try_into_chain_id() {
        if let Ok(Some(endpoint)) = get_endpoint_by_network(chain_id, profile) {
            return Ok(Some(endpoint));
        }
    }

    // by network name
    if let Some(chain_id) = config.network_names.remove(query) {
        return get_endpoint_by_network(chain_id, profile);
    } else if let Some(chain_id) = directory::get_network_chain_id(query) {
        return get_endpoint_by_network(chain_id, profile);
    }

    Ok(None)
}

/// find endpoints
pub fn find_endpoints(query: MultiEndpointQuery) -> Result<Vec<Endpoint>, MescError> {
    let config = load_config_data()?;
    let mut candidates: Vec<Endpoint> = config.endpoints.into_values().collect();

    if let Some(chain_id) = query.chain_id {
        candidates.retain(|endpoint| endpoint.chain_id.as_ref() == Some(&chain_id))
    }

    if let Some(name) = query.name_contains {
        candidates.retain(|endpoint| endpoint.name.contains(&name))
    }

    if let Some(url) = query.url_contains {
        candidates.retain(|endpoint| endpoint.url.contains(&url))
    }

    Ok(candidates)
}

/// get global metadata
pub fn get_global_metadata() -> Result<HashMap<String, serde_json::Value>, MescError> {
    let config = load_config_data()?;
    Ok(config.global_metadata)
}
