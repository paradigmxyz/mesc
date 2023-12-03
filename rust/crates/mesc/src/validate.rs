use crate::{RpcConfig, MescError};

pub(crate) fn validate_config(config: &RpcConfig) -> Result<(), MescError> {
    // all referenced endpoints exist
    if let Some(endpoint) = config.default_endpoint.as_ref() {
        if !config.endpoints.contains_key(endpoint.as_str()) {
            return Err(MescError::MissingEndpoint(endpoint.to_string()))
        }
    }
    for endpoint in config.network_defaults.values() {
        if !config.endpoints.contains_key(endpoint) {
            return Err(MescError::MissingEndpoint(endpoint.clone()))
        }
    }
    for profile in config.profiles.values() {
        if let Some(endpoint) = profile.default_endpoint.as_ref() {
            if !config.endpoints.contains_key(endpoint.as_str()) {
                return Err(MescError::MissingEndpoint(endpoint.to_string()))
            }
        }
        for endpoint in profile.network_defaults.values() {
            if !config.endpoints.contains_key(endpoint) {
                return Err(MescError::MissingEndpoint(endpoint.clone()))
            }
        }
    }

    Ok(())
}
