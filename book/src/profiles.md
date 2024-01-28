# Profiles

Profiles are a way for each tool to customize its own MESC settings.

Profiles allow each tool to:
- set its own default endpoint
- set its own default endpoints for each network
- override global metadata with its own metadata
- avoid using MESC for that tool without disabling MESC globally

## Profiles are an optional feature

If a MESC query does not specify a profile, the global configuration values are used.

If a MESC query specifies a profile, but that profile does not exist or does not specify the relevant information, MESC will fallback to using global configuration values.

## Setting up a profile

The easiest way to create a profile is by using the interactive `mesc` cli tool, using the `mesc setup` subcommand.

Alternatively, you can manually add a new `Profile` to the `profiles` key inside your `mesc.json`. The following is a template `Profile`:

```json
{
    "name": "PROFILE_NAME",
    "default_endpoint": null,
    "network_defaults": {},
    "profile_metadata": {},
    "use_mesc": true
}
```

