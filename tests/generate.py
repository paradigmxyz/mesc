#!/usr/bin/env python3

from __future__ import annotations

import copy
from typing import Any, TypeVar, Sequence, Union
import typing

if typing.TYPE_CHECKING:
    from mesc import RpcConfig, Profile, Endpoint
    from mesc.types import MescQuery

blank_config: RpcConfig = {
    "mesc_version": "MESC 1.0",
    "default_endpoint": None,
    "network_defaults": {},
    "network_names": {},
    "endpoints": {},
    "profiles": {},
    "global_metadata": {},
}

blank_endpoint: Endpoint = {
    "name": "name",
    "url": "",
    "chain_id": None,
    "endpoint_metadata": {},
}

blank_profile: Profile = {
    "name": "name",
    "default_endpoint": None,
    "network_defaults": {},
    "profile_metadata": {},
    "use_mesc": True,
}

full_config: RpcConfig = {
    "mesc_version": "MESC 1.0",
    "default_endpoint": "local_ethereum",
    "network_defaults": {
        "1": "local_ethereum",
        "5": "local_goerli",
        "10": "local_optimism",
    },
    "network_names": {
        "testnet": "5",
    },
    "endpoints": {
        "local_ethereum": {
            "name": "local_ethereum",
            "url": "localhost:8545",
            "chain_id": "1",
            "endpoint_metadata": {},
        },
        "local_goerli": {
            "name": "local_goerli",
            "url": "localhost:8546",
            "chain_id": "5",
            "endpoint_metadata": {"ecosystem": "ethereum"},
        },
        "local_optimism": {
            "name": "local_optimism",
            "url": "localhost:8547",
            "chain_id": "10",
            "endpoint_metadata": {},
        },
        "llamanodes_ethereum": {
            "name": "llamanodes_ethereum",
            "url": "https://eth.llamarpc.com",
            "chain_id": "1",
            "endpoint_metadata": {},
        },
        "llamanodes_optimism": {
            "name": "llamanodes_optimism",
            "url": "https://optimism.llamarpc.com",
            "chain_id": "10",
            "endpoint_metadata": {},
        },
    },
    "profiles": {
        "abc": {
            "name": "abc",
            "default_endpoint": None,
            "network_defaults": {},
            "profile_metadata": {},
            "use_mesc": True,
        },
        "xyz": {
            "name": "xyz",
            "default_endpoint": "llamanodes_ethereum",
            "network_defaults": {
                "1": "llamanodes_ethereum",
                "10": "llamanodes_optimism",
            },
            "profile_metadata": {
                "sample_key": "profile_value",
            },
            "use_mesc": True,
        },
        "not_using_mesc": {
            "name": "not_using_mesc",
            "default_endpoint": None,
            "network_defaults": {},
            "profile_metadata": {},
            "use_mesc": False,
        },
    },
    "global_metadata": {
        "api_keys": {
            "etherscan": "abc123",
        },
        "sample_key": "global_value",
    },
}

known_networks = {
    "1": "ethereum",
    "5": "goerli",
    "10": "optimism",
    "137": "polygon",
}

# tests are in form [test_name, env, config, query, result, should_succeed]
Test = tuple[
    str,
    dict[str, str],
    RpcConfig,
    Union[None, MescQuery],
    Any,
    bool,
]


def generate_tests() -> list[Test]:
    # for tests that should fail, the query field is set to None
    generators = [
        create_basic_query_tests,
        create_override_tests,
        create_invalid_query_tests,
        create_invalid_config_tests,
    ]

    tests = [test for generator in generators for test in generator()]

    # for tests that query with null profile, also query with non-existent profile
    for test in list(tests):
        query = test[3]
        if (
            query is not None
            and "profile" in query.get("fields", {})
            and query["fields"].get("profile") is None
        ):
            new_test = copy.deepcopy(test)
            new_test[3]["fields"]["profile"] = "unknown_profile"  # type: ignore

            new_test = (new_test[0] + ", unknown_profile",) + new_test[1:]
            tests.append(new_test)

    # every query should return None or empty when a profile has use_mesc=True
    for test in list(tests):
        query = test[3]
        target_output = test[4]
        if (
            query is not None
            and "profile" in query.get("fields", {})
            and query["fields"].get("profile") is None
            and target_output is not None
        ):
            new_test = copy.deepcopy(test)
            new_test[3]["fields"]["profile"] = "not_using_mesc"  # type: ignore
            if query["query_type"] == "global_metadata":
                expected: dict | None = {}
            else:
                expected = None
            new_test = new_test[0:4] + (expected,) + new_test[5:]
            new_test = (new_test[0] + ", profile not_using_mesc",) + new_test[1:]
            tests.append(new_test)

    return tests


