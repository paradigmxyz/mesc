from __future__ import annotations

import contextlib
import os
import json
import shutil
import subprocess
import sys
import tempfile
from typing import (
    Any,
    Callable,
    ContextManager,
    Generator,
    Mapping,
    cast,
    Union,
)
import pytest

import generate

import typing

if typing.TYPE_CHECKING:
    from mesc.types import RpcConfig, MescQuery


# tests are in form [test_name, env, config, query, result, should_succeed]
Test = tuple[
    str,
    dict[str, str],
    RpcConfig,
    Union[None, MescQuery],
    Any,
    bool,
]


class AdapterFailure(Exception):
    pass


class OutputDoesNotMatch(Exception):
    pass


class QueryFailed(Exception):
    pass


class QueryShouldHaveFailed(Exception):
    pass


def get_setups() -> (
    list[tuple[str, Callable[[RpcConfig], ContextManager[Mapping[str, str]]]]]
):
    return [
        ("path", setup_config_path),
        ("env", setup_config_env_var),
    ]


@contextlib.contextmanager
def setup_config_path(config: RpcConfig) -> Generator[Mapping[str, str], None, None]:
    temp_dir = tempfile.mkdtemp()
    temp_path = os.path.join(temp_dir, "config.json")
    with open(temp_path, "w") as f:
        json.dump(config, f)
    try:
        yield {"MESC_MODE": "PATH", "MESC_PATH": temp_path}
    finally:
        shutil.rmtree(temp_dir)


@contextlib.contextmanager
def setup_config_env_var(config: RpcConfig) -> Generator[Mapping[str, str], None, None]:
    config_data = json.dumps(config)
    try:
        yield {"MESC_MODE": "ENV", "MESC_ENV": config_data}
    finally:
        pass


tests = generate.generate_tests()


@pytest.mark.parametrize("setup", get_setups())
@pytest.mark.parametrize("test", tests, ids=[test[0] for test in tests])
def test_mesc_output(
    setup: tuple[str, Callable[[RpcConfig], ContextManager[Mapping[str, str]]]],
    test: Test,
    adapter: str,
) -> None:
    # load inputs
    if adapter in ["", None]:
        raise AdapterFailure("must specify adapter")
    test_name, test_env, config, query, target_result, should_succeed = test
    setup_name, f_setup = setup

    # run query
    env = {}
    with f_setup(test[2]) as setup_env:
        try:
            env["PATH"] = os.environ["PATH"]
            env.update(setup_env)
            env.update(test_env)
            cmd = [adapter, json.dumps(query)]
            output = subprocess.check_output([sys.executable] + cmd, env=env)
        except Exception:
            raise AdapterFailure("adapter broken, test failed to complete")

    # package error data for printing
    error_data = dict(
        config=config,
        env=env,
        test_name=test_name,
        query=query,
        output=output,
        target_result=target_result,
    )

    # decode output as json
    decode_exception: Exception | None = None
    try:
        actual_result = json.loads(output.decode("utf-8").strip())
        output_decodable = True
    except Exception as e:
        actual_result = None
        output_decodable = False
        decode_exception = e

    # assert whether output is correct
    if not output_decodable:
        if should_succeed:
            # failure case: output could not be decoded
            print_error_summary(**error_data)
            if len(output) == 0:
                message = "no output"
            elif output.decode("utf-8").startswith("FAIL"):
                message = "QueryFailed:"
            else:
                exception = cast(Exception, decode_exception)
                message = str(type(exception).__name__) + ": " + str(exception.args)
            raise QueryFailed(message, adapter, setup_name, test_name)
        else:
            # working properly
            pass
    else:
        if json_equal(actual_result, target_result):
            if should_succeed:
                # working properly
                pass
            else:
                # failure case: incorrect query output
                print_error_summary(**error_data)
                raise QueryShouldHaveFailed(
                    "success when expecting failure", adapter, setup_name, test_name
                )
        else:
            if should_succeed:
                # failure case: query should have failed
                print_error_summary(**error_data)
                raise OutputDoesNotMatch(
                    "output does not match", adapter, setup_name, test_name
                )
            else:
                # working properly
                pass


def json_equal(lhs: Any, rhs: Any) -> bool:
    return cannonical_json(lhs) == cannonical_json(rhs)


def cannonical_json(data: Any) -> str:
    if isinstance(data, list):
        data = sorted(cannonical_json(item) for item in data)
    return json.dumps(data, sort_keys=True)


def print_error_summary(
    config: Any,
    env: Mapping[str, str],
    test_name: str,
    query: Any,
    output: Any,
    target_result: Any,
) -> None:
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
            json.dumps(json.loads(output.decode("utf-8")), indent=4, sort_keys=True),
        )
    except Exception:
        print("OUTPUT:", output.decode("utf-8").strip())
    print("EXPECTED:", json.dumps(target_result, indent=4, sort_keys=True))
    print("TEST_NAME:", test_name)
