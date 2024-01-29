# Overrides

Overrides are a way to quickly modify a MESC configuration without needing to modify any files.

Each override is an environment variable that overrides a specific MESC configuration key.

Each override uses a syntax that is quick and easy to write by hand.

## Example usage

The `MESC_DEFAULT_ENDPOINT` override changes the default endpoint.

#### Use in a script

Sometimes it might be useful tweak a MESC config for a shell script. This can be achieved by inserting the MESC overrides into the script:

```bash
#!/usr/bin/env bash

export MESC_DEFAULT_ENDPOINT=local_goerli

# ...rest of script that does RPC things
```

#### Prepending syntax

Adding `VAR_NAME=VAR_VALUE` before a cli command will set an environment variable for just that command.

So if before the default url is this:

```bash
mesc url
> https://eth.llamarpc.com
```

Adding the override before the command will change the default url:

```bash
MESC_DEFAULT_ENDPOINT=local_goerli mesc url
> localhost:8545
```

This syntax works for any cli program, not just the `mesc` cli tool.

## List of overrides

Every type of information within a MESC configuration can be modified using overrides:

| override variable | value syntax | example |
| --- | --- | --- |
| `MESC_DEFAULT_ENDPOINT`  | url, endpoint name, or network name                               | `localhost:9999` |
| `MESC_NETWORK_DEFAULTS`  | space-separated pairs of `<chain_id>=<endpoint>`                  | `5=alchemy_optimism 1=local_mainnet` |
| `MESC_NETWORK_NAMES`     | space-separated pairs of `<network_name>=<chain_id>`              | `zora=7777777` |
| `MESC_ENDPOINTS`         | space-separated items of `[<endpoint_name>[:<chain_id>]=]<url>`   | `alchemy_optimism=https://alchemy.com/fjsj local_goerli:5=localhost:8545` |
| `MESC_PROFILES`          | space-separated pairs of `<profile>.<key>[.<subkey]=<endpoint>`   | `foundry.default_endpoint=local_goerli foundry.network_defaults.5=alchemy_optimism` |
| `MESC_GLOBAL_METADATA`   | JSON formatted global metadata                                    | `{}` |
| `MESC_ENDPOINT_METADATA` | JSON mapping of `{"endpoint_name": {<ENDPOINT_METADATA>}}`        | `{}` |

