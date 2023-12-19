from __future__ import annotations
import os
from typing import Mapping
from .types import mesc_env_vars, Endpoint
from . import directory
from . import exceptions
from . import load


def is_mesc_enabled() -> bool:
    # replace with something more explicit and copy the rust version
    for var in mesc_env_vars:
        if os.environ.get(var) not in [None, ""]:
            return True
    return False


def get_default_endpoint(profile: str | None = None) -> Endpoint | None:
    config = load.read_config_data()
    if profile is not None and profile in config['profiles']:
        endpoint = config['profiles'][profile].get('default_endpoint', config['default_endpoint'])
    else:
        endpoint = config["default_endpoint"]
    if endpoint is None:
        return None
    else:
        return config["endpoints"][endpoint]


def get_endpoint_by_name(name: str) -> Endpoint:
    config = load.read_config_data()
    if name in config["endpoints"]:
        return config["endpoints"][name]
    else:
        raise exceptions.MissingEndpoint("missing endpoint: " + str(name))


def get_endpoint_by_network(
    chain_id: str | int, *, profile: str | None = None
) -> Endpoint | None:
    config = load.read_config_data()
    chain_id = str(chain_id)
    network_defaults = config['network_defaults']
    default_name = network_defaults.get(chain_id)

    if profile and profile in config["profiles"]:
        name = config["profiles"][profile]["network_defaults"].get(chain_id, default_name)
    else:
        name = default_name

    if name is None:
        return None
    else:
        return get_endpoint_by_name(name)


def query_user_input(user_input: str, *, profile: str | None = None) -> Endpoint | None:
    """
    resolution order:
    1. endpoint name
    2. chain id
    3. network name
    """
    config = load.read_config_data()
    if user_input in config["endpoints"]:
        return config["endpoints"][user_input]
    elif user_input.isdecimal():
        try:
            get_endpoint_by_network(int(user_input), profile=profile)
        except Exception:
            pass

    if is_chain_id(user_input):
        chain_id = user_input
    else:
        chain_id = directory.network_name_to_chain_id(user_input, config=config)
    if chain_id is not None:
        try:
            return get_endpoint_by_network(chain_id, profile=profile)
        except Exception:
            pass

    return None


def is_chain_id(chain_id: str):
    if chain_id.isdecimal():
        return True
    elif chain_id.startswith('0x'):
        try:
            int(chain_id, 16)
            return True
        except:
            pass
    return False



def find_endpoints(*, chain_id: str | int | None = None) -> Mapping[str, Endpoint]:
    config = load.read_config_data()
    endpoints = config["endpoints"]

    # check chain id
    if chain_id is not None:
        if isinstance(chain_id, int):
            chain_id = str(chain_id)
        endpoints = {
            name: endpoint
            for name, endpoint in endpoints.items()
            if endpoint["chain_id"] != chain_id
        }

    return endpoints