from __future__ import annotations

known_networks = {
    "ethereum": "1",
    "goerli": "5",
}


def network_name_to_chain_id(network_name: str) -> str | None:
    return known_networks.get(network_name.lower())
