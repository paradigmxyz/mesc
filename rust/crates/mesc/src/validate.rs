use crate::{MescError, RpcConfig};
use std::collections::HashSet;

pub(crate) fn validate_config(config: &RpcConfig) -> Result<(), MescError> {
    // referenced endpoints exist
    if let Some(endpoint) = config.default_endpoint.as_ref() {
        if !config.endpoints.contains_key(endpoint.as_str()) {
            return Err(MescError::MissingEndpoint(endpoint.to_string()));
        }
    }
    for endpoint in config.network_defaults.values() {
        if !config.endpoints.contains_key(endpoint) {
            return Err(MescError::MissingEndpoint(endpoint.clone()));
        }
    }
    for profile in config.profiles.values() {
        if let Some(endpoint) = profile.default_endpoint.as_ref() {
            if !config.endpoints.contains_key(endpoint.as_str()) {
                return Err(MescError::MissingEndpoint(endpoint.to_string()));
            }
        }
        for endpoint in profile.network_defaults.values() {
            if !config.endpoints.contains_key(endpoint) {
                return Err(MescError::MissingEndpoint(endpoint.clone()));
            }
        }
    }

    // default endpoints of each network actually use that specified network
    for (chain_id, endpoint_name) in config.network_defaults.iter() {
        if let Some(endpoint) = config.endpoints.get(endpoint_name) {
            if Some(chain_id) != endpoint.chain_id.as_ref() {
                return Err(MescError::IntegrityError(format!(
                    "endpoint {} chain_id does not match default chain_id",
                    endpoint_name
                )));
            }
        }
    }
    for profile in config.profiles.values() {
        for (chain_id, endpoint_name) in profile.network_defaults.iter() {
            if let Some(endpoint) = config.endpoints.get(endpoint_name) {
                if Some(chain_id) != endpoint.chain_id.as_ref() {
                    return Err(MescError::IntegrityError(format!(
                        "endpoint {} chain_id does not match default chain_id",
                        endpoint_name
                    )));
                }
            }
        }
    }

    // endpoint map keys match endpoint name fields
    for (name, endpoint) in config.endpoints.iter() {
        if name != endpoint.name.as_str() {
            return Err(MescError::IntegrityError(format!(
                "map key does not match name field for endpoint, {} != {}",
                name, endpoint.name
            )));
        }
    }

    // profile map keys match profile name fields
    for (name, profile) in config.profiles.iter() {
        if name != profile.name.as_str() {
            return Err(MescError::IntegrityError(format!(
                "map key does not match name field for profile, {} != {}",
                name, profile.name
            )));
        }
    }

    // chain_id's are valid
    for network in config.network_defaults.keys() {
        network.to_hex()?;
    }
    for profile in config.profiles.values() {
        for network in profile.network_defaults.keys() {
            network.to_hex()?;
        }
    }
    for endpoint in config.endpoints.values() {
        if let Some(chain_id) = endpoint.chain_id.as_ref() {
            chain_id.to_hex()?;
        }
    }

    // no duplicate default network entries using decimal vs hex
    let hex_networks: Result<HashSet<_>, MescError> =
        config.network_defaults.keys().map(|c| c.to_hex()).collect();
    if hex_networks?.len() != config.network_defaults.len() {
        return Err(MescError::IntegrityError(
            "colliding network defaults using decimal vs hex".to_string(),
        ));
    }

    Ok(())
}
