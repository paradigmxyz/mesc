from __future__ import annotations
import json
import os

from .types import RpcConfig
from . import exceptions
from . import overrides


def read_config_data() -> RpcConfig:
    mode = os.environ.get("MESC_MODE")
    if mode == "DISABLE":
        raise exceptions.MescDisabled("MESC disabled, check with is_mesc_enabled()")
    if mode == "PATH":
        config = read_file_config()
    elif mode == "ENV":
        config = read_env_config()
    elif mode not in ["", None]:
        raise Exception("invalid mode: " + str(mode))
    elif os.environ.get("MESC_PATH") not in ["", None]:
        config = read_file_config()
    elif os.environ.get("MESC_ENV") not in ["", None]:
        config = read_env_config()
    else:
        raise Exception("config not specified")

    config = overrides.apply_env_overrides(config)

    return config


def read_env_config() -> RpcConfig:
    return json.loads(os.environ.get("MESC_ENV"))  # type: ignore


def read_file_config() -> RpcConfig:
    path = os.path.expanduser(os.environ.get("MESC_PATH"))  # type: ignore
    with open(path, "r") as f:  # type: ignore
        return json.load(f)  # type: ignore
