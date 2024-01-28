---
title: Multiple Endpoint Single Config (MESC)
description: By following this specification, a user creates a single RPC configuration that can be used by all compliant tools.
author: Storm Slivkoff (@sslivkoff)
discussions-to: https://github.com/paradigmxyz/mesc
status: Draft
type: Standards
category: Interface
created: 2023-12-24
---

<!--
  READ EIP-1 (https://eips.ethereum.org/EIPS/eip-1) BEFORE USING THIS TEMPLATE!

  This is the suggested template for new EIPs. After you have filled in the requisite fields, please delete these comments.

  Note that an EIP number will be assigned by an editor. When opening a pull request to submit your EIP, please use an abbreviated title in the filename, `eip-draft_title_abbrev.md`.

  The title should be 44 characters or less. It should not repeat the EIP number in title, irrespective of the category.

  TODO: Remove this comment before submitting
-->

## Abstract

<!--
  The Abstract is a multi-sentence (short paragraph) technical summary. This should be a very terse and human-readable version of the specification section. Someone should be able to read only the abstract to get the gist of what this specification does.

  TODO: Remove this comment before submitting
-->

MESC is a standard for how crypto tools configure their RPC endpoints.

By following this specification, a user creates a single RPC configuration that can be shared by all crypto tools on their system.

MESC has two main design goals:
1. make it easy to share RPC configuration data across tools, languages, and environments
2. make it easy to manage the configuration of a large number of RPC endpoints

## Motivation

<!--
  This section is optional.

  The motivation section should include a description of any nontrivial problems the EIP solves. It should not describe how the EIP solves those problems, unless it is not immediately obvious. It should not describe why the EIP should be made into a standard, unless it is not immediately obvious.

  With a few exceptions, external links are not allowed. If you feel that a particular resource would demonstrate a compelling case for your EIP, then save it as a printer-friendly PDF, put it in the assets folder, and link to that copy.

  TODO: Remove this comment before submitting
-->

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

## Specification

<!--
  The Specification section should describe the syntax and semantics of any new feature. The specification should be detailed enough to allow competing, interoperable implementations for any of the current Ethereum platforms (besu, erigon, ethereumjs, go-ethereum, nethermind, or others).

  It is recommended to follow RFC 2119 and RFC 8170. Do not remove the key word definitions if RFC 2119 and RFC 8170 are followed.

  TODO: Remove this comment before submitting
-->

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119 and RFC 8174.

We specify an approach that provides all of the desirable properties listed above.

#### Schema

MESC is built using three key-value data schemas:

##### `RpcConfig` schema:

| key                 | value type               | description |
| ---                 | ---                      | --- |
| `mesc_version`      | `str`                    | must equal the value `"MESC 1.0"`
| `default_endpoint`  | `str \| None`            | name of default endpoint
| `network_defaults`  | `Mapping[ChainId, str]`  | map of chain_id's to endpoint names
| `network_names`     | `Mapping[str, ChainId]`  | map of network names to chain_id's
| `endpoints`         | `Mapping[str, Endpoint]` | map of endpoint names to endpoints
| `profiles`          | `Mapping[str, Profile]`  | map of profile names to profiles
| `global_metadata`   | `Mapping[str, Any]`      | global metadata entries

##### `Endpoint` schema:

| key                 | value type          | description |
| ---                 | ---                 | --- |
| `name`              | `str`               | name of endpoint
| `url`               | `str`               | url of endpoint, including transport
| `chain_id`          | `ChainId \| None`   | chain id of network
| `endpoint_metadata` | `Mapping[str, Any]` | endpoint metadata entries

##### `Profile` schema:

| key                 | value type               | description |
| ---                 | ---                      | --- |
| `name`              | `str`                    | name of profile
| `default_endpoint`  | `str \| None`            | chain_id of default network
| `network_defaults`  | `Mapping[ChainId, str]`  | map of chain_id's to endpoint names
| `profile_metadata`  | `Mapping[str, Any]`      | profile metadata entries
| `use_mesc`          | `bool`                   | whether to disable MESC when this profile is selected

Requirements:
- All keys of `RpcConfig` and `Endpoint` are required. No additional keys must be present, except within `global_metadata`, `profile_metadata`, and `endpoint_metadata`.
- Every endpoint name specified in `RpcConfig.default_endpoint` and in `RpcConfig.network_defaults` must exist in `RpcConfig.endpoints`.
- These key-value structures can be easily represented in JSON and in most common programming languages.
- EVM `chain_id`'s must be represented using either a decimal string or a hex string. Strings are used because chain id's can be 256 bits and most programming languages do not have native 256 bit integer types. For readability, decimal should be used for small chain id values and hex should be used for values that use the entire 256 bits.
- Names of endpoints, networks, and profiles should be composed of characters that are either alphanumeric, dashes, underscores, or periods. Names should be at least 1 character long.

##### Metadata

The `global_metadata`, `profile_metadata`, and `endpoint_metadata` fields allow for optional or idiosyncratic RPC metadata to be stored alongside the core RPC data. Tools using MESC can choose to ignore these fields. Examples of common metadata:

**Endpoint metadata**
| key | value type | description | examples |
| --- | ---        | ---         | ---      |
| `rate_limit_rps`        | `int \| float`               | ratelimit in requests per second                        | `250`  |
| `rate_limit_cups`       | `int \| float`               | ratelimit in CUPS                                       | `1000` |
| `rate_limit_per_method` | `Mapping[str, int \| float]` | ratelimit in RPS for each method                        | `{"trace_block": 200}` |
| `api_key`               | `str`                        | api key                                                 | `a2798f237a2398rf7` |
| `jwt_secret`            | `str`                        | jwt secret | |
| `host`                  | `str`                        | name of provider host                                   | `"llamanodes"`, `"alchemy"`, `"quicknode"`, `"localhost"`
| `ecosystem`             | `str`                        | ecosystem of chain, (e.g. relates mainnets to testnets) | `"ethereum"`, `"polygon"` |
| `node_client`           | `str`                        | versioned node client                                   | `erigon/2.48.1/linux-amd64/go1.20.5` `reth/v0.1.0-alpha.10-7b781eb60/x86_64-unknown-linux-gnu` |
| `namespaces`            | `Sequence[str]`              | RPC name spaces enabled for endpoint                    | `["eth", "trace, "debug"]`
| `explorer`              | `str`                        | block explorer url                                      | `https://etherscan.io`
| `location`              | `str`                        | geographic region                                       | `Paris, France` |
| `cloud_region`          | `str`                        | cloud provider region                                   | `aws-us-east-1a` |
| `labels`                | `Sequence[str]`              | tags                                                    | `private_mempool`, `cache`, `archive`, `consensus_layer`, `execution_layer`, `validator`, `ephemeral` |

**Global Metadata** and **Profile Metadata**
| key                  | value type                    | description                                                               | examples |
| ---                  | ---                           | ---                                                                       | ---      |
| `last_modified_by`   | `str`                         | versioned tool used to create configuration                               | `mesc__1.0` |
| `last_modified_time` | `int`                         | timestamp of config modification                                          | `1700200462` |
| `creation_time`      | `int`                         | timestamp of config creation                                              | `1700200462` |
| `api_keys`           | `Mapping[str, str]`           | API keys to RPC-related services                                          | `{"etherscan": "abc123"}` |
| `groups`             | `Mapping[str, Sequence[str]]` | groupings of endpoints, mapping of group name to list of endpoint names   | `{"load_balancer": ["alchemy_optimism", "quicknode_optimism"]}` |
| `conceal`            | `bool`                        | whether tool should avoid casually revealing private RPC url's unprompted | `true` |

#### Environment

##### Environment Setup

`RpcConfig` data is stored as JSON either in a file or in an environmental variable.

This configuration data is located using 3 environment variables:
- `MESC_MODE`
- `MESC_PATH`
- `MESC_ENV`

Configuration data is loaded using the following resolution order:
1. check `MESC_MODE`
    - if set to `PATH`, interpret file at `MESC_PATH` as JSON `RpcConfig` data
    - if set to `ENV`, interpret the contents of `MESC_ENV` as JSON `RpcConfig` data
    - if set to `DISABLED` or other value, raise error
    - if unset or empty, continue to (2)
2. check `MESC_PATH`
    - if set to an existing file, interpret as JSON `RpcConfig` data
    - if set to a nonexistent file, raise error
    - if unset or empty, continue to (3)
3. check `MESC_ENV`
    - if set to valid JSON, interpret as JSON `RpcConfig` data
    - if unset or empty, MESC is not being used, continue to (4)
4. check values of MESC environment overrides (see below)
    - if any overrides are set to non-empty values, build config from them
    - if none are set, continue to (5)
5. MESC is not enabled, raise error

MESC is considered to be enabled on a system if 1) at least one MESC environment variable is set to a non-empty value and 2) the `MESC_MODE` environment variable is not set to `DISABLED`.

