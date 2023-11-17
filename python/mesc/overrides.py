from __future__ import annotations
import copy
import json
import os
from typing import Any, Mapping

from . import directory, exceptions
from .types import RpcConfig, Endpoint, Profile


def apply_env_overrides(config: RpcConfig | None) -> RpcConfig:
    if config is not None:
        config = copy.deepcopy(config)
    else:
        config = {
            "mesc_version": "1.0",
            "default_endpoint": None,
            "network_defaults": {},
            "network_names": {},
            "endpoints": {},
            "profiles": {},
            "global_metadata": {},
        }

    config["endpoints"].update(env_endpoints())
    config["network_names"].update(env_network_names())
    config["network_defaults"].update(env_network_defaults())
    config["profiles"].update(env_profiles())
    default_endpoint = env_default_endpoint()
    if default_endpoint is not None:
        config["default_endpoint"] = default_endpoint

    # metadata
    config["global_metadata"].update(env_global_metadata())
    for endpoint, metadata in env_endpoint_metadata.items():
        config["endpoints"][endpoint]["endpoint_metadata"].update(metadata)

    return config


def env_default_endpoint(config: RpcConfig) -> str | None:
    default_endpoint = os.environ.get("MESC_DEFAULT_ENDPOINT")
    if default_endpoint in config['endpoints']:
        return default_endpoint
    elif default_endpoint.is_decimal():
        return _chain_id_to_endpoint_name(int(default_endpoint), config)
    elif default_endpoint in config['network_names']:
        chain_id = config['network_names'][default_endpoint]
        return _chain_id_to_endpoint_name(chain_id, config)
    elif directory.network_name_to_chain_id(default_endpoint) is not None:
        chain_id = directory.network_name_to_chain_id(default_endpoint)
        return _chain_id_to_endpoint_name(chain_id, config)
    else:
        return exceptions.InvalidOverride('Invalid value used for MESC_DEFAULT_ENDPOINT')


def _chain_id_to_endpoint_name(chain_id: int, config: RpcConfig) -> str:
    endpoint = config['network_defaults'].get(chain_id)
    if endpoint is None:
        raise exceptions.MissingEndpoint("no endpoint for given default network: " + str(chain_id))
    else:
        return endpoint


def env_network_defaults() -> Mapping[str, str]:
    network_defaults = os.environ.get("MESC_NETWORK_DEFAULTS")
    items = [item.split('=') for item in network_defaults.split(' ')]
    return {name: int(chain_id) for name, chain_id in items}


def env_network_names() -> Mapping[str, int]:
    network_names = os.environ.get("MESC_NETWORK_NAMES")
    if network_names.startswith("{"):
        return json.loads(network_names)
    else:
        pairs = [item.split("=") for item in network_names.split(" ")]
        return {key: int(value) for key, value in pairs}


def env_endpoints() -> Mapping[str, Endpoint]:
    endpoints = os.environ.get("MESC_ENDPOINTS")


def env_profiles() -> Mapping[str, Profile]:
    raw_profiles = os.environ.get("MESC_PROFILES")
    if raw_profiles is None:
        return {}

    profiles = {}
    for item in profiles.split(" "):
        key, value = item.split("=")
        subkeys = key.split(".")
        profiles.setdefault(
            subkeys[0],
            {"default_endpoint": None, "network_defaults": None},
        )
        if len(subkeys) == 2 and subkeys[1] == "default_endpoint":
            profiles[subkeys[0]]["default_endpoint"] = value
        elif len(subkeys) == 3 and subkeys[1] == "network_defaults":
            name, _, network = subkeys
            profiles[name]["network_defaults"][network] = value
        else:
            raise Exception("invalid value for MESC_PROFILES")
    return profiles


def env_global_metadata() -> Mapping[str, Any]:
    global_metadata = os.environ.get("MESC_GLOBAL_METADATA")
    if global_metadata is not None:
        return json.loads(global_metadata)
    else:
        return {}


def env_endpoint_metadata() -> Mapping[str, Mapping[str, Any]]:
    endpoint_metadata = os.environ.get("MESC_ENDPOINT_METADATA")
    if endpoint_metadata is not None:
        return json.loads(endpoint_metadata)
    else:
        return {}
