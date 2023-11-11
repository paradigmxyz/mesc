
# Rust MESC Implementation

This is a reference rust implementation of the MESC standard.

The `crates/` directory contains rust libraries for reading, writing, and validating MESC configurations.

The `crates/mesc_cli` crate is a tool for reading, writing, and validating MESC from the command line. See [CLI](../cli) for details.

## Contents
- [Installation](#Installation)
- [Example-Usage](#Example-Usage)
- [Reference](#Reference)

## Installation

Inside a cargo project: `cargo add mesc`

Inside a `Cargo.toml`: `mesc = "1.0"`

## Example Usage

```rust
use mesc;

let default_network: Result<Option<u64>> = mesc::get_default_network(None);
let goerli_endpoint: Result<Endpoint> = mesc::get_default_endpoint(5, None);
let goerli_endpoint: Result<Endpoint> = mesc::get_endpoint("local_goerli");
```

## Reference

```rust
struct Endpoint {
    chain_id: u64,
    url: String,
    endpoint_extras: HashMap<str, serde_json::Value>,
}

fn get_default_network(profile: Option<&str>) -> Result<Option<u64>>;
fn get_default_endpoint(chain_id: u64, profile: Option<&str>) -> Result<Endpoint>;
fn get_endpoint(name: &str) -> Result<Endpoint>;
```
