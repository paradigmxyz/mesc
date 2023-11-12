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

This is a specification of how EVM tools can configure their RPC endpoints.

By following this specification, a user creates a single RPC configuration that can be used by all compliant tools in an OS-agnostic and language-agnostic way.

This approach allows specification of 1) multiple endpoints for multiple chains, 2) a default endpoint for each chain, and 3) a default chain.

## Motivation

<!--
  This section is optional.

  The motivation section should include a description of any nontrivial problems the EIP solves. It should not describe how the EIP solves those problems, unless it is not immediately obvious. It should not describe why the EIP should be made into a standard, unless it is not immediately obvious.

  With a few exceptions, external links are not allowed. If you feel that a particular resource would demonstrate a compelling case for your EIP, then save it as a printer-friendly PDF, put it in the assets folder, and link to that copy.

  TODO: Remove this comment before submitting
-->

Between mainnet, testnets, chainforks, rollups, and alternative L1's, modern EVM tools must manage the configuration of multiple RPC endpoints. This configuration process is not standardized across tools.

The most common approach for configuring RPC endpoint information is the `ETH_RPC_URL` environment variable (dapptools, forge, heimdall, checkthechain). However, this is not a formal standard nor is it used universally. This approach also can only specify a single provider for a single chain. It also cannot specify any provider metadata beyond the url.

Instead it would be desirable to have a solution that:
- allows using a single configuration across multiple tools
- allows specifying multiple providers for multiple chains
- allows selection of a default endpoint for each chain
- allows selection of a default chain
- allows using either environment variables or config files
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

This approach is built on three key-value schemas:

##### `RpcConfig` schema:

| key                 | value type               | description |
| ---                 | ---                      | --- |
| `schema`            | `str`                    | must equal the value `"MESC 1.0"`
| `default_network`   | `int \| None`            | chain_id of default network
| `default_endpoints` | `Mapping[str, str]`      | map of chain_id's to endpoint names
| `endpoints`         | `Mapping[str, Endpoint]` | map of endpoint names to endpoints
| `profiles`          | `Mapping[str, Profile]`  | map of profile names to profiles
| `global_extras`     | `Mapping[str, Any]`      | global metadata entires

##### `Endpoint` schema:

| key               | value type          | description |
| ---               | ---                 | --- |
| `url`             | `str`               | url of rpc endpoint, should include transport and port
| `chain_id`        | `int`               | chain id of network
| `endpoint_extras` | `Mapping[str, Any]` | endpoint metadata entries

##### `Profile` schema:

| key                 | value type               | description |
| ---                 | ---                      | --- |
| `default_network`   | `int \| None`            | chain_id of default network
| `default_endpoints` | `Mapping[str, str]`      | map of chain_id's to endpoint names

Requirements:
- All keys of `RpcConfig` and `Endpoint` are required. No additional keys should be present, except within `global_extras` and `endpoint_extras`.
- Every endpoint name specified in `RpcConfig.default_endpoints` must exist in `RpcConfig.endpoints`.
- These key-value structures can be represented simply in JSON and in most common programming languages. The `chain_id` keys of `default_endpoints` should be strings as required by JSON.

The `global_extras` and `endpoint_extras` fields allow for optional or idiosyncratic RPC metadata to be colocated with the core RPC config. Tools can choose to ignore these fields.
- Possible `global_extras` fields:
    - `conceal`: whether the tool should avoid casually revealing private RPC url's
    - other field names specific to a particular tool should be prefixed with `tool_name + '__'`
- Possible `endpoint_extras` fields:
    - `api_key`: `str` api key for endpoint
    - `rate_limit`: `float` rate limit for endpoint in units of requests per second
    - `method_rate_limits`: `Mapping` of `str` rpc method names to `float` rate limits 
    - `labels`: `list` of `str` labels for endpoint
    - other field names specific to a particular tool should be prefixed with `tool_name + '__'`

#### Environment

`RpcConfig` data is stored either in a JSON file or in an environmental variable.

To locate the configuration, this specification introduces 3 environment variables:
- `RPC_CONFIG_MODE`
- `RPC_CONFIG_PATH`
- `RPC_CONFIG_ENV`

The following resolution order is then used:
1. check `RPC_CONFIG_MODE`
    - if set to `"PATH"`, interpret file at `RPC_CONFIG_PATH` as JSON `RpcConfig` data
    - if set to `"ENV"`, interpret the contents of `RPC_CONFIG_ENV` as JSON `RpcConfig` data
    - if set to other nonempty value, raise error
    - if unset or empty, continue to (2)
2. check `RPC_CONFIG_PATH`
    - if set to an existing file, interpret as JSON `RpcConfig` data
    - if set to a nonexistent file, raise error
    - if unset or empty, continue to (3)
3. check `RPC_CONFIG_ENV`
    - if set to valid JSON, interpret as JSON `RpcConfig` data
    - if nonempty and set to invalid JSON, raise error
    - if unset or empty, MESC is not being used, continue to (4)
4. MESC standard not being used, can fallback `ETH_RPC_URL` or other solutions

#### Example `RpcConfig`

