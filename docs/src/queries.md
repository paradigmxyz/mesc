# Querying MESC data

Each MESC implementation provides the same set of 7 functions for querying MESC data.

The behavior of these functions is the same across languages.

| function | output type | description | example call |
| --- | --- | --- | --- |
| `is_mesc_enabled()`         | `bool`               | return whether MESC is enabled | `is_mesc_enabled()` |
| `get_default_endpoint()`    | `Endpoint` or `None` | get default MESC endpoint | `get_default_endpoint(profile='xyz')` |
| `get_endpoint_by_network()` | `Endpoint` or `None` | get default endpoint for network | `get_endpoint_by_network(5, profile='xyz')` |
| `get_endpoint_by_name()`    | `Endpoint` or `None` | get endpoint by name | `get_endpoint_by_name('local_goerli')` |
| `get_endpoint_by_query()`   | `Endpoint` or `None` | get endpoint for user input query | `get_endpoint_by_query(user_str, profile='xyz')` |
| `find_endpoints()`          | `Sequence[Endpoint]` | find endpoint that match input criteria | `find_endpoints(chain_id=5)` |
| `get_global_metadata()`     | `Mapping[str, Any]`  | get non-endpoint metadata | `get_global_metadata(profile='xyz')` |

The `profile` argument is optional for each function (it allows users to customize the settings for each tool, see [Profiles](./profiles.md) for details).

