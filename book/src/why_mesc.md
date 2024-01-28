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
