#!/usr/bin/env python3

import copy
import os

from mesc import RpcConfig, Profile, Endpoint
from mesc.types import EndpointQuery, MultiEndpointQuery

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
}

full_config: RpcConfig = {
    "mesc_version": "MESC 1.0",
    "default_endpoint": 'local_ethereum',
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
            "endpoint_metadata": {},
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
        },
        "xyz": {
            "name": "xyz",
            "default_endpoint": "llamanodes_ethereum",
            "network_defaults": {
                "1": "llamanodes_ethereum",
                "10": "llamanodes_optimism",
            },
        },
    },
    "global_metadata": {},
}


def create_basic_query_tests() -> (
    list[tuple[str, RpcConfig, EndpointQuery, Endpoint | None]]
):
    tests: list[tuple[str, RpcConfig, EndpointQuery, Endpoint | None]] = []

    # default endpoint queries
    tests += [
        (
            "default endpoint",
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "default endpoint null",
            blank_config,
            {"query_type": "default_endpoint", "fields": {"profile": None}},
            None,
        ),
        (
            "default endpoint null profile",
            blank_config,
            {"query_type": "default_endpoint", "fields": {"profile": "abc"}},
            None,
        ),
        (
            "default endpoint profile",
            full_config,
            {"query_type": "default_endpoint", "fields": {"profile": "xyz"}},
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
    ]

    # get endpoint by name
    tests += [
        (
            "get endpoint by name",
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "local_goerli"},
            },
            full_config["endpoints"]["local_goerli"],
        ),
        (
            "get endpoint by name",
            full_config,
            {
                "query_type": "endpoint_by_name",
                "fields": {"name": "llamanodes_optimism"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
        ),
    ]

    # get endpoint by network
    tests += [
        (
            "get endpoint by network",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by network",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "5", "profile": None},
            },
            full_config["endpoints"]["local_goerli"],
        ),
        (
            "get endpoint by network",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "137", "profile": None},
            },
            None,
        ),
        (
            "get endpoint by network profile fallback",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by network profile",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "get endpoint by network dne",
            full_config,
            {
                "query_type": "endpoint_by_network",
                "fields": {"chain_id": "137", "profile": "abc"},
            },
            None,
        ),
    ]

    # get endpoint by user query
    tests += [
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "local_ethereum", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "llamanodes_ethereum", "profile": None},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "1", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "10", "profile": None},
            },
            full_config["endpoints"]["local_optimism"],
        ),
        (
            "get endpoint by user query, network name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "ethereum", "profile": None},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, custom network name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "testnet", "profile": None},
            },
            full_config["endpoints"]["local_goerli"],
        ),
    ]

    # get endpoint by user query, profile blank
    tests += [
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "local_ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "llamanodes_ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "1", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "10", "profile": "abc"},
            },
            full_config["endpoints"]["local_optimism"],
        ),
        (
            "get endpoint by user query, network name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "ethereum", "profile": "abc"},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, custom network name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "testnet", "profile": "abc"},
            },
            full_config["endpoints"]["local_goerli"],
        ),
    ]

    # get endpoint by user query, profile full
    tests += [
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "local_ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["local_ethereum"],
        ),
        (
            "get endpoint by user query, endpoint name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "llamanodes_ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "1", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "get endpoint by user query, chain id",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "10", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_optimism"],
        ),
        (
            "get endpoint by user query, network name",
            full_config,
            {
                "query_type": "user_input_query",
                "fields": {"user_input": "ethereum", "profile": "xyz"},
            },
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        # (
        #     "get endpoint by user query, custom network name",
        #     full_config,
        #     {"query_type": "user_input_query", "fields": {"user_input": "testnet", "profile": "xyz"}},
        #     full_config['endpoints']['llamanodes_goerli'],
        # ),
    ]

    return tests


