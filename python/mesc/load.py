from __future__ import annotations
import json
import os

from .types import RpcConfig


def read_config_data() -> RpcConfig:
    mode = os.environ.get('RPC_CONFIG_MODE')
    if mode == 'PATH':
        return read_file_config()
    elif mode == 'ENV':
        return read_env_config()
    elif mode not in ['', None]:
        raise Exception('invalid mode: ' + str(mode))
    elif os.environ.get('RPC_CONFIG_PATH') not in ['', None]:
        return read_file_config()
    elif os.environ.get('RPC_CONFIG_ENV') not in ['', None]:
        return read_env_config()
    else:
        raise Exception('config not specified')


def read_env_config() -> RpcConfig:
    return json.loads(os.environ.get('RPC_CONFIG_ENV'))  # type: ignore


def read_file_config() -> RpcConfig:
    with open(os.environ.get('RPC_CONFIG_PATH'), 'r') as f:  # type: ignore
        return json.load(f)  # type: ignore
