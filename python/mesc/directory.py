from __future__ import annotations
from mesc.types import RpcConfig

known_networks = {
    "ethereum": "1",
    "goerli": "5",
    "optimism": "10",
    "polygon": "137",
    "holesky": "17000",
    "arbitrum": "42161",
}


def network_name_to_chain_id(
    network_name: str, *, config: RpcConfig | None = None
) -> str | None:
    network_name = network_name.lower()
    if config is not None and network_name in config["network_names"]:
        return config["network_names"][network_name]
    else:
        return known_networks.get(network_name.lower())