##### Environment Overrides

MESC also introduces environment variables that can override each configuration key. These overrides allow quick, ad-hoc configuration changes without needing to edit the underlying configuration files. 

These overrides use a simple syntax that is intended to be easily written by humans:

| override variable | value syntax | example |
| --- | --- | --- |
| `MESC_DEFAULT_ENDPOINT`  | url, endpoint name, or network name                               | `localhost:9999` |
| `MESC_NETWORK_DEFAULTS`  | space-separated pairs of `<chain_id>=<endpoint>`                  | `5=alchemy_optimism 1=local_mainnet` |
| `MESC_NETWORK_NAMES`     | space-separated pairs of `<network_name>=<chain_id>`              | `zora=7777777` |
| `MESC_ENDPOINTS`         | space-separated items of `[<endpoint_name>[:<chain_id>]=]<url>`   | `alchemy_optimism=https://alchemy.com/fjsj local_goerli:5=localhost:8545` |
| `MESC_PROFILES`          | space-separated pairs of `<profile>.<key>[.<subkey]=<endpoint>`   | `foundry.default_endpoint=local_goerli foundry.network_defaults.5=alchemy_optimism` |
| `MESC_GLOBAL_METADATA`   | JSON formatted global metadata                                    | `{}` |
| `MESC_ENDPOINT_METADATA` | JSON mapping of `{"endpoint_name": {<ENDPOINT_METADATA>}}`        | `{}` |

