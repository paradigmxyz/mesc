
# Multiple Endpoint Shared Configuration (MESC) Standard

MESC is a standard for how crypto tools configure their RPC endpoints.

By following this specification, a user creates a single RPC configuration that can be shared by all crypto tools on their system.

MESC has two main design goals:
1. make it easy to share RPC configuration data across tools, languages, and environments
2. make it easy to manage the configuration of a large number of RPC endpoints

MESC is formally defined in [SPECIFICATION.md](./SPECIFICATION.md).

### Contents
- [Reference Implementations](#reference-implementations)
- [Quickstart](#quickstart)
- [Tutorial](#tutorial)
  - [Tracked Information](#tracked-information)
  - [Common Interface](#common-interface)
  - [Typical Usage](#typical-usage)

## Reference Implementations

Reference implementations are provided for each of the following:
- [cli](/cli)
- [go](/go) [WIP]
- [python](/python)
- [rust](/rust)
- [typescript](/typescript) [WIP]

These implementations provide a consistent language-agnostic interface while still obeying the best practices of each language.

## Quickstart

The interactive [`mesc`](./cli) CLI tool makes it easy to create and manage a MESC configuration.
1. Install: `cargo install mesc_cli`
2. Create config interactively: `mesc setup`

To create a MESC config manually:
1) Create a JSON file (can use [the example](./SPECIFICATION.md#example-rpcconfig) from the spec as a template).
2) Set `MESC_PATH` to the path of this JSON file.

## Tutorial

Below is a brief tutorial on MESC. For more detail, see [SPECIFICATION.md](./SPECIFICATION.md). 

- [Tracked Information](#tracked-information)
- [Common Interface](#common-interface)
- [Typical Usage](#typical-usage)

### Tracked Information

MESC tracks the following information:
1. a list of RPC endpoints, including their `name`, `chain_id`, and `url`
2. the default RPC endpoint to use
3. the default RPC endpoint to use for each network

MESC can also track other information like metadata and tool-specific defaults. Configuration data is stored in a JSON file. To create this file, follow the [Quickstart](#quickstart) instructions above.

### Common Interface

All reference MESC implementations use the same common interface.

Here is a comparison between the python interface and the rust interface:

###### python
```python
import mesc

# get the default endpoint
endpoint = mesc.get_default_endpoint()

# get the default endpoint of a network
endpoint = mesc.get_endpoint_by_network(5)

# get the default endpoint for a particular tool
endpoint = mesc.get_default_endpoint(profile='xyz_tool')

# get the default endpoint of a network for a particular tool
endpoint = mesc.get_endpoint_by_network(5, profile='xyz_tool')

# get an endpoint by name
endpoint = mesc.get_endpoint_by_name('local_goerli')

# parse a user-provided string into a matching endpoint
# (first try 1. endpoint name, then 2. chain id, then 3. network name)
endpoint = mesc.get_endpoint_by_query(user_str, profile='xyz_tool')

# find all endpoints matching given criteria
endpoints = mesc.find_endpoints(chain_id=5)
```

###### rust
```rust
use mesc;
use mesc::MescError;

// get the default endpoint
let endpoint: Result<Endpoint, MescError> = mesc::get_default_endpoint(None);

// get the default endpoint of a network
let endpoint: Result<Endpoint, MescError> = mesc::get_endpoint_by_network(5, None);

// get the default network for a particular tool
let chain_id: Result<Endpoint, MescError> = mesc::get_default_endpoint("xyz_tool");

// get the default endpoint of a network for a particular tool
let endpoint: Result<Endpoint, MescError> = mesc::get_endpoint_by_network(5, "xyz_tool");

// get an endpoint by name
let endpoint: Result<Endpoint, MescError> = mesc::get_endpoint_by_name("local_query");

// parse a user-provided string into a matching endpoint
// (first try 1. endpoint name, then 2. chain id, then 3. network name)
let endpoint: Result<Option<Endpoint>, MescError> = mesc::get_endpoint_by_query(user_str, "xyz_tool");

// find all endpoints matching given criteria
let query = mesc::MultiEndpointQuery::new().chain_id(5);
let endpoints: Result<Vec<Endpoint>, MescError> = mesc::find_endpoints(query);
```

### Typical Usage

Imagine a crypto cli tool `xyz`. This tool has an argument `-r <RPC_URL>` that specifies which RPC endpoint to use.

If `xyz` uses MESC, then `-r` can become a much more versatile argument. Instead of just accepting a plain URL, `-r` can accept 1. an endpoint name, 2. chain id, or 3. a network name. Each of the following might resolve to the same RPC url:
- `xyz -r localhost:8545` (url)
- `xyz -r local_goerli` (endpoint name)
- `xyz -r 5` (chain id)
- `xyz -r goerli` (network name)

This url resolution can implemented within `xyz` using:

```python
# python code used by xyz tool
endpoint = mesc.get_endpoint_by_query(user_input, profile='xyz')
url = endpoint['url']
```

```rust
// rust code used by xyz tool
let endpoint = mesc::get_endpoint_by_query(user_input, Some("xyz"))?;
let url = endpoint.url;
```
