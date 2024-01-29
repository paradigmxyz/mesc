from __future__ import annotations
from _pytest.config.argparsing import Parser
from _pytest.python import Metafunc

default_adapters = ["adapters/python", "adapters/cli"]


def pytest_addoption(parser: Parser) -> None:
    parser.addoption("--adapters", action="store", nargs="+", default=default_adapters)


def pytest_generate_tests(metafunc: Metafunc) -> None:
    if "adapter" in metafunc.fixturenames:
        adapters = metafunc.config.getoption("adapters")
        metafunc.parametrize("adapter", adapters, scope="function")
