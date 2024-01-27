from __future__ import annotations

from _pytest.config.argparsing import Parser
from _pytest.fixtures import FixtureRequest
import pytest


def pytest_addoption(parser: Parser) -> None:
    parser.addoption("--adapter", action="append")


@pytest.fixture
def adapter(request: FixtureRequest) -> str:
    adapter = request.config.getoption("adapter")
    if isinstance(adapter, str):
        return adapter
    else:
        raise Exception("must use str adapter")
