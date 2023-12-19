"""MESC is the Multiple Endpoint Shared Configuration Standard"""

from .interface import (
    is_mesc_enabled,
    get_default_endpoint,
    get_endpoint_by_name,
    get_endpoint_by_network,
    query_user_input,
    find_endpoints,
)
from .types import Endpoint, Profile, RpcConfig

__version__ = "0.1.0"

__all__ = (
    "Endpoint",
    "Profile",
    "RpcConfig",
    "is_mesc_enabled",
    "get_default_endpoint",
    "get_endpoint_by_name",
    "get_endpoint_by_network",
    "query_user_input",
    "find_endpoints",
)
