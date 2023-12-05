
# Cli MESC Implementation

This is a reference cli implementation of the Multiple Endpoint Shared Configuration standard.

## Contents
- [Installation](#installation)
- [Example Usage](#example-usage)
- [Reference](#reference)

## Installation

Use one of the 3 options below. Check that `mesc` is properly installed and on your `PATH` by running `mesc -h`.

### Install pre-built binary

Download the appropriate binary for your architecture from the [releases]() page.

### Install using crates.io

`cargo install mesc`

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

Print all configuration data
```console
# print default goerli endpoint data, human readable
mesc pretty goerli

# print default goerli endpoint data, as json
mesc json goerli

# print entire configuration
mesc all --pretty
mesc all --json
```

## Reference

```
mesc setup                         perform configuration setup
mesc status                        print configuration status
mesc url [ENDPOINT_OR_NETWORK]     print endpoint url
mesc json [ENDPOINT_OR_NETWORK]    print configuration data as json
mesc pretty [ENDPOINT_OR_NETWORK]  print configuration data as human-readable
mesc find                          search for endpoints
```

- If endpoint is omitted, use default endpoint of default network
- If a chain_id or network name is provided, use default endpoint of network
- Can `--profile PROFILENAME` to use a particular profile