def create_find_endpoints_tests() -> (
    list[tuple[str, RpcConfig, MultiEndpointQuery, list[Endpoint]]]
):
    tests: list[tuple[str, RpcConfig, MultiEndpointQuery, list[Endpoint]]] = []

    return tests


def create_invalid_config_tests():
    invalid_configs = []

    # unknown default endpoint
    config = copy.deepcopy(blank_config)
    config["default_endpoint"] = "random_unknown"
    invalid_configs.append(("unknown default endpoint", config))

    # get missing endpoint by name

    # unknown network defaults

    # use incorrect types

    # missing field tests
    for field in blank_config.keys():
        invalid_config = copy.deepcopy(config)
        del invalid_config[field]
        invalid_configs.append(("missing global field: " + field, invalid_config))
    for field in blank_endpoint.keys():
        invalid_config = copy.deepcopy(config)
        invalid_config["endpoints"]["name"] = copy.deepcopy(blank_endpoint)
        del invalid_config["endpoints"]["name"][field]
        invalid_configs.append(("missing endpoint field: " + field, invalid_config))
    for field in blank_profile.keys():
        invalid_config = copy.deepcopy(config)
        invalid_config["profile"] = copy.deepcopy(blank_profile)
        del invalid_config["profile"][field]
        invalid_configs.append(("missing profile field: " + field, invalid_config))

    # invalid overrides tests

    return invalid_configs


def create_invalid_env_tests():
    # invalid config mode
    pass

    # invalid path
    pass

    # invalid env json
    pass


def create_override_tests():
    tests: list[
        tuple[str, dict[str, str], RpcConfig, EndpointQuery, Endpoint | list[Endpoint]]
    ] = []

    # override default endpoint
    tests += [
        (
            "override endpoint",
            {"MESC_DEFAULT_ENDPOINT": "local_goerli"},
            full_config,
            {"type": "default_endpoint", "fields": {}},
            full_config["endpoints"]["local_goerli"],
        ),
        (
            "override endpoint blank",
            {"MESC_DEFAULT_ENDPOINT": ""},
            full_config,
            {"type": "default_endpoint", "fields": {}},
            full_config["endpoints"]["local_ethereum"],
        ),
    ]

    # override network defaults
    tests += [
        (
            "override network defaults, change network default",
            {"MESC_NETWORK_DEFAULTS": "1=llamanodes_ethereum 5="},
            full_config,
            {"type": "endpoint_by_network", "fields": {"network": "1"}},
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
        (
            "override network defaults, remove network default",
            {"MESC_NETWORK_DEFAULTS": "1=llamanodes_ethereum 5="},
            full_config,
            {"type": "endpoint_by_network", "fields": {"network": "5"}},
            None,
        ),
        (
            "override network defaults, blank",
            {"MESC_NETWORK_DEFAULTS": ""},
            full_config,
            {"type": "default_endpoint", "fields": {"network": 1}},
            full_config["endpoints"]["local_ethereum"],
        ),
    ]

    # override network names
    # 'override network names, new network no endpoint',
    # 'override network names, new network with endpoint',
    # 'override network names, rename existing network',
    tests += [
        (
            "override network names, new network no endpoint",
            {"MESC_NETWORK_NAMES": "xyz=123"},
            full_config,
            {"type": "endpoint_by_network", "fields": {"network": "1"}},
            full_config["endpoints"]["llamanodes_ethereum"],
        ),
    ]

    # override endpoints
    tests += []

    # override profiles
    tests += []

    # override global metadata
    tests += []

    # override endpoint metadata
    tests += []

    return tests


if __name__ == "__main__":
    import json

    all_tests = {
        "basic_query_tests": create_basic_query_tests() + create_find_endpoints_tests(),
        "override_tests": create_override_tests(),
        "invalid_config_tests": create_invalid_config_tests(),
    }
    for test_name, test_data in all_tests.items():
        path = "generated/" + test_name + ".json"
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as f:
            json.dump(test_data, f)
