
# Setting up MESC

## Basic setup

MESC becomes enabled by setting one or more environment variables.

In the simplest case, only one variable necessary: `MESC_PATH` is set to the path of a `mesc.json` config file.

As described in the [quickstart](./quickstart.md), the `mesc.json` file is usually created using either 1) the interactive `mesc setup` command, or 2) copying from a starter template. The quickstart guide also describes how to set environment variables in your terminal shell.

## Alternative setup without a `mesc.json`

Sometimes it is convenient to configure a system without editing any files (e.g. inside a container, or on a network drive, or in a low-privilege environment).

This can be accomplished with MESC by setting the `MESC_ENV` variable instead of the `MESC_PATH` variable. `MESC_ENV` should simply contain the JSON content of a MESC configuration.

If both `MESC_PATH` and `MESC_ENV` are set, you can select which one to use by setting `MESC_MODE` to either `PATH` or `ENV`. `MESC_PATH` takes precedence over `MESC_ENV` if `MESC_MODE` is not set.

## Disabling MESC

MESC can be disabled by either 1) unsetting all `MESC_*` variables, or 2) setting `MESC_MODE=DISABLED`.

If MESC is disabled, the `is_mesc_disabled()` function will return `false` and all MESC querying functions will return an error (depending on language).

## Overrides

MESC also uses environment variables for overrides. See the [overrides](overrides.md) section for details.

