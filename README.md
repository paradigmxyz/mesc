
# Multiple Endpoint Shared Configuration (MESC) Standard

[![Specification](https://img.shields.io/badge/Spec-blueviolet)](https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md)
[![issues - badge-generator](https://img.shields.io/badge/Docs-blueviolet)](https://paradigmxyz.github.io/mesc)
[![Rust Tests](https://github.com/paradigmxyz/mesc/workflows/Rust%20Tests/badge.svg)](https://github.com/paradigmxyz/mesc/tree/main/tests)
[![Python Tests](https://github.com/paradigmxyz/mesc/workflows/Python%20Tests/badge.svg)](https://github.com/paradigmxyz/mesc/tree/main/tests)


MESC is a standard for how crypto tools configure their RPC endpoints. By following this specification, a user creates a single RPC configuration that can be shared by all crypto tools on their system.

MESC has two main design goals:
1. make it easy to share RPC configuration data across tools, languages, and environments
2. make it easy to manage the configuration of a large number of RPC endpoints

MESC is formally defined in [SPECIFICATION.md](./SPECIFICATION.md).

Additional information can be found in the [MESC Documentation](https://paradigmxyz.github.io/mesc/).

![basics](https://github.com/paradigmxyz/mesc/assets/7907648/2f84d90b-9cfd-42fd-89f1-bd35949e1c14)


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

The quickest way to use MESC is:
1. create a `mesc.json` config file
2. set the `MESC_PATH` environment variable to the path of this file

These steps can be performed automatically using the interactive [`mesc`](./cli) CLI tool:
1. Install: `cargo install mesc_cli`
2. Perform interactive setup: `mesc setup`

Installing the `mesc` cli on some linux distributions may require installing ssl libraries (e.g. `sudo apt-get install pkg-config libssl-dev` on ubunutu)

## Tutorial

Below is a brief tutorial on MESC. For more detail, see the MESC [Specification](./SPECIFICATION.md) and [Documentation](https://paradigmxyz.github.io/mesc). 

Topics:
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
endpoints: list[Endpoint] = mesc.find_endpoints(chain_id=5)
```

###### rust
```rust
use mesc;
use mesc::MescError;

type OptionalResult = Result<Option<Endpoint>, MescError>;
type MultiResult = Result<Vec<Endpoint>, MescError>;

// get the default endpoint
let endpoint: OptionalResult = mesc::get_default_endpoint(None);

// get the default endpoint of a network
let endpoint: OptionalResult = mesc::get_endpoint_by_network(5, None);

// get the default network for a particular tool
let chain_id: OptionalResult = mesc::get_default_endpoint("xyz_tool");

// get the default endpoint of a network for a particular tool
let endpoint: OptionalResult = mesc::get_endpoint_by_network(5, "xyz_tool");

// get an endpoint by name
let endpoint: OptionalResult = mesc::get_endpoint_by_name("local_goerli");

// parse a user-provided string into a matching endpoint
// (first try 1. endpoint name, then 2. chain id, then 3. network name)
let endpoint: OptionalResult = mesc::get_endpoint_by_query(user_str, "xyz_tool");

// find all endpoints matching given criteria
let query = mesc::MultiEndpointQuery::new().chain_id(5);
let endpoints: MultiResult = mesc::find_endpoints(query);
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
