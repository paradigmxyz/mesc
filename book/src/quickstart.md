# Quickstart

The quickest way to use MESC is
1) create a `mesc.json` config file
2) set the `MESC_PATH` environment variable to the path of this file

You can create a `mesc.json` either 1) by using the interactive [MESC cli tool](https://github.com/paradigmxyz/mesc/tree/main/cli) (install using `cargo install mesc_cli`) or 2) by modifying the template below:

```json
{
    "mesc_version": "MESC 1.0",
    "default_endpoint": "local_ethereum",
    "network_defaults": {
        "1": "local_ethereum"
    },
    "network_names": {},
    "endpoints": {
        "local_ethereum": {
            "name": "local_ethereum",
            "url": "http://localhost:8545",
            "chain_id": "1",
            "endpoint_metadata": {}
        }
    },
    "profiles": {
        "xyz": {
            "name": "xyz",
            "default_endpoint": "local_ethereum",
            "network_defaults": {
                "1": "local_ethereum"
            },
            "profile_metadata": {},
            "use_mesc": true
        }
    },
    "global_metadata": {}
}
```

Configuration should follow the MESC [specification](https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md).
