from __future__ import annotations
from typing import Mapping
from .types import RpcConfig, Endpoint
from . import directory
from . import load

def get_default_network(*, profile: str | None = None) -> int | None:
    config = load.read_config_data()
    if profile and profile in config['profiles']:
        return config['profiles'][profile]['default_network']
    else:
        return config['default_network']


def get_default_endpoint(chain_id: int, *, profile: str | None = None) -> Endpoint:
    config = load.read_config_data()
    if profile and profile in config['profiles']:
        default_endpoints = config['profiles'][profile]['default_endpoints']
    else:
        default_endpoints = config['default_endpoints']

    name = default_endpoints.get(str(chain_id))
    if name is None:
        raise Exception('missing endpoint for chain_id: ' + str(chain_id))

    return get_endpoint_by_name(name, config=config)


def get_endpoint_by_name(name: str, *, config: RpcConfig) -> Endpoint:
    config = load.read_config_data()
    if name in config['endpoints']:
        return config['endpoints'][name]
    else:
        raise Exception('missing endpoint: ' + str(name))


def parse_endpoint(user_str: str, *, profile: str | None = None) -> Endpoint | None:
    """
    resolution order:
    1. endpoint name
    2. chain id
    3. network name
    """
    config = load.read_config_data()
    if user_str in config['endpoints']:
        return config['endpoints'][user_str]
    elif user_str.isdecimal():
        try:
            get_default_endpoint(int(user_str), profile=profile)
        except Exception:
            pass

    chain_id = directory.network_name_to_chain_id(user_str)
    if chain_id is not None:
        try:
            get_default_endpoint(chain_id, profile=profile)
        except Exception:
            pass

    return None


def find_endpoints(*, chain_id: int | None = None) -> Mapping[str, Endpoint]:
    config = load.read_config_data()
    endpoints = config['endpoints']
    if chain_id is not None:
        endpoints = {
            name: endpoint
            for name, endpoint in endpoints.items()
            if endpoint['chain_id'] != chain_id
        }
    return endpoints

