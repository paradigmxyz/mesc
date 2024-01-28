# Metadata

In a MESC configuration, the `global_metadata`, `profile_metadata`, and `endpoint_metadata` fields allow for optional or idiosyncratic metadata to be stored alongside the core RPC data.

Tools using MESC can choose to ignore these fields. 


## Metadata fields

Contents of MESC metadata is only loosely specified. This is intentional to allow for some degree of future-proofing. It enables MESC to handle use-cases that are either unanticipated or highly-specific to a given tool.


## Examples

**Endpoint metadata**

| key | value type | description | examples |
| --- | ---        | ---         | ---      |
| `rate_limit_rps`        | `int` or `float`               | ratelimit in requests per second                        | `250`  |
| `rate_limit_cups`       | `int` or `float`               | ratelimit in CUPS                                       | `1000` |
| `rate_limit_per_method` | `Mapping[str, int or float]` | ratelimit in RPS for each method                        | `{"trace_block": 200}` |
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