- Overrides can be placed within a shell script or inlined to a shell command. For example, to quickly change the default endpoint used by cli tool `xyz`, could use the command `MESC_DEFAULT_ENDPOINT=goerli xyz`. Overrides can also be used with CI/CD environments or containers.
- If URL's are given to `MESC_DEFAULT_ENDPOINT`, `MESC_NETWORK_DEFAULTS`, or `MESC_ENDPOINTS`, `Endpoint` entries will be created as needed in `RpcConfig.endpoints`. If a name is not provided, a random name should be assigned.
- Setting an override variable to an empty value will disable that override from being used.
- Setting an override variable to an invalid value should result in an error upon loading the config.
- Metadata overrides (`MESC_GLOBAL_METADATA` and `MESC_ENDPOINT_METADATA`) merge into the underlying config values instead of replacing them.
- Using `MESC_MODE=DISABLED` is a simple way to completely disable MESC. Using `MESC_PROFILES=profile.use_mesc=false` will disable MESC for a particular profile.

#### Querying Data

When a user specifies an endpoint, as in `-r <ENDPOINT>`, a MESC library must resolve this input into an endpoint URL. It is generally preferable to allow this user query to be a URL, an endpoint name, a network name, or a chain_id. 

MESC data must be searched in the following order:
1. endpoint names
2. chain_id
3. network name

#### Examples

This is a basic configuration of 5 endpoints across 3 networks. It also contains a profile for a tool called `xyz` that defaults to 3rd party endpoints instead of local endpoints.

```json
{
    "mesc_version": "MESC 1.0",
    "default_endpoint": "local_ethereum",
    "network_defaults": {
        "1": "local_ethereum",
        "5": "local_goerli",
        "137": "llamanodes_polygon"
    },
    "network_names": {},
    "endpoints": {
        "local_ethereum": {
            "name": "local_ethereum",
            "url": "http://localhost:8545",
            "chain_id": "1",
            "endpoint_metadata": {}
        },
        "local_goerli": {
            "name": "local_goerli",
            "url": "http://localhost:8546",
            "chain_id": "5",
            "endpoint_metadata": {}
        },
        "llamanodes_ethereum": {
            "name": "llamanodes_ethereum",
            "url": "https://eth.llamarpc.com",
            "chain_id": "1",
            "endpoint_metadata": {}
        },
        "llamanodes_polygon": {
            "name": "llamanodes_polygon",
            "url": "https://polygon.llamarpc.com",
            "chain_id": "137",
            "endpoint_metadata": {}
        }
    },
    "profiles": {
        "xyz": {
            "name": "xyz",
            "default_endpoint": "llamanodes_polygon",
            "network_defaults": {
                "1": "llamanodes_ethereum",
                "137": "llamanodes_polygon"
            },
            "profile_metadata": {},
            "use_mesc": true
        }
    },
    "global_metadata": {}
}
```

## Rationale

