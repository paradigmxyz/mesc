
# MESC CLI

This is a utility for creating and managing MESC RPC configurations.

Under the hood, this cli is implemented using the rust crate [here](../rust/crates/mesc_cli).

The most important cli subcommands:
1. `mesc setup`: create and modify MESC configs
2. `mesc ls`: list endpoints
3. `mesc ping`: ping endpoints
4. `mesc url`: print endpoint url

View help for each subcommand by typing `mesc [SUBCOMMAND] --help`

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
git clone https://github.io/paradigmxyz/msec
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

```
Utility for creating and managing MESC RPC configurations

Usage: mesc <COMMAND>

Commands:
  setup     Create new configuration or modify existing configuration
  status    Print status of configuration
  ls        Print list of endpoints
  defaults  Print list of defaults
  ping      Ping endpoints and fetch metadata
  endpoint  Print endpoint
  metadata  Print metadata
  url       Print endpoint URL
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

- If an endpoint is omitted, use the default endpoint
- If a chain_id or network name is provided, use the default endpoint of network
- Can use `--profile PROFILENAME` to use defaults of a particular profile