T = TypeVar("T")


def set_path_value(data: T, path: Sequence[str], value: Any) -> T:
    if len(path) == 1:
        data[path[0]] = value  # type: ignore
    else:
        set_path_value(data[path[0]], path[1:], value)  # type: ignore
    return data


def set_paths_values(data: T, pairs: Sequence[tuple[Sequence[str], Any]]) -> T:
    for path, value in pairs:
        set_path_value(data, path, value)
    return data


def create_basic_query_tests() -> list[Test]:
    tests: list[Test] = []

    # default endpoint queries
    tests += [
        (
            "default endpoint",
            {},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "default endpoint null",
            {},
            blank_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            None,
            True,
        ),
        (
            "default endpoint null profile",
            {},
            blank_config,
            {"query_type": "default_endpoint", "fields": {"profile": "abc"}},
            None,
            True,
        ),
        (
            "default endpoint profile",
            {},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": "xyz"}},
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
    ]

    # get endpoint by name
    tests += [
        (
            "get endpoint by name",
            {},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "get endpoint by name",
            {},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "llamanodes_optimism"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
            True,
        ),
    ]

    # get endpoint by network
    tests += [
        (
            "get endpoint by network",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by network",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "5", "profile": None},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "query endpoint chain_id by int",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": 1, "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by network",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "137", "profile": None},
            },
            None,
            True,
        ),
        (
            "get endpoint by network profile fallback",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by network profile",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by network dne",
            {},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "137", "profile": "abc"},
            },
            None,
            True,
        ),
    ]

    # get endpoint by user query
    tests += [
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "local_ethereum", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "llamanodes_ethereum", "profile": None},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "1", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "10", "profile": None},
            },
            full_config["endpoints"]["local_optimism"],
            True,
        ),
        (
            "get endpoint by user query, network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "ethereum", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, custom network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "testnet", "profile": None},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "get endpoint by user query, dne",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "this_query_has_no_results", "profile": None},
            },
            None,
            True,
        ),
    ]

    # get endpoint by user query, with default profile
    tests += [
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "local_ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "llamanodes_ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "1", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "10", "profile": "abc"},
            },
            full_config["endpoints"]["local_optimism"],
            True,
        ),
        (
            "get endpoint by user query, network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, custom network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "testnet", "profile": "abc"},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "get endpoint by user query, dne",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "this_query_has_no_results", "profile": "abc"},
            },
            None,
            True,
        ),
    ]

    # get endpoint by user query, profile full
    tests += [
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "local_ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, endpoint name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "llamanodes_ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, chain id",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "10", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
            True,
        ),
        (
            "get endpoint by user query, network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "get endpoint by user query, custom network name",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "testnet", "profile": "xyz"},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "get endpoint by user query, dne",
            {},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "this_query_has_no_results", "profile": "xyz"},
            },
            None,
            True,
        ),
    ]

    # test querying by known network names
    for chain_id, network_name in known_networks.items():
        # with a default endpoint
        network_endpoint = copy.deepcopy(blank_endpoint)
        network_endpoint["chain_id"] = chain_id
        network_config = copy.deepcopy(full_config)
        network_config["endpoints"][network_endpoint["name"]] = network_endpoint
        network_config["network_defaults"][chain_id] = network_endpoint["name"]
        test: Test = (
            "test querying " + network_name + " by name",
            {},
            network_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": network_name, "profile": None},
            },
            network_endpoint,
            True,
        )
        tests.append(test)

        # without a default endpoint
        network_config = copy.deepcopy(full_config)
        if chain_id in network_config["network_defaults"]:
            del network_config["network_defaults"][chain_id]
        test_without: Test = (
            "test querying " + network_name + " by name",
            {},
            network_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": network_name, "profile": None},
            },
            None,
            True,
        )
        tests.append(test_without)

    # test multi-endpoint queries
    tests += [
        (
            "fuzzy name query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"name_contains": "local"}},
            [
                full_config["endpoints"]["local_ethereum"],
                full_config["endpoints"]["local_goerli"],
                full_config["endpoints"]["local_optimism"],
            ],
            True,
        ),
        (
            "empty fuzzy name query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"name_contains": "yyyyyy"}},
            [],
            True,
        ),
        (
            "fuzzy url query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"url_contains": "llama"}},
            [
                full_config["endpoints"]["llamanodes_ethereum"],
                full_config["endpoints"]["llamanodes_optimism"],
            ],
            True,
        ),
        (
            "fuzzy url query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"url_contains": "yyyyyy"}},
            [],
            True,
        ),
        (
            "chain_id query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"chain_id": "1"}},
            [
                full_config["endpoints"]["local_ethereum"],
                full_config["endpoints"]["llamanodes_ethereum"],
            ],
            True,
        ),
        (
            "chain_id int query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"chain_id": 1}},
            [
                full_config["endpoints"]["local_ethereum"],
                full_config["endpoints"]["llamanodes_ethereum"],
            ],
            True,
        ),
        (
            "empty chain_id query",
            {},
            full_config,
            {"query_type": "multi_endpoint", "fields": {"chain_id": "1111"}},
            [],
            True,
        ),
    ]

    # test fetching metadata
    tests += [
        (
            "get full global metadata, no profile",
            {},
            full_config,
            {"query_type": "global_metadata", "fields": {"profile": None}},
            full_config["global_metadata"],
            True,
        ),
        (
            "get full global metadata, irrelevant profile",
            {},
            full_config,
            {"query_type": "global_metadata", "fields": {"profile": "abc"}},
            full_config["global_metadata"],
            True,
        ),
        (
            "get full global metadata, merged with profile",
            {},
            full_config,
            {"query_type": "global_metadata", "fields": {"profile": "xyz"}},
            dict(
                full_config["global_metadata"],
                **full_config["profiles"]["xyz"]["profile_metadata"],
            ),
            True,
        ),
    ]

    return tests


