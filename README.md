
# Multiple Endpoint Single Configuration Standard (MESC)

MESC is a specification for how EVM tools can configure their RPC endpoints.

By following this specification, a user creates a single RPC configuration that can be used by all compliant tools in an OS-agnostic and language-agnostic way.

The MESC specification is defined in [SPECIFICATION.md](./SPECIFICATION.md).

### Contents
- [Reference Implementations](#reference-implementations)
- [Quickstart](#quickstart)
- [Tutorial](#tutorial)

## Reference Implementations

Reference implementations are provided for each of the following:
- [cli](/cli) [TODO]
- [javascript](/javascript) [TODO]
- [python](/python) [TODO]
- [rust](/rust) [TODO]

These implementations provide a consistent language-agnostic interface while still obeying the conventions of each language.

## Quickstart

The interactive [`mesc`](./cli) CLI tool makes it easy to create and manage a MESC configuration. Running `mesc setup` will prompt a user to enter their RPC endpoints, choose their defaults, and configure their environment variables.

To perform this process manually:
1) Create a MESC JSON file (can use [the example](./SPECIFICATION.md#example-rpcconfig) from the spec as a template).
2) Set `RPC_CONFIG_PATH` to the path of this JSON file.

## Tutorial

MESC tracks the following information:
1. a list of RPC endpoints, including their `name`, `chain_id`, and `url`
2. the default RPC endpoint that should be used for each network
3. the default network that should be used

MESC can also track additional information like metadata and tool-specific default settings.

This configuration data is stored in a JSON file. Users should set their `RPC_CONFIG_PATH` environment to the location of a MESC JSON file.

A more thorough tutorial can be found in the [In-Depth Tutorial]().
