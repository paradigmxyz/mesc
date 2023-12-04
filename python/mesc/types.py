from __future__ import annotations
from typing import Any, MutableMapping, TypedDict


mesc_env_vars = [
    "MESC_CONFIG_MODE",
    "MESC_CONFIG_PATH",
    "MESC_CONFIG_JSON",
    "MESC_ENDPOINT_METADATA",
    "MESC_GLOBAL_METADATA",
    "MESC_ENDPOINTS",
    "MESC_NETWORK_NAMES",
    "MESC_DEFAULT_ENDPOINT",
    "MESC_NETWORK_DEFAULTS",
    "MESC_PROFILES",
]


class Endpoint(TypedDict):
    name: str
    url: str
    chain_id: str | None
    endpoint_extras: MutableMapping[str, Any]


class Profile(TypedDict):
    default_endpoint: str | None
    network_defaults: MutableMapping[str, str]


class RpcConfig(TypedDict):
    mesc_version: str
    default_endpoint: str | None
    endpoints: MutableMapping[str, Endpoint]
    network_defaults: MutableMapping[str, str]
    network_names: MutableMapping[str, str]
    profiles: MutableMapping[str, Profile]
    global_metadata: MutableMapping[str, Any]
