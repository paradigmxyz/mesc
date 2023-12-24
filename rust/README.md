
# Rust MESC Implementation

This is a reference rust implementation of the MESC standard.

The `crates/mesc` crate contains rust libraries for reading, writing, and validating MESC configurations.

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
