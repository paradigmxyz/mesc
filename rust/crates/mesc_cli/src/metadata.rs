use super::rpc;
use crate::MescCliError;
use std::net::ToSocketAddrs;

pub(crate) struct EndpointMetadata {
    url: String,
    node_client_version: Option<String>,
    current_block: Option<u32>,
    ip_address: Option<String>,
    response_latency: Option<f64>,
    namespaces: Option<Vec<String>>,
    geolocation: Option<String>,
}

pub(crate) async fn get_node_metadata(url: String) -> Result<EndpointMetadata, MescCliError> {
    let client = reqwest::Client::new();

    let node_client_version = rpc::request_node_client_version(client.clone(), url.clone()).await?;
    let current_block = rpc::request_block_number(client.clone(), url.clone()).await?;
    let ip_address = get_ip_address(url.as_str())?;
    let namespaces = None;
    let response_latency = None;
    let geolocation = None;

    Ok(EndpointMetadata {
        url,
        node_client_version: Some(node_client_version),
        current_block: Some(current_block),
        ip_address: Some(ip_address),
        response_latency,
        namespaces,
        geolocation,
    })
}

fn get_ip_address(url: &str) -> Result<String, MescCliError> {
    if let Ok(mut addresses) = url.to_socket_addrs() {
        if let Some(address) = addresses.next() {
            return Ok(address.ip().to_string());
        }
    }
    Err(MescCliError::InvalidNetworkResponse)
}

fn is_using_trace_namespace() -> Result<bool, MescCliError> {
    todo!()
}

fn is_using_debug_namespace() -> Result<bool, MescCliError> {
    todo!()
}
