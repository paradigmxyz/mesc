
# Python MESC Implementation

This is a reference implementation of the Python MESC Standard

It has no required external dependencies.

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

Basic data structures
```python
class Endpoint(TypedDict):
    url: str
    chain_id: int
    endpoint_extras: Mapping[str, Any]
```

Basic read functions

```python
import mesc

# get the default network
chain_id = mesc.get_default_network()

# get the default endpoint of a network
endpoint = mesc.get_default_endpoint(5)

# get the default network for a particular tool
chain_id = mesc.get_default_network(profile='xyz_tool')

# get the default endpoint of a network for a particular tool
endpoint = mesc.get_default_endpoint(5, profile='xyz_tool')

# get an endpoint by name
endpoint = mesc.get_endpoint_by_name(name)

# parse a user-provided string into a matching endpoint
# (first try 1. endpoint name, then 2. chain id, and then 3. network name)
endpoint = mesc.parse_endpoint(user_str, profile='xyz_tool')

# find all endpoints matching given criteria
endpoints = mesc.find_endpoints(chain_id=5)
```
