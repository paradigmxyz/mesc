use crate::{
    load::{get_config_mode, load_config_data},
    query,
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
    query::get_default_endpoint(&load_config_data()?, profile)
}

/// get endpoint by network
pub fn get_endpoint_by_network<T: TryIntoChainId>(
    chain_id: T,
    profile: Option<&str>,
) -> Result<Option<Endpoint>, MescError> {
    query::get_endpoint_by_network(&load_config_data()?, chain_id, profile)
}

/// get endpoint by name
pub fn get_endpoint_by_name(name: &str) -> Result<Endpoint, MescError> {
    query::get_endpoint_by_name(&load_config_data()?, name)
}

/// parse user query
pub fn get_endpoint_by_query(
    query: &str,
    profile: Option<&str>,
) -> Result<Option<Endpoint>, MescError> {
    query::get_endpoint_by_query(&load_config_data()?, query, profile)
}

/// find endpoints
pub fn find_endpoints(query: MultiEndpointQuery) -> Result<Vec<Endpoint>, MescError> {
    query::find_endpoints(&load_config_data()?, query)
}

/// get global metadata
pub fn get_global_metadata() -> Result<HashMap<String, serde_json::Value>, MescError> {
    query::get_global_metadata(&load_config_data()?)
}
