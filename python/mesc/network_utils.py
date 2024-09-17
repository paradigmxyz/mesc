from __future__ import annotations

import typing
from . import network_names

if typing.TYPE_CHECKING:
    from .types import RpcConfig


def is_chain_id(chain_id: str) -> bool:
    """return True if input is a valid chain_id"""
    if chain_id.isdecimal():
        return True
    elif chain_id.startswith('0x'):
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
    if config is not None and network_name in config['network_names']:
        return config['network_names'][network_name]
    else:
        for chain_id, chain_id_name in network_names.network_names.items():
            if network_name == chain_id_name:
                return chain_id
        else:
            return None


def chain_id_to_standard_hex(chain_id: str) -> str | None:
    if chain_id.startswith('0x'):
        if len(chain_id) > 2:
            as_hex = chain_id
    else:
        try:
            as_hex = hex(int(chain_id))
        except ValueError:
            return None

    return '0x' + as_hex[2:].lstrip('0')


T = typing.TypeVar('T')


def get_by_chain_id(mapping: typing.Mapping[str, T], chain_id: str) -> T | None:
    if chain_id in mapping:
        return mapping[chain_id]

    standard_mapping = {chain_id_to_standard_hex(k): v for k, v in mapping.items()}
    return standard_mapping.get(chain_id_to_standard_hex(chain_id))


def chain_ids_equal(lhs: str, rhs: str) -> bool:
    return chain_id_to_standard_hex(lhs) == chain_id_to_standard_hex(rhs)


def get_network_name(network: int | str) -> str | None:
    if isinstance(network, str):
        # 1. check if a known name
        if network in network_names.network_names.values():
            for k, v in network_names.network_names.items():
                if v == network:
                    return v

        # 2. check if a hex number
        try:
            if network.startswith('0x'):
                as_int = int(network, 16)
            else:
                as_int = int(network)
            as_int_str = str(as_int)
            if as_int_str in network_names.network_names:
                return network_names.network_names[as_int_str]
        except ValueError:
            pass

        # 3. return as a name
        return network

    elif isinstance(network, int):
        return network_names.network_names.get(str(network))

    else:
        raise Exception('invalid format for network')
