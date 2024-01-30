from __future__ import annotations

import os
import typing

from .types import mesc_env_vars
from . import network_utils
from . import load

if typing.TYPE_CHECKING:
    from typing_extensions import Any, Mapping, Sequence
    from .types import Endpoint, RpcConfig


def is_mesc_enabled() -> bool:
    """MESC is enabled if two criteria are met:
    1) MESC_MODE != "DISABLED"
    2) at least 1 MESC env var is set
    """
    if os.environ.get('MESC_MODE') == 'DISABLED':
        return False
    for var in mesc_env_vars:
        if os.environ.get(var) not in [None, '']:
            return True
    return False


def get_default_endpoint(
    profile: str | None = None, *, config: RpcConfig | None = None
) -> Endpoint | None:
    """get default MESC endpoint"""
    if config is None:
        config = load.read_config_data()

    # get endpoint name
    if profile is not None and profile in config['profiles']:
        if not config['profiles'][profile]['use_mesc']:
            return None
        endpoint = config['profiles'][profile].get(
            'default_endpoint', config['default_endpoint']
        )
    else:
        endpoint = config['default_endpoint']

    # get endpoint
    if endpoint is None:
        return None
    else:
        return get_endpoint_by_name(endpoint, config=config)


def get_endpoint_by_name(
    name: str, *, config: RpcConfig | None = None
) -> Endpoint | None:
    """get MESC endpoint by name"""
    if config is None:
        config = load.read_config_data()
    if not isinstance(name, str):
        raise Exception('invalid type for name query, it must be a str')
    return config['endpoints'].get(name)


def get_endpoint_by_network(
    chain_id: str | int,
    *,
    profile: str | None = None,
    config: RpcConfig | None = None,
) -> Endpoint | None:
    """get default MESC endpoint for network"""
    if config is None:
        config = load.read_config_data()

    # get global default for network
    if chain_id is None:
        raise ValueError('chain_id must be a str or int')
    chain_id = str(chain_id)
    network_defaults = config['network_defaults']
    default_name = network_utils.get_by_chain_id(network_defaults, chain_id)

    # get profile default for network
    if profile is not None and profile in config['profiles']:
        if not config['profiles'][profile]['use_mesc']:
            return None
        name = network_utils.get_by_chain_id(
            config['profiles'][profile]['network_defaults'],
            chain_id,
        )
        if name is None:
            name = default_name
    else:
        name = default_name

    # get endpoint
    if name is None:
        return None
    else:
        return get_endpoint_by_name(name, config=config)


def get_endpoint_by_query(
    user_input: str,
    *,
    profile: str | None = None,
    config: RpcConfig | None = None,
) -> Endpoint | None:
    """get MESC endpoint that satisfies user input query

    resolution order:
    1. endpoint name
    2. chain id
    3. network name
    """
    if config is None:
        config = load.read_config_data()
    if (
        profile is not None
        and profile in config['profiles']
        and not config['profiles'][profile]['use_mesc']
    ):
        return None
    if user_input in config['endpoints']:
        return config['endpoints'][user_input]
    if network_utils.is_chain_id(user_input):
        chain_id: str | None = user_input
    else:
        chain_id = network_utils.network_name_to_chain_id(user_input, config=config)
    if chain_id is not None:
        try:
            return get_endpoint_by_network(chain_id, profile=profile)
        except Exception:
            pass

    return None


def find_endpoints(
    *,
    chain_id: str | int | None = None,
    name_contains: str | None = None,
    url_contains: str | None = None,
    config: RpcConfig | None = None,
) -> Sequence[Endpoint]:
    """find all inputs that match input criteria"""

    if config is None:
        config = load.read_config_data()
    endpoints = list(config['endpoints'].values())

    # check chain_id
    if chain_id is not None:
        if isinstance(chain_id, int):
            chain_id = str(chain_id)
        chain_id = network_utils.chain_id_to_standard_hex(chain_id)
        endpoints = [
            endpoint
            for endpoint in endpoints
            if endpoint['chain_id'] is not None
            and network_utils.chain_id_to_standard_hex(endpoint['chain_id']) == chain_id
        ]

    # check name_contains
    if name_contains is not None:
        endpoints = [
            endpoint for endpoint in endpoints if name_contains in endpoint['name']
        ]

    # check url_contains
    if url_contains is not None:
        endpoints = [
            endpoint for endpoint in endpoints if url_contains in endpoint['url']
        ]

    return endpoints


def get_global_metadata(
    *, profile: str | None = None, config: RpcConfig | None = None
) -> Mapping[str, Any]:
    """return MESC global metadata"""

    if config is None:
        config = load.read_config_data()

    if profile is not None:
        profile_data = config['profiles'].get(profile)
        if profile_data is not None:
            if not profile_data['use_mesc']:
                return {}
            return dict(config['global_metadata'], **profile_data['profile_metadata'])

    return config['global_metadata']
