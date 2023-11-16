use std::collections::HashMap;

fn known_networks() -> HashMap<&'static str, u64> {
    let items = [("ethereum", 1), ("goerli", 5)];
    items.iter().cloned().collect()
}


pub(crate) fn get_network_chain_id(network: &str) -> Option<u64> {
    known_networks().get(network).map(|&chain_id| chain_id)
}

