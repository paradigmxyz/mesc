from __future__ import annotations
import json
import os

from .types import RpcConfig, Endpoint


def get_default_network(
    *,
    profile: str | None = None,
    require_profile: bool = False,
) -> int | None:
    config = read_config_data()
    if profile and (require_profile or profile in config['profiles']):
        return config['profiles'][profile]['default_network']
    else:
        return config['default_network']


def get_default_endpoint(
    chain_id: int,
    *,
    profile: str | None = None,
    require_profile: bool = False,
) -> Endpoint:
    config = read_config_data()
    if profile and (require_profile or profile in config['profiles']):
        default_endpoints = config['profiles'][profile]['default_endpoints']
    else:
        default_endpoints = config['default_endpoints']

    name = default_endpoints.get(str(chain_id))
    if name is None:
        raise Exception('missing endpoint for chain_id: ' + str(chain_id))

    return get_endpoint(name, config=config)


def get_endpoint(name: str, *, config: RpcConfig) -> Endpoint:
    for endpoint_name, endpoint in read_config_data()['endpoints'].items():
        if endpoint_name == name:
            return endpoint
    else:
        raise Exception('missing endpoint: ' + str(name))


def read_config_data() -> RpcConfig:
    mode = os.environ.get('RPC_CONFIG_MODE')
    if mode == 'PATH':
        return read_file_config()
    elif mode == 'ENV':
        return read_env_config()
    elif mode not in ['', None]:
        raise Exception('invalid mode: ' + str(mode))
    elif os.environ.get('RPC_CONFIG_PATH') not in ['', None]:
        return read_file_config()
    elif os.environ.get('RPC_CONFIG_ENV') not in ['', None]:
        return read_env_config()
    else:
        raise Exception('config not specified')


def read_env_config() -> RpcConfig:
    return json.loads(os.environ.get('RPC_CONFIG_ENV'))  # type: ignore


def read_file_config() -> RpcConfig:
    with open(os.environ.get('RPC_CONFIG_PATH'), 'r') as f:  # type: ignore
        return json.load(f)  # type: ignore
