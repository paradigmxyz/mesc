# What is MESC?

MESC (Multiple Endpoint Shared Configuration) is a standardized approach for configuring RPC endpoints. Using MESC, a user creates a single RPC config that is shared by all RPC tools on their system.

## Design Goals

MESC has two main design goals:

1. make it easy to share RPC configuration data across tools, languages, and environments
2. make it easy to manage the configuration of a large number of RPC endpoints

MESC also has many secondary goals. It aims to be a solution that:
- allows specifying multiple providers for multiple chains
- allows selection of a default endpoint for each chain
- allows using either environment variables or config files
- is a single source of truth across multiple tools
- is OS-agnostic, using no OS-specific features
- is language-agnostic, using no language-specific features
- is backwards compatible with previous solutions

## MESC is opt-in

MESC is an **opt-in** standard: MESC is only enabled if a user has set a MESC-specific environment variable in their environment. This maximizes backward-compatiblity for existing tools that want to integrate MESC: If a user has not opted-in to MESC then simply fall back to the existing configuration method.

