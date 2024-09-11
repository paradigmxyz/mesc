use crate::ChainId;

/// get chain id of given network name
pub(crate) fn get_network_chain_id(network_name: &str) -> Option<ChainId> {
    crate::network_names::NETWORKS
        .iter()
        .find(|(_, name)| *name == network_name)
        .map(|(chain_id, _)| (*chain_id).into())
}

/// get network name of given chain id
pub fn get_network_name(chain_id: &ChainId) -> Option<&'static str> {
    crate::network_names::get_network_names().get(chain_id).copied()
}
