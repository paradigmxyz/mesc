from __future__ import annotations
import os
from typing import Any, Mapping, Sequence
from .types import mesc_env_vars, Endpoint, RpcConfig
from . import network_utils
from . import exceptions
from . import load


def is_mesc_enabled() -> bool:
    """MESC is enabled if two criteria are met:
    1) MESC_MODE != "DISABLED"
    2) at least 1 MESC env var is set
    """
    if os.environ.get("MESC_MODE") == "DISABLED":
        return False
    for var in mesc_env_vars:
        if os.environ.get(var) not in [None, ""]:
            return True
    return False


def get_default_endpoint(
    profile: str | None = None, *, config: RpcConfig | None = None
) -> Endpoint | None:
    """get default MESC endpoint"""
    if config is None:
        config = load.read_config_data()

    # get endpoint name
    if profile is not None and profile in config["profiles"]:
        endpoint = config["profiles"][profile].get(
            "default_endpoint", config["default_endpoint"]
        )
    else:
        endpoint = config["default_endpoint"]

    # get endpoint
    if endpoint is None:
        return None
    else:
        return get_endpoint_by_name(endpoint, config=config)


def get_endpoint_by_name(name: str, *, config: RpcConfig | None = None) -> Endpoint:
    """get MESC endpoint by name"""
    if config is None:
        config = load.read_config_data()
    try:
        return config["endpoints"][name]
    except KeyError:
        raise exceptions.MissingEndpoint("missing endpoint: " + str(name))


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
        raise ValueError("chain_id must be a str")
    chain_id = str(chain_id)
    network_defaults = config["network_defaults"]
    default_name = network_defaults.get(chain_id)

    # get profile default for network
    if profile and profile in config["profiles"]:
        name = config["profiles"][profile]["network_defaults"].get(
            chain_id, default_name
        )
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
    if user_input in config["endpoints"]:
        return config["endpoints"][user_input]
    elif user_input.isdecimal():
        try:
            get_endpoint_by_network(int(user_input), profile=profile)
        except Exception:
            pass

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
    endpoints = list(config["endpoints"].values())

    # check chain_id
    if chain_id is not None:
        if isinstance(chain_id, int):
            chain_id = str(chain_id)
        endpoints = [
            endpoint for endpoint in endpoints if endpoint["chain_id"] == chain_id
        ]

    # check name_contains
    if name_contains is not None:
        endpoints = [
            endpoint for endpoint in endpoints if name_contains in endpoint["name"]
        ]

    # check url_contains
    if url_contains is not None:
        endpoints = [
            endpoint for endpoint in endpoints if url_contains in endpoint["url"]
        ]

    return endpoints


def get_global_metadata() -> Mapping[str, Any]:
    """return MESC global metadata"""
    return load.read_config_data()["global_metadata"]
