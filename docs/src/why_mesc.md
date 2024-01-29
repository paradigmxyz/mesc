# Why MESC?

Between mainnet, testnets, chainforks, rollups, and alternative L1's, modern crypto tools must manage the configuration of multiple RPC endpoints. This configuration process is not standardized across tools.

The most common approach for configuring RPC endpoint information is the `ETH_RPC_URL` environment variable (dapptools, forge, heimdall, checkthechain). However, this is not a formal standard and many tools use other approaches. Furthermore, using `ETH_RPC_URL` can only specify a single provider for a single chain, and it cannot specify any provider metadata beyond the url.

Instead it would be desirable to have a solution that:
- allows specifying multiple providers for multiple chains
- allows selection of a default endpoint for each chain
- allows using either environment variables or config files
- is a single source of truth across multiple tools
- is OS-agnostic, using no OS-specific features
- is language-agnostic, using no language-specific features
- is backwards compatible with previous solutions

MESC aims to satisfy all of these constraints.


## As a developer, why should I integrate MESC into my tools?

- **Easier Onboarding**. As soon as a user installs your tool, they will be able to use your tool with all of their configured networks. This eliminates one of the major setup steps for most RPC consuming tools.
- **Interoperability**. If you maintain multiple tools, or your tool has components in multiple languages, MESC will create a single, shared source of truth across all of these tool components.
- **Safety and Robustness**. MESC is thoroughly tested against a large number of config-related edge cases. In many cases it will be safer to user MESC than to create a configuration system from scratch.
- **Minimize maintenance burden**. Leaning on MESC to handle configuration means there is one less thing you need to maintain. You can spend your time on other things.
- **Developer Features**. If you need to develop or test your tool with multiple endpoints, the `mesc` CLI tools contains many useful developer features for managing endpoints.
- **Go config-free**. In some cases, adopting MESC means that your tool no longer needs its own config. You can store your tool's config data in MESC's metadata and then just use MESC for all config IO.
