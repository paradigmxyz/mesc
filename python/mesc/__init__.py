"""MESC is the Multiple Endpoint Shared Configuration Standard"""

from .interface import (
    is_mesc_enabled,
    get_default_endpoint,
    get_endpoint_by_name,
    get_endpoint_by_network,
    get_endpoint_by_query,
    find_endpoints,
    get_global_metadata,
)
from .network_utils import (
    get_network_name,
)

import typing

if typing.TYPE_CHECKING:
    from .types import Endpoint, Profile, RpcConfig

    __all__ = (
        'Endpoint',
        'Profile',
        'RpcConfig',
        'is_mesc_enabled',
        'get_default_endpoint',
        'get_endpoint_by_name',
        'get_endpoint_by_network',
        'get_endpoint_by_query',
        'find_endpoints',
        'get_global_metadata',
        'get_network_name',
    )

else:
    __all__ = (
        'is_mesc_enabled',
        'get_default_endpoint',
        'get_endpoint_by_name',
        'get_endpoint_by_network',
        'get_endpoint_by_query',
        'find_endpoints',
        'get_global_metadata',
        'get_network_name',
    )

__version__ = '0.2.0'
