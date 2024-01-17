from __future__ import annotations
import json
import os
from typing_extensions import cast, overload, Literal, Any

from .types import RpcConfig
from . import exceptions
from . import overrides
from . import validation


def read_config_data() -> RpcConfig:
    """read MESC config according to MESC environment variables"""
    # locate and load config using env vars
    mode = os.environ.get('MESC_MODE')
    if mode == 'DISABLED':
        raise exceptions.MescDisabled(
            'MESC_MODE=DISABLED, check with is_mesc_enabled()'
        )
    elif mode == 'PATH':
        config = read_file_config(validate=False)
    elif mode == 'ENV':
        config = read_env_config(validate=False)
    elif mode not in ['', None]:
        raise Exception(
            'invalid mode: ' + str(mode) + ', must be PATH, ENV, or DISABLED'
        )
    elif os.environ.get('MESC_PATH') not in ['', None]:
        config = read_file_config(validate=False)
    elif os.environ.get('MESC_ENV') not in ['', None]:
        config = read_env_config(validate=False)
    else:
        raise exceptions.MescDisabled('To enable MESC, set MESC_PATH or MESC_ENV')

    # apply overrides
    config = overrides.apply_env_overrides(config)

    # validate
    validation.validate(config)
    return cast(RpcConfig, config)


@overload
def read_env_config(*, validate: Literal[False]) -> Any:
    ...


@overload
def read_env_config(*, validate: Literal[True]) -> RpcConfig:
    ...


def read_env_config(*, validate: bool = True) -> RpcConfig | Any:
    """read config from MESC_ENV environment variable"""
    # obtain raw config data from env
    value = os.environ.get('MESC_ENV')
    if value is None or value == '':
        raise exceptions.LoadError(
            'Cannot load MESC config from MESC_ENV, value is not set'
        )

    # parse config data as JSON
    try:
        config = json.loads(value)
    except json.JSONDecodeError:
        raise exceptions.InvalidConfig('MESC_ENV is not formatted as valid JSON')

    # validate config
    if validate:
        validation.validate(config)
        return cast(RpcConfig, config)
    else:
        return config


@overload
def read_file_config(*, path: str | None = None, validate: Literal[False]) -> Any:
    ...


@overload
def read_file_config(*, path: str | None = None, validate: Literal[True]) -> RpcConfig:
    ...


def read_file_config(
    *, path: str | None = None, validate: bool = True
) -> RpcConfig | Any:
    """read config from MESC config file"""
    # obtain config file path
    if path is None:
        path = os.environ.get('MESC_PATH')
    if path is None or path == '':
        raise exceptions.LoadError(
            'Cannot load MESC config from MESC_ENV, value is not set'
        )
    path = os.path.expanduser(path)

    # load file contents
    try:
        with open(path, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        raise exceptions.LoadError('File does not exist: ' + str(path))
    except PermissionError:
        raise exceptions.LoadError(
            'Insufficient permissions to load file: ' + str(path)
        )
    except Exception:
        raise exceptions.LoadError('Unable to load file: ' + str(path))

    # parse raw data as json
    try:
        config = json.loads(content)
    except json.JSONDecodeError:
        raise exceptions.InvalidConfig(
            'file at MESC_PATH is not formatted as valid JSON'
        )

    # validate config
    if validate:
        validation.validate(config)
        return cast(RpcConfig, config)
    else:
        return config
