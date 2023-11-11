from __future__ import annotations
from typing import Any, Mapping, TypedDict


class Endpoint(TypedDict):
    url: str
    chain_id: int
    endpoint_extras: Mapping[str, Any]


class Profile(TypedDict):
    default_network: int | None
    default_endpoints: Mapping[str, str]


class RpcConfig(TypedDict):
    schema: str
    default_network: int | None
    default_endpoints: Mapping[str, str]
    endpoints: Mapping[str, Endpoint]
    profiles: Mapping[str, Profile]
    global_extras: Mapping[str, Any]
