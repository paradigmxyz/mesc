
# Python MESC Implementation

This is a reference implementation of the Python MESC Standard

- [Installation](#installation)
- [Example Usage](#example-usage)
- [Reference](#reference)

## Installation

### From PyPI
`pip install mesc`

### From Source
```
git clone https://github/paradigmxyz/mesc
cd mesc
pip install ./
```

## Example Usage

```python
import mesc

default_network = mesc.get_default_network()
default_goerli_endpoint = mesc.get_default_endpoint(chain_id=5)
local_goerli_endpoint = mesc.get_endpoint('local_goerli')
```

## Reference

Data structures
```python
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
```

Basic read functions
```python
def get_default_network(
    *,
    profile: str | None = None,
    require_profile: bool = False,
) -> int | None:
    ...

def get_default_endpoint(
    chain_id: int,
    *,
    profile: str | None = None,
    require_profile: bool = False
) -> Endpoint:
    ...

def get_endpoint(name: str) -> Endpoint:
    ...
```
