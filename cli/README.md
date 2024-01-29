
# MESC CLI

This is a utility for creating and managing MESC RPC configurations.

Under the hood, this cli is implemented using the rust crate [here](../rust/crates/mesc_cli).

The most important cli subcommands:
1. `mesc setup`: create and modify MESC configs
2. `mesc ls`: list endpoints
3. `mesc ping`: ping endpoints
4. `mesc url`: print endpoint url

View help for each subcommand by typing `mesc [SUBCOMMAND] --help`

![basics](https://github.com/paradigmxyz/mesc/assets/7907648/2f84d90b-9cfd-42fd-89f1-bd35949e1c14)

## Contents
- [Installation](#installation)
- [Example Usage](#example-usage)
- [Reference](#reference)

## Installation

Use one of the 3 options below. Check that `mesc` is properly installed and on your `PATH` by running `mesc -h`.

### Install from crates.io

`cargo install mesc_cli`

Ensure that your cargo install path is on your cli path

### Install from source

```console
# install rust and cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install mesc
git clone https://github.com/paradigmxyz/mesc
cd mesc
cargo install --path rust/crates/cli
```

## Example Usage

Quickly obtain RPC url's:
```console
# curl the default network rpc url
curl $(mesc url) ...

# curl the default goerli url
curl $(mesc url goerli) ...

# curl an endpoint by name
curl $(mesc url local_goerli) ...
```

Print configuration data
```console
# print all endpoints in table
mesc ls

# ping endpoints and collect metadata
mesc ping

# print default goerli endpoint data, human readable
mesc endpoint goerli

# print default goerli endpoint data, as json
mesc endpoint goerli --json
```

## Reference

Show in terminal by typing `mesc --help`:

```
command line interface for creating, loading, and modifying MESC configuration data

Usage: mesc <COMMAND>

Commands:
  setup     Create or modify config interactively
  import    Modify config by importing from file or other source
  set       Modify config by setting specific values
  ping      Ping endpoints and fetch metadata
  defaults  Print list of defaults
  endpoint  Print endpoint
  help      Print help
  ls        Print list of endpoints
  metadata  Print metadata
  status    Print status of configuration
  url       Print endpoint URL

Options:
  -h, --help     Print help
  -V, --version  Print version

Help topics: (print these with mesc help <TOPIC>)
  env       Environmental variables
  python    Python interface
  rust      Rust interface
  schema    Schemas of configs, endpoints, and profiles
  setup     How to set up MESC
```

- If an endpoint is omitted, use the default endpoint
- If a chain_id or network name is provided, use the default endpoint of network
- Can use `--profile PROFILENAME` to use defaults of a particular profile
