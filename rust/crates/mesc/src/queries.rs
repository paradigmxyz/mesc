use crate::directory;
use crate::load::{load_config_data,load_env_profile};
use crate::types::{Endpoint, ConfigError};

pub fn get_default_endpoint(profile: Option<&str>) -> Result<Option<Endpoint>, ConfigError> {
    match get_default_network(profile)? {
        Some(chain_id) => get_default_network_endpoint(chain_id, profile),
        None => Ok(None),
    }
}

pub fn get_default_network(profile: Option<&str>) -> Result<Option<u64>, ConfigError> {
    let config = load_config_data()?;
    let profile = match profile {
        Some(profile) => Some(profile.to_string()),
        None => load_env_profile(),
    };

    if let Some(profile) = profile {
        if let Some(Some(network)) = config.profiles.get(&profile).map(|p| p.default_network) {
            return Ok(Some(network))
        };
    };

    Ok(config.default_network)
}

pub fn get_default_network_endpoint(
    chain_id: u64,
    profile: Option<&str>,
) -> Result<Option<Endpoint>, ConfigError> {
    let config = load_config_data()?;
    let profile = match profile {
        Some(profile) => Some(profile.to_string()),
        None => load_env_profile(),
    };

    if let Some(profile) = profile {
        let name = config
            .profiles
            .get(&profile)
            .and_then(|p| p.default_network_endpoints.get(&chain_id.to_string()));
        if let Some(name) = name {
            return get_endpoint_by_name(name).map(Some)
        }
    };

    match config.default_network_endpoints.get(&chain_id.to_string()) {
        Some(name) => get_endpoint_by_name(name).map(Some),
        None => Ok(None),
    }
}

pub fn get_endpoint_by_name(name: &str) -> Result<Endpoint, ConfigError> {
    let config = load_config_data()?;
    if let Some(endpoint) = config.endpoints.get(name) {
        Ok(endpoint.clone())
    } else {
        Err(ConfigError::MissingEndpoint)
    }
}


pub fn parse_user_query(query: &str, profile: Option<&str>) -> Result<Option<Endpoint>, ConfigError> {
    let mut config = load_config_data()?;

    // by endpoint name
    if let Some(endpoint) = config.endpoints.remove(query) {
        return Ok(Some(endpoint))
    }

    // by chain_id
    if let Ok(chain_id) = query.parse::<u64>() {
        if let Ok(Some(endpoint)) = get_default_network_endpoint(chain_id, profile) {
            return Ok(Some(endpoint))
        }
    }

    // by network name
    if let Some(chain_id) = config.network_names.get(query) {
        if let Ok(Some(endpoint)) = get_default_network_endpoint(*chain_id, profile) {
            return Ok(Some(endpoint))
        }
    }
    if let Some(chain_id) = directory::get_network_chain_id(query) {
        if let Ok(Some(endpoint)) = get_default_network_endpoint(chain_id, profile) {
            return Ok(Some(endpoint))
        }
    }

    Ok(None)
}
