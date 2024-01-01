use crate::ChainId;
use std::collections::HashMap;

fn known_networks() -> HashMap<&'static str, ChainId> {
    [
        ("ethereum", 1u32.into()),
        ("goerli", 5u32.into()),
        ("optimism", 10u32.into()),
        ("polygon", 137u32.into()),
        ("arbitrum", 42161u32.into()),
    ]
    .iter()
    .cloned()
    .collect()
}

/// get chain id of given network name
pub(crate) fn get_network_chain_id(network: &str) -> Option<ChainId> {
    known_networks().remove(network)
}

/// get network name of given chain id
pub fn get_network_name(chain_id: &ChainId) -> Option<String> {
    for (name, other_chain_id) in known_networks().iter() {
        if chain_id == other_chain_id {
            return Some(name.to_string())
        }
    }
    None
}
