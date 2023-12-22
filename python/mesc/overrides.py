from __future__ import annotations
import copy
import json
import os
import re
from typing import Any, Mapping, MutableMapping

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

    for name, endpoint in env_endpoints().items():
        if name in config["endpoints"]:
            if endpoint["url"] is not None:
                config["endpoints"][name]["url"] = endpoint["url"]
            if endpoint["chain_id"] is not None:
                config["endpoints"][name]["chain_id"] = endpoint["chain_id"]
            for key, value in endpoint["endpoint_metadata"].items():
                config["endpoints"][name]["endpoint_metadata"][key] = value
        else:
            config["endpoints"][name] = endpoint
    config["network_names"].update(env_network_names())
    for chain_id, endpoint_name in env_network_defaults().items():
        if endpoint_name == "":
            if chain_id in config["network_defaults"]:
                del config["network_defaults"][chain_id]
        else:
            config["network_defaults"][chain_id] = endpoint_name
    config["profiles"].update(env_profiles())
    default_endpoint = env_default_endpoint(config)
    if default_endpoint is not None:
        config["default_endpoint"] = default_endpoint

    # metadata
    config["global_metadata"].update(env_global_metadata())
    for endpoint_name, metadata in env_endpoint_metadata().items():
        config["endpoints"][endpoint_name]["endpoint_metadata"].update(metadata)

    return config


def env_default_endpoint(config: RpcConfig) -> str | None:
    default_endpoint = os.environ.get("MESC_DEFAULT_ENDPOINT")
    if default_endpoint is None or default_endpoint == "":
        return None

    if default_endpoint in config["endpoints"]:
        return default_endpoint
    elif default_endpoint.isdecimal():
        return _chain_id_to_endpoint_name(default_endpoint, config)
    elif default_endpoint in config["network_names"]:
        chain_id = config["network_names"][default_endpoint]
        return _chain_id_to_endpoint_name(chain_id, config)

    dir_chain_id = directory.network_name_to_chain_id(default_endpoint, config=config)
    if dir_chain_id is not None:
        return _chain_id_to_endpoint_name(dir_chain_id, config)
    else:
        raise exceptions.InvalidOverride(
            "Invalid syntax used for MESC_DEFAULT_ENDPOINT"
        )


def _chain_id_to_endpoint_name(chain_id: str, config: RpcConfig) -> str:
    endpoint = config["network_defaults"].get(chain_id)
    if endpoint is None:
        raise exceptions.MissingEndpoint(
            "no endpoint for given default network: " + str(chain_id)
        )
    else:
        return endpoint


def env_network_defaults() -> Mapping[str, str]:
    network_defaults = os.environ.get("MESC_NETWORK_DEFAULTS")
    if network_defaults is None or network_defaults == "":
        return {}
    else:
        items = [item.split("=", 1) for item in network_defaults.split(" ")]
        return {str(network): endpoint for network, endpoint in items}


def env_network_names() -> Mapping[str, str]:
    network_names = os.environ.get("MESC_NETWORK_NAMES")
    if network_names is None or network_names == "":
        return {}

    if network_names.startswith("{"):
        return json.loads(network_names)  # type: ignore
    else:
        pairs = [item.split("=") for item in network_names.split(" ")]
        return {key: value for key, value in pairs}


def env_endpoints() -> Mapping[str, Endpoint]:
    endpoints: MutableMapping[str, Endpoint] = {}

    # get override
    raw_endpoints = os.environ.get("MESC_ENDPOINTS")
    if raw_endpoints is None or raw_endpoints == "":
        return endpoints

    # gather explicit endpoints
    pattern = r"^(?:(?P<name>[A-Za-z_-]+)(?::(?P<chain_id>\w+))?=\s*)?(?P<url>\S+)$"
    for item in raw_endpoints.split(" "):
        match = re.match(pattern, item)
        if match:
            chain_id = match.group("chain_id")
            url = match.group("url")
            name = match.group("name")
            if name is None:
                from urllib.parse import urlparse

                if not urlparse(url).scheme:
                    name = urlparse("http://" + url).hostname
                else:
                    name = urlparse(url).hostname
                if name is None:
                    raise Exception("could create name for endpoint")
                name = ".".join(name.split(".")[:-1])

            endpoints[name] = {
                "name": name,
                "url": url,
                "chain_id": chain_id,
                "endpoint_metadata": {},
            }
        else:
            raise exceptions.InvalidOverride("Invalid syntax used for MESC_ENDPOINTS")

    # gather ad hoc endpoints
    # ad_hoc_endpoints = _collect_ad_hoc_endpoints()

    return endpoints


# def _collect_ad_hoc_endpoints(endpoints: Mapping[str, Endpoint]) -> Mapping[str, Endpoint]:
#     # look in MESC_DEFAULT_ENDPOINT, MESC_NETWORK_DEFAULTS, MESC_PROFILES
#     raw_endpoints = []

#     default_endpoint = os.environ.get("MESC_DEFAULT_ENDPOINT")
#     raw_endpoints.append(default_endpoint)
#     network_defaults = env_network_defaults(replace=False)
#     for network, endpoint in network_defaults.items():
#         raw_endpoints.append(endpoint)
#     profiles = env_profiles(replace=False)
#     for name, profile in profiles.items():
#         profile.get

# # process raw endpoints
# for endpoint in raw_endpoints:
#     if _is_url(endpoint):
#         pass


def env_profiles() -> Mapping[str, Profile]:
    raw_profiles = os.environ.get("MESC_PROFILES")
    if raw_profiles is None or raw_profiles == "":
        return {}

    profiles: MutableMapping[str, Profile] = {}
    for item in raw_profiles.split(" "):
        key, value = item.split("=")
        subkeys = key.split(".")
        profiles.setdefault(
            subkeys[0],
            {"name": subkeys[0], "default_endpoint": None, "network_defaults": {}},
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
    if global_metadata is None or global_metadata == "":
        return {}
    else:
        return json.loads(global_metadata)  # type: ignore


def env_endpoint_metadata() -> Mapping[str, Mapping[str, Any]]:
    endpoint_metadata = os.environ.get("MESC_ENDPOINT_METADATA")
    if endpoint_metadata is None or endpoint_metadata == "":
        return {}
    else:
        return json.loads(endpoint_metadata)  # type: ignore
