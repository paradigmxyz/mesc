from __future__ import annotations

from typing_extensions import Any, MutableMapping, TypedDict, Literal, Sequence


mesc_env_vars = [
    'MESC_MODE',
    'MESC_PATH',
    'MESC_ENV',
    'MESC_ENDPOINT_METADATA',
    'MESC_GLOBAL_METADATA',
    'MESC_ENDPOINTS',
    'MESC_NETWORK_NAMES',
    'MESC_DEFAULT_ENDPOINT',
    'MESC_NETWORK_DEFAULTS',
    'MESC_PROFILES',
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
    use_mesc: bool


class RpcConfig(TypedDict):
    mesc_version: str
    default_endpoint: str | None
    endpoints: MutableMapping[str, Endpoint]
    network_defaults: MutableMapping[str, str]
    network_names: MutableMapping[str, str]
    profiles: MutableMapping[str, Profile]
    global_metadata: MutableMapping[str, Any]


endpoint_types: dict[str, type | tuple[type, ...]] = {
    'name': str,
    'url': str,
    'chain_id': (str, type(None)),
    'endpoint_metadata': dict,
}

profile_types: dict[str, type | tuple[type, ...]] = {
    'name': str,
    'default_endpoint': (str, type(None)),
    'network_defaults': dict,
    'use_mesc': bool,
}

rpc_config_types: dict[str, type | tuple[type, ...]] = {
    'mesc_version': str,
    'default_endpoint': (str, type(None)),
    'endpoints': dict,
    'network_defaults': dict,
    'network_names': dict,
    'profiles': dict,
    'global_metadata': dict,
}

#
# # query types
#


class EndpointQuery(TypedDict):
    query_type: Literal[
        'default_endpoint',
        'endpoint_by_name',
        'endpoint_by_network',
        'user_input',
    ]
    fields: (
        DefaultEndpointQuery | EndpointNameQuery | EndpointNetworkQuery | UserInputQuery
    )


class DefaultEndpointQuery(TypedDict):
    profile: str | None


class EndpointNameQuery(TypedDict):
    name: str


class EndpointNetworkQuery(TypedDict):
    profile: str | None
    chain_id: str


class UserInputQuery(TypedDict):
    profile: str | None
    user_input: str


class MultiEndpointQuery(TypedDict, total=False):
    name_contains: str | None
    url_contains: str | None
    chain_id: str | None


class GlobalMetadataQuery(TypedDict, total=False):
    path: Sequence[str] | None


class MescQuery(TypedDict):
    query_type: Literal[
        'default_endpoint',
        'endpoint_by_name',
        'endpoint_by_network',
        'user_input',
        'multi_endpoint',
        'global_metadata',
    ]
    fields: (
        DefaultEndpointQuery
        | EndpointNameQuery
        | EndpointNetworkQuery
        | UserInputQuery
        | MultiEndpointQuery
        | GlobalMetadataQuery
    )