def create_invalid_query_tests() -> list[Test]:
    invalid_queries: Sequence[tuple[str, Any]] = [
        (
            "query default endpoint with int profile",
            {"query_type": "default_endpoint", "fields": {"profile": 1}},
        ),
        (
            "query endpoint name by null",
            {"query_type": "endpoint_by_name", "fields": {"name": None}},
        ),
        (
            "query endpoint name by int",
            {"query_type": "endpoint_by_name", "fields": {"name": 1}},
        ),
        (
            "query endpoint chain_id by null",
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": None, "profile": None},
            },
        ),
        (
            "query endpoint chain_id by int profile",
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": 1},
            },
        ),
        (
            "query user input by null",
            {
                "query_type": "user_input",
                "fields": {"user_input": None, "profile": None},
            },
        ),
        (
            "query user input by bool",
            {
                "query_type": "user_input",
                "fields": {"user_input": True, "profile": None},
            },
        ),
        (
            "query user input by int profile",
            {"query_type": "user_input", "fields": {"user_input": True, "profile": 1}},
        ),
    ]

    tests = []
    for test_name, query in invalid_queries:
        test: Test = (
            test_name,
            {},
            full_config,
            query,
            None,
            False,
        )
        tests.append(test)

    return tests


def create_invalid_config_tests() -> list[Test]:
    tests: list[Test] = []

    # unknown default endpoint
    config = copy.deepcopy(blank_config)
    config["default_endpoint"] = "random_unknown"
    tests.append(
        (
            "unknown default endpoint",
            {},
            config,
            None,
            None,
            False,
        )
    )

    # unknown network defaults
    config = copy.deepcopy(full_config)
    config["network_defaults"]["10"] = "random_unknown"
    tests.append(
        (
            "unknown network defaults",
            {},
            config,
            None,
            None,
            False,
        ),
    )

    # incorrect types tests
    invalid_type_tests = [
        (
            "mesc version is null",
            set_path_value(copy.deepcopy(full_config), ["mesc_version"], None),
        ),
        (
            "mesc version is int",
            set_path_value(copy.deepcopy(full_config), ["mesc_version"], 1),
        ),
        (
            "default endpoint is int",
            set_path_value(copy.deepcopy(full_config), ["default_endpoint"], 1),
        ),
        (
            "default endpoint is list",
            set_path_value(copy.deepcopy(full_config), ["default_endpoint"], [1]),
        ),
        (
            "endpoint name is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["endpoints", "local_ethereum", "name"],
                None,
            ),
        ),
        (
            "endpoint name is int",
            set_path_value(
                copy.deepcopy(full_config), ["endpoints", "local_ethereum", "name"], 1
            ),
        ),
        (
            "endpoint url is null",
            set_path_value(
                copy.deepcopy(full_config), ["endpoints", "local_ethereum", "url"], None
            ),
        ),
        (
            "endpoint url is int",
            set_path_value(
                copy.deepcopy(full_config), ["endpoints", "local_ethereum", "url"], 1
            ),
        ),
        (
            "endpoint chain_id is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["endpoints", "local_ethereum", "chain_id"],
                None,
            ),
        ),
        (
            "endpoint chain_id is int",
            set_path_value(
                copy.deepcopy(full_config),
                ["endpoints", "local_ethereum", "chain_id"],
                1,
            ),
        ),
        (
            "endpoint metadata is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["endpoints", "local_ethereum", "endpoint_metadata"],
                None,
            ),
        ),
        (
            "endpoint metadata is int",
            set_path_value(
                copy.deepcopy(full_config),
                ["endpoints", "local_ethereum", "endpoint_metadata"],
                None,
            ),
        ),
        (
            "network_defaults is null",
            set_path_value(copy.deepcopy(full_config), ["network_defaults"], None),
        ),
        (
            "network_defaults is int",
            set_path_value(copy.deepcopy(full_config), ["nework_defaults"], 1),
        ),
        (
            "network_defaults entry is null",
            set_path_value(copy.deepcopy(full_config), ["network_defaults", "1"], None),
        ),
        (
            "network_defaults entry is int",
            set_path_value(copy.deepcopy(full_config), ["network_defaults", "1"], 1),
        ),
        (
            "network_names is null",
            set_path_value(copy.deepcopy(full_config), ["network_names"], None),
        ),
        (
            "network_names is list",
            set_path_value(copy.deepcopy(full_config), ["network_names"], []),
        ),
        (
            "network_names entry is int",
            set_path_value(
                copy.deepcopy(full_config), ["network_names", "some_name"], 1
            ),
        ),
        (
            "network_names entry is null",
            set_path_value(
                copy.deepcopy(full_config), ["network_names", "some_name"], None
            ),
        ),
        (
            "profiles is null",
            set_path_value(copy.deepcopy(full_config), ["profiles"], None),
        ),
        (
            "profiles is list",
            set_path_value(copy.deepcopy(full_config), ["profiles"], []),
        ),
        (
            "profile name is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["profiles", "some_profile"],
                set_path_value(copy.deepcopy(blank_profile), ["name"], None),
            ),
        ),
        (
            "profile name is int",
            set_path_value(
                copy.deepcopy(full_config),
                ["profiles", "some_profile"],
                set_path_value(copy.deepcopy(blank_profile), ["name"], 1),
            ),
        ),
        (
            "profile default_endpoint is int",
            set_path_value(
                copy.deepcopy(full_config), ["profiles", "xyz", "default_endpoint"], 1
            ),
        ),
        (
            "profile network_defaults is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["profiles", "xyz", "network_defaults"],
                None,
            ),
        ),
        (
            "profile network_defaults entry is int",
            set_path_value(
                copy.deepcopy(full_config),
                ["profiles", "xyz", "network_defaults", "some_profile"],
                1,
            ),
        ),
        (
            "profile network_defaults entry is null",
            set_path_value(
                copy.deepcopy(full_config),
                ["profiles", "xyz", "network_defaults", "some_profile"],
                None,
            ),
        ),
        (
            "global metadata is int",
            set_path_value(copy.deepcopy(full_config), ["global_metadata"], 1),
        ),
        (
            "global metadata is null",
            set_path_value(copy.deepcopy(full_config), ["global_metadata"], None),
        ),
    ]
    for test_name, config in invalid_type_tests:
        test: Test = (
            test_name,
            {},
            config,
            None,
            None,
            False,
        )
        tests.append(test)

    # missing field tests
    for field in blank_config.keys():
        invalid_config = copy.deepcopy(full_config)
        del invalid_config[field]  # type: ignore
        tests.append(
            (
                "missing global field: " + field,
                {},
                invalid_config,
                None,
                None,
                False,
            )
        )
    for field in blank_endpoint.keys():
        invalid_config = copy.deepcopy(full_config)
        invalid_config["endpoints"]["name"] = copy.deepcopy(blank_endpoint)
        del invalid_config["endpoints"]["name"][field]  # type: ignore
        tests.append(
            (
                "missing endpoint field: " + field,
                {},
                invalid_config,
                None,
                None,
                False,
            )
        )
    for field in blank_profile.keys():
        invalid_config = copy.deepcopy(full_config)
        name = blank_profile["name"]
        invalid_config["profiles"][name] = copy.deepcopy(blank_profile)
        del invalid_config["profiles"][name][field]  # type: ignore
        tests.append(
            (
                "missing profile field: " + field,
                {},
                invalid_config,
                None,
                None,
                False,
            )
        )

    # invalid env setup
    tests += [
        (
            "invalid config mode: ALL",
            {"MESC_MODE": "ALL"},
            full_config,
            None,
            None,
            False,
        ),
        (
            "invalid config mode: path",
            {"MESC_MODE": "path"},
            full_config,
            None,
            None,
            False,
        ),
        (
            "invalid MESC_PATH: DNE",
            {"MESC_PATH": "/this/path/dne.json"},
            full_config,
            None,
            None,
            False,
        ),
        (
            "invalid MESC_ENV",
            {"MESC_ENV": "{invalid}"},
            full_config,
            None,
            None,
            False,
        ),
    ]

    return tests


