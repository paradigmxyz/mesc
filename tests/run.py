#!/usr/bin/env python3

import argparse
import contextlib
import copy
import os
import json
import shutil
import subprocess
import sys
import tempfile
from typing import Any, Mapping, MutableSequence

from mesc.types import EndpointQuery, MultiEndpointQuery, RpcConfig


# tests are in form [test_name, env, config, query, result, should_succeed]
Test = tuple[
    str,
    dict[str, str],
    RpcConfig,
    None | EndpointQuery | MultiEndpointQuery,
    Any,
    bool,
]


class OutputDoesNotMatch(Exception):
    pass


class FailedQuery(Exception):
    pass


def get_setups():
    return {
        "path": setup_config_path,
        "env": setup_config_env_var,
    }


@contextlib.contextmanager
def setup_config_path(config: RpcConfig):
    temp_dir = tempfile.mkdtemp()
    temp_path = os.path.join(temp_dir, "config.json")
    with open(temp_path, "w") as f:
        json.dump(config, f)
    try:
        yield {"MESC_CONFIG_MODE": "PATH", "MESC_CONFIG_PATH": temp_path}
    finally:
        shutil.rmtree(temp_dir)


@contextlib.contextmanager
def setup_config_env_var(config: RpcConfig):
    config_data = json.dumps(config)
    try:
        yield {"MESC_CONFIG_MODE": "ENV", "MESC_CONFIG_ENV": config_data}
    finally:
        pass


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("adapters", help="path to adapter", nargs="+")
    parser.add_argument(
        "--verbose", help="print extra error output", action="store_true"
    )
    parser.add_argument("--halt", help="halt on first error", action="store_true")
    parser.add_argument(
        "--generated-path",
        help="path to generated tests",
        default="./generated/tests.json",
    )
    args = parser.parse_args()

    return {
        "adapters": args.adapters,
        "generated_path": args.generated_path,
        "verbose": args.verbose,
        "halt": args.halt,
    }


def run_test(
    test: Test,
    adapter: str,
    setup_name: str,
    setup_env: Mapping[str, str],
    halt: bool,
    verbose: bool,
    successes: MutableSequence[tuple[str, str, str]],
    failures: MutableSequence[tuple[str, str, str, str]],
) -> None:
    test_name, test_env, config, query, target_result, should_succeed = test

    # run command
    try:
        # create env
        env = {}
        env["PATH"] = os.environ["PATH"]
        env.update(setup_env)
        env.update(test_env)

        cmd = [adapter, json.dumps(query)]
        output = subprocess.check_output([sys.executable] + cmd, env=env)
    except Exception:
        if halt:
            sys.exit()
        message = "adapter broken, test failed to complete"
        failures.append((adapter, setup_name, test_name, message))
        return

    # parse output
    try:
        actual_result = json.loads(output.decode("utf-8").strip())
        if not json_equal(actual_result, target_result):
            raise OutputDoesNotMatch("")
        if should_succeed:
            successes.append((adapter, setup_name, test_name))
        else:
            message = "success when expecting failure"
            failures.append((adapter, setup_name, test_name, message))
            if halt:
                if verbose:
                    print(message)
                sys.exit()
    except Exception as e:
        if should_succeed:
            if verbose:
                print("CONFIG", json.dumps(config, sort_keys=True, indent=4))
                print(
                    "ENV:",
                    json.dumps(
                        {k: v for k, v in env.items() if k != "PATH"},
                        sort_keys=True,
                        indent=4,
                    ),
                )
                print("QUERY:", query)
                try:
                    print(
                        "OUTPUT:",
                        json.dumps(
                            json.loads(output.decode("utf-8")), indent=4, sort_keys=True
                        ),
                    )
                except Exception:
                    print("OUTPUT:", output.decode('utf-8').strip())
                print("EXPECTED:", json.dumps(target_result, indent=4, sort_keys=True))
                print("EXCEPTION:", type(e), e)
                print("TEST_NAME:", test_name)
                print("INDEX:", len(successes) + len(failures))
            if halt:
                sys.exit()
            if len(output) == 0:
                message = "no output"
            elif output.decode("utf-8").startswith("FAIL"):
                message = "QueryFailed:"
            else:
                message = str(type(e).__name__) + ": " + str(e.args[0])
            failures.append((adapter, setup_name, test_name, message))
        else:
            successes.append((adapter, setup_name, test_name))


def json_equal(lhs: Any, rhs: Any):
    return json.dumps(lhs, sort_keys=True) == json.dumps(rhs, sort_keys=True)


if __name__ == "__main__":
    # load data
    args = parse_args()
    adapters = args["adapters"]
    setups = get_setups()
    with open(args["generated_path"], "r") as f:
        tests = json.load(f)

    # store successes as (adapter, test_name, setup_name)
    successes: MutableSequence[tuple[str, str, str]] = []

    # store failures as (adapter, test_name, setup_name, error)
    failures: MutableSequence[tuple[str, str, str, str]] = []

    # run tests
    print(
        "testing",
        len(tests),
        "queries in",
        len(setups),
        "modes for",
        len(adapters),
        "adapter(s)",
    )
    print()
    for adapter in adapters:
        for test in tests:
            for setup_name, setup in setups.items():
                with setup(test[2]) as setup_env:
                    run_test(
                        adapter=adapter,
                        test=test,
                        setup_name=setup_name,
                        setup_env=setup_env,
                        halt=args["halt"],
                        verbose=args["verbose"],
                        successes=successes,
                        failures=failures,
                    )

    # summary
    print()
    print(len(successes), "/", len(setups) * len(tests), "successful")
    n_failures = len(failures)
    if n_failures == 0:
        print()
        print("SUCCESS")
    else:
        print()
        print(n_failures, "total failures:")

        try:
            import toolstr

            labels = ["adapter", "mode", "test", "error"]
            rows = [list(row) for row in failures]
            for row in rows:
                row[0] = row[0].split("/")[-1]
                row[2] = row[2].replace(", ", "\n")
                row[3] = row[3]
            column_justify = {"error": "left"}
            toolstr.print_multiline_table(
                rows,
                separate_all_rows=False,
                compact=3,
                labels=labels,
                add_row_index=True,
                max_table_width=shutil.get_terminal_size().columns,
                column_justify=column_justify,
            )
        except ImportError:
            for failure in failures:
                print("-", failure)
        print()
        print("FAILURE")
