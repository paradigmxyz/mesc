from __future__ import annotations
from typing import Any, MutableMapping, TypedDict, Literal


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
    endpoint_metadata: MutableMapping[str, Any]


class Profile(TypedDict):
    name: str
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


class EndpointQuery(TypedDict):
    query_type: Literal[
        "default_endpoint",
        "endpoint_by_name",
        "endpoint_by_network",
        "user_input_query",
    ]
    fields: (
        DefaultEndpointQuery | EndpointNameQuery | EndpointNetworkQuery | UserInputQuery
    )


class DefaultEndpointQuery(TypedDict):
    profile: str | None


class EndpointNameQuery(TypedDict):
    profile: str | None
    name: str


class EndpointNetworkQuery(TypedDict):
    profile: str | None
    chain_id: str


class UserInputQuery(TypedDict):
    profile: str | None
    user_input: str


class MultiEndpointQuery(TypedDict):
    name_contains: str | None
    url_contains: str | None
    network: str | None