def create_override_tests() -> list[Test]:
    tests: list[Test] = []

    # override default endpoint
    tests += [
        (
            "override endpoint",
            {"MESC_DEFAULT_ENDPOINT": "local_goerli"},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "override endpoint blank",
            {"MESC_DEFAULT_ENDPOINT": ""},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
    ]

    # override network defaults
    tests += [
        (
            "override network defaults, change network default",
            {"MESC_NETWORK_DEFAULTS": "1=llamanodes_ethereum"},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": None},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
        (
            "override network defaults, remove network default",
            {"MESC_NETWORK_DEFAULTS": "1=llamanodes_ethereum 5="},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "5", "profile": None},
            },
            None,
            True,
        ),
        (
            "override network defaults blank",
            {"MESC_NETWORK_DEFAULTS": ""},
            full_config,
            {
                "query_type": "default_endpoint",
                "fields": {"profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
    ]

    # override network names
    tests += [
        (
            "override network names, new network no endpoint",
            {"MESC_NETWORK_NAMES": "xyz=123"},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "override network names, new network with endpoint",
            {"MESC_NETWORK_NAMES": "xyz=123"},
            set_paths_values(
                copy.deepcopy(full_config),
                [
                    (["endpoints", "name"], dict(blank_endpoint, chain_id="123")),
                    (["network_defaults", "123"], "name"),
                ],
            ),
            {
                "query_type": "user_input",
                "fields": {"user_input": "123", "profile": None},
            },
            dict(blank_endpoint, chain_id="123"),
            True,
        ),
        (
            "override network names, rename existing network",
            {"MESC_NETWORK_NAMES": "xyz=1"},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "xyz", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "override network names blank",
            {"MESC_NETWORK_NAMES": ""},
            full_config,
            {
                "query_type": "user_input",
                "fields": {"user_input": "testnet", "profile": None},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
    ]

    # override endpoints
    tests += [
        (
            "override endpoints, change existing url",
            {"MESC_ENDPOINTS": "local_ethereum=other_url.com"},
            full_config,
            {"query_type": "endpoint_by_name", "fields": {"name": "local_ethereum"}},
            set_path_value(
                copy.deepcopy(full_config["endpoints"]["local_ethereum"]),
                ["url"],
                "other_url.com",
            ),
            True,
        ),
        # (
        #     "override endpoints, change existing chain_id and url",
        #     {"MESC_ENDPOINTS": "local_ethereum:2=other_url.com"},
        #     full_config,
        #     {"query_type": "endpoint_by_name", "fields": {"name": "local_ethereum"}},
        #     set_paths_values(
        #         copy.deepcopy(full_config["endpoints"]["local_ethereum"]),
        #         [(["url"], "other_url.com"), (["chain_id"], "2")],
        #     ),
        #     True,
        # ),
        (
            "override endpoints, add new endpoint with url",
            {"MESC_ENDPOINTS": "new_endpoint=other_url.com"},
            full_config,
            {"query_type": "endpoint_by_name", "fields": {"name": "new_endpoint"}},
            {
                "name": "new_endpoint",
                "chain_id": None,
                "url": "other_url.com",
                "endpoint_metadata": {},
            },
            True,
        ),
        (
            "override endpoints, add new endpoint with url and chain_id",
            {"MESC_ENDPOINTS": "new_endpoint:2=other_url.com"},
            full_config,
            {"query_type": "endpoint_by_name", "fields": {"name": "new_endpoint"}},
            {
                "name": "new_endpoint",
                "chain_id": "2",
                "url": "other_url.com",
                "endpoint_metadata": {},
            },
            True,
        ),
        (
            "override endpoints, add new nameless endpoint",
            {"MESC_ENDPOINTS": "other_url.com"},
            full_config,
            {"query_type": "endpoint_by_name", "fields": {"name": "other_url"}},
            {
                "name": "other_url",
                "chain_id": None,
                "url": "other_url.com",
                "endpoint_metadata": {},
            },
            True,
        ),
        (
            "override endpoints blank",
            {"MESC_ENDPOINTS": ""},
            full_config,
            {"query_type": "multi_endpoint", "fields": {}},
            list(full_config["endpoints"].values()),
            True,
        ),
    ]

    # override profiles
    tests += [
        (
            "override profiles, create new profile with default_endpoint",
            {"MESC_PROFILES": "jkl.default_endpoint=local_optimism"},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": "jkl"}},
            full_config["endpoints"]["local_optimism"],
            True,
        ),
        (
            "override profiles, create new profile with network_defaults",
            {"MESC_PROFILES": "jkl.network_defaults.10=llamanodes_optimism"},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "10", "profile": "jkl"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
            True,
        ),
        (
            "override profiles, create new profile with default_endpoint and network_defaults",
            {
                "MESC_PROFILES": "jkl.default_endpoint=local_optimism jkl.network_defaults.10=llamanodes_optimism"
            },
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": "jkl"}},
            full_config["endpoints"]["local_optimism"],
            True,
        ),
        (
            "override profiles, create new profile with default_endpoint and network_defaults",
            {
                "MESC_PROFILES": "jkl.default_endpoint=local_optimism jkl.network_defaults.10=llamanodes_optimism"
            },
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "10", "profile": "jkl"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
            True,
        ),
        (
            "override profiles, edit existing default_endpoint",
            {"MESC_PROFILES": "xyz.default_endpoint=local_goerli"},
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": "xyz"}},
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "override profiles, edit existing network default",
            {"MESC_PROFILES": "xyz.network_defaults.1=local_ethereum"},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["local_ethereum"],
            True,
        ),
        (
            "override profiles blank",
            {"MESC_PROFILES": ""},
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
            True,
        ),
    ]

    # override global metadata
    tests += [
        (
            "override global metadata, change existing key",
            {"MESC_GLOBAL_METADATA": '{"api_keys": {"etherscan": "new_key"}}'},
            full_config,
            {"query_type": "global_metadata", "fields": {}},
            dict(full_config["global_metadata"], api_keys={"etherscan": "new_key"}),
            True,
        ),
        (
            "override global metadata, add new key",
            {"MESC_GLOBAL_METADATA": '{"some_new_key": "value"}'},
            full_config,
            {"query_type": "global_metadata", "fields": {}},
            dict(full_config["global_metadata"], some_new_key="value"),
            True,
        ),
        (
            "override global metadata blank",
            {"MESC_GLOBAL_METADATA": ""},
            full_config,
            {"query_type": "global_metadata", "fields": {}},
            full_config["global_metadata"],
            True,
        ),
    ]

    # override endpoint metadata
    tests += [
        (
            "override endpoint metadata, add new key",
            {"MESC_ENDPOINT_METADATA": '{"local_goerli": {"password": "abc123"}}'},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            set_path_value(
                copy.deepcopy(full_config["endpoints"]["local_goerli"]),
                ["endpoint_metadata", "password"],
                "abc123",
            ),
            True,
        ),
        (
            "override endpoint metadata, change existing key",
            {"MESC_ENDPOINT_METADATA": '{"local_goerli": {"ecosystem": "polygon"}}'},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            set_path_value(
                copy.deepcopy(full_config["endpoints"]["local_goerli"]),
                ["endpoint_metadata", "ecosystem"],
                "polygon",
            ),
            True,
        ),
        (
            "override endpoint metadata, blank",
            {"MESC_ENDPOINT_METADATA": ""},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
        (
            "override endpoint metadata, blank entry",
            {"MESC_ENDPOINT_METADATA": '{"local_goerli": {}}'},
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            full_config["endpoints"]["local_goerli"],
            True,
        ),
    ]

    # invalid overrides tests
    pass

    return tests
