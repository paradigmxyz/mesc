use crate::MescCliError;
use mesc::{ChainId, TryIntoChainId};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ResponseWeb3ClientVersion {
    jsonrpc: String,
    id: u64,
    result: String,
}

#[derive(Deserialize)]
struct ResponseEthChainId {
    jsonrpc: String,
    id: u64,
    result: String,
}

#[derive(Deserialize)]
struct ResponseEthBlockNumber {
    jsonrpc: String,
    id: u64,
    result: String,
}

pub(crate) async fn request_node_client_version(
    client: reqwest::Client,
    url: String,
) -> Result<String, MescCliError> {
    let data = json!({
        "jsonrpc": "2.0",
        "method": "web3_clientVersion",
        "id": 1,
    });
    let response = client.post(url).json(&data).send().await?;
    let json: ResponseWeb3ClientVersion = response.json().await?;
    Ok(json.result)
}

pub(crate) async fn request_block_number(
    client: reqwest::Client,
    url: String,
) -> Result<u32, MescCliError> {
    let data = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "id": 1,
    });
    let response = client.post(url).json(&data).send().await?;
    let json: ResponseWeb3ClientVersion = response.json().await?;
    match u32::from_str_radix(json.result.get(2..).unwrap_or(""), 16) {
        Ok(value) => Ok(value),
        Err(_) => Err(MescCliError::InvalidNetworkResponse),
    }
}

pub(crate) async fn request_chain_id(
    client: reqwest::Client,
    url: String,
) -> Result<ChainId, MescCliError> {
    let data = json!({
        "jsonrpc": "2.0",
        "method": "eth_chainId",
        "id": 1,
    });
    let response = client.post(url).json(&data).send().await?;
    let json: ResponseWeb3ClientVersion = response.json().await?;

    match u64::from_str_radix(json.result.get(2..).unwrap_or(""), 16) {
        Ok(value) => Ok(value.to_string().try_into_chain_id()?),
        Err(_) => Err(MescCliError::InvalidNetworkResponse),
    }
}