<!--
  The rationale fleshes out the specification by describing what motivated the design and why particular design decisions were made. It should describe alternate designs that were considered and related work, e.g. how the feature is supported in other languages.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

Want to satisfy all of these constraints:
- has an interface that can be expressed naturally in most common programming languages
- able to manage large numbers of endpoints, including multiple endpoints per network and a default endpoint for each network
- able to label each endpoint with metadata
- able to express groupings of endpoints
- able to store the config in either a JSON file or an environment variable
- able to override individual settings with human-writable environment variables
- minimize complexity

Other notes:
- `global_metadata`, `profile_metadata`, and `endpoint_metadata` allow extra information to be stored in the config without breaking the standard. This includes api keys, rate limits, and organizational labels. This information might be specific to each application.
- `Profile`s allow different defaults to be assigned to each tool or each mode of operation.
- Allowing RPC information to be configured using either a file or an environment variable allows optimal deployment patterns across a wide range of computing environments. Each also has their own advantages, e.g. file can be used with version control whereas environment variables can be changed more dynamically.


## Backwards Compatibility

<!--

  This section is optional.

  All EIPs that introduce backwards incompatibilities must include a section describing these incompatibilities and their severity. The EIP must explain how the author proposes to deal with these incompatibilities. EIP submissions without a sufficient backwards compatibility treatise may be rejected outright.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

No backward compatibility issues found.

MESC is an opt-in specification that only becomes activated when a user explicitly sets one or more of the environment variables listed above (`MESC_MODE`, `MESC_PATH`, or `MESC_ENV`). These variables are not currently used by any common crypto tools. Tools that use `ETH_RPC_URL` or other configuration approaches will continue working as before.

<!-- ## Test Cases -->

<!--
  This section is optional for non-Core EIPs.

  The Test Cases section should include expected input/output pairs, but may include a succinct set of executable tests. It should not include project build files. No new requirements may be be introduced here (meaning an implementation following only the Specification section should pass all tests here.)
  If the test suite is too large to reasonably be included inline, then consider adding it as one or more files in `../assets/eip-####/`. External links will not be allowed

  TODO: Remove this comment before submitting
-->

## Reference Implementation

<!--
  This section is optional.

  The Reference Implementation section should include a minimal implementation that assists in understanding or implementing this specification. It should not include project build files. The reference implementation is not a replacement for the Specification section, and the proposal should still be understandable without it.
  If the reference implementation is too large to reasonably be included inline, then consider adding it as one or more files in `../assets/eip-####/`. External links will not be allowed.

  TODO: Remove this comment before submitting
-->

A library that reads raw MESC data should provide the following core functions:

```python
# check whether mesc is enabled
enabled = mesc.is_mesc_enabled()

# get the default endpoint
endpoint: Endpoint | None = mesc.get_default_endpoint()

# get the default endpoint of a network
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5)

# get the default endpoint for a particular tool
endpoint: Endpoint | None = mesc.get_default_endpoint(profile='xyz_tool')

# get the default endpoint of a network for a particular tool
endpoint: Endpoint | None = mesc.get_endpoint_by_network(5, profile='xyz_tool')

# get an endpoint by name
endpoint: Endpoint | None = mesc.get_endpoint_by_name('local_goerli')

# parse a user-provided string into a matching endpoint
# (first try 1. endpoint name, then 2. chain id, then 3. network name)
endpoint: Endpoint | None = mesc.get_endpoint_by_query(user_str, profile='xyz_tool')

# find all endpoints matching given criteria
endpoints: Sequence[Endpoint] = mesc.find_endpoints(chain_id=5)

# get non-endpoint metadata
metadata: Mapping[str, Any] = mesc.get_global_metadata(profile='xyz_tool')
```

A reference implementation is provided in the supplemental files of this repository.

## Security Considerations

<!--
  All EIPs must contain a section that discusses the security implications/considerations relevant to the proposed change. Include information that might be important for security discussions, surfaces risks and can be used throughout the life cycle of the proposal. For example, include security-relevant design decisions, concerns, important discussions, implementation-specific guidance and pitfalls, an outline of threats and risks and how they are being addressed. EIP submissions missing the "Security Considerations" section will be rejected. An EIP cannot proceed to status "Final" without a Security Considerations discussion deemed sufficient by the reviewers.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

A malicious RPC endpoint can serve false or misleading information to a user. It is therefore critical that MESC-related tooling comes from a trustworthy source.

## Copyright

Copyright and related rights waived via [CC0](../LICENSE.md).
