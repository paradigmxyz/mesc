#!/usr/bin/env python3

import argparse
import os
import json
import subprocess
import tempfile
from typing import Any

from mesc import RpcConfig

#
# # setups
#


def get_setups():
    return {"path": setup_config_path, "env": setup_config_env_var}


def setup_config_path(config: RpcConfig):
    temp_dir = tempfile.mkdtemp()
    temp_path = os.path.join(temp_dir, "config.json")
    with open(temp_path, "w") as f:
        json.dump(config, f)
    try:
        yield {"MESC_CONFIG_MODE": "PATH", "MESC_CONFIG_PATH": temp_path}
    finally:
        os.remove(temp_dir)


def setup_config_env_var(config: RpcConfig):
    config_data = json.dumps(config)
    try:
        yield {"MESC_CONFIG_MODE": "ENV", "MESC_CONFIG_JSON": config_data}
    finally:
        pass


#
# # runners
#


def run_basic_query_tests(tests, adapter):
    success = []
    failure = []
    setups = get_setups()
    for test in tests:
        name, config, query, endpoint = test
        for setup_name, setup in setups.items():
            with setup(config) as env:
                cmd = [adapter, json.dumps(query)]
                output = subprocess.check_output(cmd, env=env)
                if json_equal(output, endpoint):
                    success.append(name)
                else:
                    failure.append(name)
    return success, failure


def json_equal(lhs: Any, rhs: Any):
    return json.dumps(lhs, sort_keys=True) == json.dumps(rhs, sort_keys=True)


def run_override_tests(tests, adapter):
    pass


def run_invalid_data_tests(tests, adapter):
    success = []
    failure = []
    setups = get_setups()
    for test in tests:
        name, config, query = test
        for setup_name, setup in setups.items():
            with setup(config) as env:
                cmd = [adapter, json.dumps(query)]
                output = subprocess.check_output(cmd, env=env)
                if output.startswith("ERROR"):
                    success.append(name)
                else:
                    failure.append(name)
    return success, failure


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("adapter", help="path to adapter")
    parser.add_argument(
        "generated_path", help="path to generated tests", default="./generated"
    )
    args = parser.parse_args()
    adapter = args.adapter
    generated_path = args.generated_path

    print("running basic tests...")
    path = os.path.join(generated_path, "basic_query_tests.json")
    with open(path) as f:
        tests = json.load(f)
    basic_success, basic_failure = run_basic_query_tests(tests, adapter)
    print(
        len(basic_success), "/", len(basic_success) + len(basic_failure), "successful"
    )
    print()
    print("running override tests...")
    path = os.path.join(generated_path, "override_tests.json")
    with open(path) as f:
        tests = json.load(f)
    override_success, override_failure = run_override_tests(tests, adapter)
    print(
        len(override_success),
        "/",
        len(override_success) + len(override_failure),
        "successful",
    )
    print()
    print("running invalid data tests...")
    path = os.path.join(generated_path, "invalid_data_tests.json")
    with open(path) as f:
        tests = json.load(f)
    invalid_data_success, invalid_data_failure = run_invalid_data_tests(tests, adapter)
    print(
        len(invalid_data_success),
        "/",
        len(invalid_data_success) + len(invalid_data_failure),
        "successful",
    )
    print()
    n_failures = len(basic_failure) + len(override_failure) + len(invalid_data_failure)
    if n_failures == 0:
        print("SUCCESS")
    else:
        print(n_failures, "total failures")
        print()
        print("FAILURE")