```json
{
    "schema": "MESC 1.0",
    "default_network": 1,
    "default_endpoints": {
        "1": "local_ethereum",
        "5": "local_goerli",
        "137": "llamanodes_polygon",
    },
    "endpoints": {
        "local_ethereum": {
            "url": "http://localhost:8545",
            "chain_id": 1,
            "endpoint_extras": {}
        },
        "local_goerli": {
            "url": "http://localhost:8546",
            "chain_id": 5,
            "endpoint_extras": {}
        },
        "llamanodes_ethereum": {
            "url": "https://eth.llamarpc.com",
            "chain_id": 1,
            "endpoint_extras": {}
        },
        "llamanodes_polygon": {
            "url": "https://polygon.llamarpc.com",
            "chain_id": 137,
            "endpoint_extras": {}
        },
    },
    "global_extras": {}
}
```

## Rationale

<!--
  The rationale fleshes out the specification by describing what motivated the design and why particular design decisions were made. It should describe alternate designs that were considered and related work, e.g. how the feature is supported in other languages.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

- `global_extras` and `endpoint_extras` allow extra information to be stored in the config without breaking the standard. This includes api keys, rate limits, and organizational labels. This information might be specific to idiosyncratic to each application.
- `Profile`s allow different defaults to be assigned to each tool or each mode of operation.
- Allowing RPC information to be configured using either a file or an environment variable allows optimal deployment patterns across a wide range of computing environments. Each also have their own advantages, e.g. file can be used with version control whereas environment variables can be changed more dynamically.


## Backwards Compatibility

<!--

  This section is optional.

  All EIPs that introduce backwards incompatibilities must include a section describing these incompatibilities and their severity. The EIP must explain how the author proposes to deal with these incompatibilities. EIP submissions without a sufficient backwards compatibility treatise may be rejected outright.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

No backward compatibility issues found.

MESC is an opt-in specification that only becomes activated when a user explicitly sets one or more of the environment variables listed above (`RPC_CONFIG_MODE`, `RPC_CONFIG_PATH`, or `RPC_CONFIG_ENV`). These variables are not currently used by any common EVM tools. Tools that use `ETH_RPC_URL` or other configuration approaches will continue working as before.

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

A minimal reference python implementation is included below. A more detailed implementation might include additional caching, validation, or querying functionality.

```python
from __future__ import annotations
import json
import os
from typing import Any, Mapping, TypedDict


class Endpoint(TypedDict):
    url: str
    chain_id: int
    endpoint_extras: Mapping[str, Any]


class Profile(TypedDict):
    default_network: int | None
    default_endpoints: Mapping[str, str]


class RpcConfig(TypedDict):
    schema: str
    default_network: int | None
    default_endpoints: Mapping[str, str]
    endpoints: Mapping[str, Endpoint]
    profiles: Mapping[str, Profile]
    global_extras: Mapping[str, Any]


def get_default_network(
    *,
    profile: str | None = None,
    require_profile: bool = False,
) -> int | None:
    config = read_config_data()
    if profile and (require_profile or profile in config['profiles']):
        return config['profiles'][profile]['default_network']
    else:
        return config['default_network']


def get_default_endpoint(
    chain_id: int,
    *,
    profile: str | None = None,
    require_profile: bool = False,
) -> Endpoint:
    config = read_config_data()
    if profile and (require_profile or profile in config['profiles']):
        default_endpoints = config['profiles'][profile]['default_endpoints']
    else:
        default_endpoints = config['default_endpoints']

    name = default_endpoints.get(str(chain_id))
    if name is None:
        raise Exception('missing endpoint for chain_id: ' + str(chain_id))

    return get_endpoint(name, config=config)


def get_endpoint(name: str, *, config: RpcConfig) -> Endpoint:
    for endpoint_name, endpoint in read_config_data()['endpoints'].items():
        if endpoint_name == name:
            return endpoint
    else:
        raise Exception('missing endpoint: ' + str(name))


def read_config_data() -> RpcConfig:
    mode = os.environ.get('RPC_CONFIG_MODE')
    if mode == 'PATH':
        return read_file_config()
    elif mode == 'ENV':
        return read_env_config()
    elif mode not in ['', None]:
        raise Exception('invalid mode: ' + str(mode))
    elif os.environ.get('RPC_CONFIG_PATH') not in ['', None]:
        return read_file_config()
    elif os.environ.get('RPC_CONFIG_ENV') not in ['', None]:
        return read_env_config()
    else:
        raise Exception('config not specified')


def read_env_config() -> RpcConfig:
    return json.loads(os.environ.get('RPC_CONFIG_ENV'))


def read_file_config() -> RpcConfig:
    with open(os.environ.get('RPC_CONFIG_PATH'), 'r') as f:
        return json.load(f)

```

## Security Considerations

<!--
  All EIPs must contain a section that discusses the security implications/considerations relevant to the proposed change. Include information that might be important for security discussions, surfaces risks and can be used throughout the life cycle of the proposal. For example, include security-relevant design decisions, concerns, important discussions, implementation-specific guidance and pitfalls, an outline of threats and risks and how they are being addressed. EIP submissions missing the "Security Considerations" section will be rejected. An EIP cannot proceed to status "Final" without a Security Considerations discussion deemed sufficient by the reviewers.

  The current placeholder is acceptable for a draft.

  TODO: Remove this comment before submitting
-->

A malicious RPC endpoint can serve false or misleading information to a user. It is therefore critical that MESC-related tooling comes from a trustworthy source.

## Copyright

Copyright and related rights waived via [CC0](../LICENSE.md).
