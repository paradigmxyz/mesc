
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

Inside a `Cargo.toml`: `mesc = "0.2.1"`

## Example Usage

Basic data structures
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcConfig {
    pub mesc_version: String,
    pub default_endpoint: Option<String>,
    pub endpoints: HashMap<String, Endpoint>,
    pub network_defaults: HashMap<ChainId, String>,
    pub network_names: HashMap<String, ChainId>,
    pub profiles: HashMap<String, Profile>,
    pub global_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub url: String,
    pub chain_id: Option<ChainId>,
    pub endpoint_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub default_endpoint: Option<String>,
    pub network_defaults: HashMap<ChainId, String>,
    pub profile_metadata: HashMap<String, serde_json::Value>,
    pub use_mesc: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChainId(String);
```

Basic read functions
```rust
use mesc::{MescError, Endpoint};
use std::collections::HashMap;

type OptionalResult = Result<Option<Endpoint>, MescError>;
type MultiResult = Result<Vec<Endpoint>, MescError>;
type MetadataResult = Result<HashMap<String, serde_json::Value>, MescError>;

// check whether mesc is enabled
let enabled: bool = mesc::is_mesc_enabled();

// get the default endpoint
let endpoint: OptionalResult = mesc::get_default_endpoint(None);

// get the default endpoint of a network
let endpoint: OptionalResult = mesc::get_endpoint_by_network("5", None);

// get the default network for a particular tool
let chain_id: OptionalResult = mesc::get_default_endpoint(Some("xyz_tool"));

// get the default endpoint of a network for a particular tool
let endpoint: OptionalResult = mesc::get_endpoint_by_network("5", Some("xyz_tool"));

// get an endpoint by name
let endpoint: OptionalResult = mesc::get_endpoint_by_name("local_goerli");

// parse a user-provided string into a matching endpoint
// (first try 1. endpoint name, then 2. chain id, then 3. network name)
let user_str = "local_goerli";
let endpoint: OptionalResult = mesc::get_endpoint_by_query(user_str, Some("xyz_tool"));

// find all endpoints matching given criteria
let query = mesc::MultiEndpointQuery::new().chain_id("5").unwrap();
let endpoints: MultiResult = mesc::find_endpoints(query);

// get non-endpoint metadata
let metadata: MetadataResult  = mesc::get_global_metadata(Some("xyz_tool"));
```
