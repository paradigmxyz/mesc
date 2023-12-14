use super::rpc;
use crate::MescCliError;
use std::net::ToSocketAddrs;
use std::time::Instant;

pub(crate) struct EndpointMetadata {
    pub url: String,
    pub node_client_version: Option<String>,
    pub current_block: Option<u32>,
    pub ip_address: Option<String>,
    pub latency: Option<f64>,
    pub namespaces: Option<Vec<String>>,
    pub location: Option<String>,
}

pub(crate) async fn get_node_metadata(
    url: String,
    fields: &[String],
    timeout: u64,
) -> Result<EndpointMetadata, MescCliError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout))
        .build()?;

    // node client
    let node_client_version = if fields.contains(&"client".to_string()) {
        Some(rpc::request_node_client_version(client.clone(), url.clone()).await?)
    } else {
        None
    };

    // latest block and latency
    let (current_block, latency) =
        if fields.contains(&"block".to_string()) | fields.contains(&"latency".to_string()) {
            let start = Instant::now();
            let current_block = rpc::request_block_number(client.clone(), url.clone()).await?;
            let duration = start.elapsed();
            let response_latency =
                (duration.as_secs() as f64) + ((duration.subsec_nanos() as f64) / 1_000_000_000.0);
            (Some(current_block), Some(response_latency))
        } else {
            (None, None)
        };

    // ip address and location
    let (ip_address, location) =
        if fields.contains(&"ip".to_string()) | fields.contains(&"location".to_string()) {
            match get_ip_address(url.as_str()) {
                Ok(ip) => {
                    let service = ipgeolocate::Service::IpApi;
                    let location = match ipgeolocate::Locator::get(ip.as_str(), service).await {
                        Ok(ip) => format!("{}, {}, {}", ip.city, ip.region, ip.country),
                        Err(_) => "-".to_string(),
                    };
                    (Some(ip), Some(location))
                }
                Err(_) => (None, None),
            }
        } else {
            (None, None)
        };

    // namespaces
    let namespaces = None;

    Ok(EndpointMetadata {
        url,
        node_client_version,
        current_block,
        ip_address,
        latency,
        namespaces,
        location,
    })
}

fn get_ip_address(url: &str) -> Result<String, MescCliError> {
    let parsed_url = url::Url::parse(url)
        .map_err(|_| MescCliError::UrlError("could not parse url".to_string()))?;
    let host = parsed_url
        .host_str()
        .ok_or(MescCliError::UrlError("could not parse host".to_string()))?;
    let port = parsed_url
        .port_or_known_default()
        .ok_or(MescCliError::UrlError("could not parse port".to_string()))?;
    let addr_str = format!("{}:{}", host, port);

    match addr_str.to_socket_addrs() {
        Ok(mut addresses) => {
            if let Some(address) = addresses.next() {
                Ok(address.ip().to_string())
            } else {
                Err(MescCliError::InvalidNetworkResponse)
            }
        }
        Err(_) => Err(MescCliError::InvalidNetworkResponse),
    }
}

fn is_using_trace_namespace() -> Result<bool, MescCliError> {
    todo!()
}

fn is_using_debug_namespace() -> Result<bool, MescCliError> {
    todo!()
}
