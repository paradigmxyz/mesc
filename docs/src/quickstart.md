# Quickstart

The quickest way to use MESC is
1) create a `mesc.json` config file
2) set the `MESC_PATH` environment variable to the path of this file

## Creating a config file

You can create a `mesc.json` by one of two ways:
1) use the interactive [MESC cli tool](https://github.com/paradigmxyz/mesc/tree/main/cli) (install using `cargo install mesc_cli` and run `mesc setup`)
2) modify the template below:

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

The structure of the config must follow the MESC [specification](https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md).

## Setting environment variables

The typical way to set environment variables is in your shell configuration files: `~/.bashrc`, `~.profile`, and/or `~/.bash_profile`. Including this line in those files will enable MESC:

```bash
export MESC_PATH=/path/to/your/mesc.json
```

You can avoid editing these files yourself by running the MESC setup tool (`mesc setup`) as specified above. It will give you the option to automatically edit your shell files.

