
# Rust MESC Implementation

This is a reference rust implementation of the MESC standard.

The `crates/` directory contains rust libraries for reading, writing, and validating MESC configurations.

The `crates/mesc_cli` crate is a tool for reading, writing, and validating MESC from the command line. See [CLI](../cli) for details.

### Contents
- [Installation](#Installation)
- [Example-Usage](#Example-Usage)
- [Reference](#Reference)

## Installation

Inside a cargo project: `cargo add mesc`

Inside a `Cargo.toml`: `mesc = "1.0"`

## Example Usage

Basic data structures
```rust
struct Endpoint {
    chain_id: u64,
    url: String,
    endpoint_extras: HashMap<str, serde_json::Value>,
}
```

Basic read functions
```rust
use mesc;

// get the default network
let chain_id: Result<Option<u64>> = mesc::get_default_network(None);

// get the default endpoint of a network
let endpoint: Result<Endpoint> = mesc::get_default_endpoint(5, None);

// get the default network for a particular tool
let chain_id: Result<Endpoint> = mesc::get_default_network(profile="xyz_tool");

// get the default endpoint of a network for a particular tool
let endpoint: Result<Endpoint> = mesc::get_default_endpoint(5, profile="xyz_tool");

// get an endpoint by name
let endpoint: Result<Endpoint> = mesc::get_endpoint_by_name(name);

// parse a user-provided string into a matching endpoint
// (first try 1. endpoint name, then 2. chain id, and then 3. network name)
let endpoint: Result<Option<Endpoint>> = mesc.parse_endpoint(user_str, "xyz_tool");

// find all endpoints matching given criteria
let query = mesc::EndpointQuery { chain_id: Some(5) };
let endpoints: Result<Vec<Endpoint>> = mesc.find_endpoints(query);
```
