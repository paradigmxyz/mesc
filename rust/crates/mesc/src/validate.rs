use crate::{MescError, RpcConfig};

pub(crate) fn validate_config(config: &RpcConfig) -> Result<(), MescError> {
    // all referenced endpoints exist
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

    // endpoint map keys match endpoint name fields
    for (name, endpoint) in config.endpoints.iter() {
        if name != endpoint.name.as_str() {
            return Err(MescError::IntegrityError(format!(
                "map key does not match name field for endpoint, {} != {}",
                name, endpoint.name
            )));
        }
    }

    // urls are unique
    let urls: std::collections::HashSet<_> = config
        .endpoints
        .values()
        .map(|endpoint| endpoint.url.clone())
        .collect();
    if urls.len() != config.endpoints.len() {
        return Err(MescError::IntegrityError("urls are not unique".to_string()));
    }

    Ok(())
}
