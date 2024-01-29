
# Python MESC Implementation

This is a reference implementation of the Python MESC Standard

It has no external dependencies.

- [Installation](#installation)
- [Example Usage](#example-usage)
- [Reference](#reference)

## Installation

### From PyPI
`pip install mesc`

### From Source
```
git clone https://github.com/paradigmxyz/mesc
cd mesc
pip install ./
```

## Example Usage

```python
from typing import Any, Mapping, Sequence
import mesc

# check whether mesc is enabled
enabled: bool = mesc.is_mesc_enabled()

# get the default endpoint
endpoint: Endpoint | None = mesc.get_default_endpoint()

# get the default endpoint of a network
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5)

# get the default endpoint for a particular tool
endpoint: Endpoint | None = mesc.get_default_endpoint(profile='xyz_tool')

# get the default endpoint of a network for a particular tool
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5, profile='xyz_tool')

# get an endpoint by name
endpoint: Endpoint | None = mesc.get_endpoint_by_name('local_goerli')

# parse a user-provided string into a matching endpoint
# (first try 1. endpoint name, then 2. chain id, then 3. network name)
endpoint: Endpoint | None = mesc.get_endpoint_by_query(user_str, profile='xyz_tool')

# find all endpoints matching given criteria
endpoints: Sequence[Endpoint] = mesc.find_endpoints(chain_id=5)

# get non-endpoint metadata
metadata: Mapping[str, Any] = mesc.get_global_metadata(profile='xyz_tool')
```

## Reference

```python
from typing import Any, MutableMapping, TypedDict, Literal, Sequence

class RpcConfig(TypedDict):
    mesc_version: str
    default_endpoint: str | None
    endpoints: MutableMapping[str, Endpoint]
    network_defaults: MutableMapping[str, str]
    network_names: MutableMapping[str, str]
    profiles: MutableMapping[str, Profile]
    global_metadata: MutableMapping[str, Any]

class Endpoint(TypedDict):
    name: str
    url: str
    chain_id: str | None
    endpoint_metadata: MutableMapping[str, Any]

class Profile(TypedDict):
    name: str
    default_endpoint: str | None
    network_defaults: MutableMapping[str, str]
```
