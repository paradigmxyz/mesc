use super::rpc;
use crate::MescCliError;
use mesc::{ChainId, TryIntoChainId};
use std::{collections::HashMap, net::ToSocketAddrs, time::Instant};

pub(crate) struct EndpointNetworkInfo {
    pub(crate) url: String,
    pub(crate) node_client_version: Option<String>,
    pub(crate) current_block: Option<u32>,
    pub(crate) ip_address: Option<String>,
    pub(crate) latency: Option<f64>,
    pub(crate) namespaces: Option<Vec<String>>,
    pub(crate) location: Option<String>,
}

pub(crate) async fn get_node_network_info(
    url: String,
    fields: &[String],
    timeout: u64,
) -> Result<EndpointNetworkInfo, MescCliError> {
    let client =
        reqwest::Client::builder().timeout(std::time::Duration::from_secs(timeout)).build()?;

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

    Ok(EndpointNetworkInfo {
        url,
        node_client_version,
        current_block,
        ip_address,
        latency,
        namespaces,
        location,
    })
}

pub(crate) fn is_ip(url: &str) -> bool {
    use std::net::{Ipv4Addr, Ipv6Addr};
    if url.is_empty() {
        return false;
    }
    let domain = match url.split_once('/') {
        Some((domain, _)) => domain,
        None => url,
    };
    let domain = domain.split(':').next().unwrap_or("");
    domain.parse::<Ipv4Addr>().is_ok() || domain.parse::<Ipv6Addr>().is_ok()
}

fn get_ip_address(url: &str) -> Result<String, MescCliError> {
    let parsed_url = url::Url::parse(url)
        .map_err(|_| MescCliError::UrlError("could not parse url".to_string()))?;
    let host =
        parsed_url.host_str().ok_or(MescCliError::UrlError("could not parse host".to_string()))?;
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

pub(crate) async fn fetch_network_list() -> Result<HashMap<ChainId, String>, MescCliError> {
    let url = "https://chainid.network/chains.json";
    let response = reqwest::get(url).await?;

    // Check if the response is success and parse the JSON body
    if response.status().is_success() {
        let data: Vec<serde_json::Value> = response.json().await?;

        // Create a HashMap to store the chainId and name pairs
        let mut result_map: HashMap<ChainId, String> = HashMap::new();

        // Iterate through the array of data
        for datum in data {
            if let (Some(chain_id), Some(name)) =
                (datum["chainId"].as_u64(), datum["name"].as_str())
            {
                // Insert the pair into the HashMap
                result_map.insert(chain_id.try_into_chain_id()?, name.to_string());
            }
        }

        Ok(result_map)
    } else {
        Err(MescCliError::InvalidNetworkResponse)
    }
}
