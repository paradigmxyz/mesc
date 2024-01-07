from __future__ import annotations
from mesc.types import RpcConfig
from . import network_names


def is_chain_id(chain_id: str) -> bool:
    """return True if input is a valid chain_id"""
    if chain_id.isdecimal():
        return True
    elif chain_id.startswith("0x"):
        try:
            int(chain_id, 16)
            return True
        except ValueError:
            return False
    else:
        return False


def network_name_to_chain_id(
    network_name: str, *, config: RpcConfig | None = None
) -> str | None:
    """return chain_id of given network name"""
    network_name = network_name.lower()
    if config is not None and network_name in config["network_names"]:
        return config["network_names"][network_name]
    else:
        for chain_id, chain_id_name in network_names.network_names.items():
            if network_name == chain_id_name:
                return chain_id
        else:
            return None
