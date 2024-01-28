# Integrating MESC

MESC is designed to be easy to integrate into any existing tool in a backward-compatible manner.

MESC integration patterns can be designed so that they only affect users that have opted-in to MESC, while leaving other users unaffected.

Examples below are shown in python but work nearly identically for each language.

## Workflow for getting the default RPC endpoint

Let's say that an existing tool named `xyz` currently gets its default RPC endpoint from an environment variable `ETH_RPC_URL` like so:

```python
def get_default_rpc_url() -> str | None:
    return os.environ.get('ETH_RPC_URL')
```

MESC can be integrated using a few lines of code:

```python
import mesc

def get_default_rpc_url() -> str | None:
    if mesc.is_mesc_enabled():
        endpoint = mesc.get_default_endpoint(profile='xyz')
        if endpoint is not None:
            return endpoint['url']
    return os.environ.get('ETH_RPC_URL')
```

This new function loads the MESC default endpoint for users that have opted-in to MESC. But it falls back to using `ETH_RPC_URL` if MESC is not enabled or MESC does not have a default endpoint set.

Using `profile='xyz'` is optional, but it allows users to set custom settings for the `xyz` tool instead of using the global settings.

## Workflow for letting users select an RPC endpoint

Let's say that an existing tool named `xyz` currently lets users choose an RPC endpoint using an `-r` input argument. Users might use the tool in a similar manner to `xyz -r https://eth.llamarpc.com` or `xyz -r localhost:8545`.

If we add an extra parsing step for the user input, we can allow the user to select from any endpoint inside the MESC configuration by name, by network name, or by chain id. Using a pattern like this:

```python
def get_endpoint_url(user_input: str) -> str:
    if mesc.is_mesc_enabled():
        endpoint = mesc.get_endpoint_by_query(user_input)
        if endpoint is not None:
            return endpoint['url']
    return user_input

# ...

url = get_endpoint_url(user_input)
```

Then users will be able to do any of the following

- `xyz -r localhost:8545` (use a url)
- `xyz -r llamanodes_goerli` (use the endpoint named `llamanodes_goerli`)
- `xyz -r 5` (use the default endpoint for network with chain id `5`)
- `xyz -r goerli` (use the default endpoint for the network named `goerli`)

