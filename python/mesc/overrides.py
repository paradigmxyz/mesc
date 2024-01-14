from __future__ import annotations

import copy
import json
import os
import re
from typing_extensions import Any, Mapping, MutableMapping

from . import interface, network_utils, exceptions
from .types import RpcConfig, Endpoint, Profile


def apply_env_overrides(config: RpcConfig | None) -> RpcConfig:
    """apply overrides to config"""

    if config is not None:
        config = copy.deepcopy(config)
    else:
        config = {
            'mesc_version': '1.0',
            'default_endpoint': None,
            'network_defaults': {},
            'network_names': {},
            'endpoints': {},
            'profiles': {},
            'global_metadata': {},
        }

    # add override endpoints
    for name, endpoint in env_endpoints().items():
        if name in config['endpoints']:
            if endpoint['url'] is not None:
                config['endpoints'][name]['url'] = endpoint['url']
            if endpoint['chain_id'] is not None:
                config['endpoints'][name]['chain_id'] = endpoint['chain_id']
            for key, value in endpoint['endpoint_metadata'].items():
                config['endpoints'][name]['endpoint_metadata'][key] = value
        else:
            config['endpoints'][name] = endpoint

    # add override network names
    config['network_names'].update(env_network_names())

    # add override network defaults
    for chain_id, endpoint_name in env_network_defaults().items():
        if endpoint_name == '':
            if chain_id in config['network_defaults']:
                del config['network_defaults'][chain_id]
        else:
            config['network_defaults'][chain_id] = endpoint_name

    # add override profiles
    config['profiles'].update(env_profiles())

    # add override default endpoint
    default_endpoint = env_default_endpoint(config)
    if default_endpoint is not None:
        config['default_endpoint'] = default_endpoint

    # add override global metadata
    config['global_metadata'].update(env_global_metadata())

    # add override endpoint metadata
    for endpoint_name, metadata in env_endpoint_metadata().items():
        config['endpoints'][endpoint_name]['endpoint_metadata'].update(metadata)

    return config


def env_default_endpoint(config: RpcConfig) -> str | None:
    """get override default endpoint"""
    # load from environment
    default_endpoint = os.environ.get('MESC_DEFAULT_ENDPOINT')
    if default_endpoint is None or default_endpoint == '':
        return None

    if default_endpoint in config['endpoints']:
        # if endpoint already exists, use it
        return default_endpoint
    elif default_endpoint.isdecimal():
        # if endpoint is a number, use network default as global default
        chain_id = default_endpoint
    elif default_endpoint in config['network_names']:
        # if endpoint is a network name, use network default as global default
        chain_id = config['network_names'][default_endpoint]
    else:
        # otherwise, try to use override value as a network name
        maybe_chain_id = network_utils.network_name_to_chain_id(
            default_endpoint, config=config
        )
        if maybe_chain_id is None:
            raise exceptions.InvalidOverride(
                'Invalid syntax used for MESC_DEFAULT_ENDPOINT'
            )
        else:
            chain_id = maybe_chain_id

    endpoint = interface.get_endpoint_by_network(chain_id, config=config)
    if endpoint is not None:
        return endpoint['name']
    else:
        # try using as implicit endpoint url here
        raise exceptions.InvalidOverride('Invalid value used for MESC_DEFAULT_ENDPOINT')


def env_network_defaults() -> Mapping[str, str]:
    """get override network defaults"""
    network_defaults = os.environ.get('MESC_NETWORK_DEFAULTS')
    if network_defaults is None or network_defaults == '':
        return {}
    try:
        items = [item.split('=', 1) for item in network_defaults.split(' ')]
        return {network: endpoint for network, endpoint in items}
    except Exception:
        raise exceptions.InvalidOverride(
            'Invalid syntax used for MESC_NETWORK_DEFAULTS'
        )


def env_network_names() -> Mapping[str, str]:
    """get override network names"""
    network_names = os.environ.get('MESC_NETWORK_NAMES')
    if network_names is None or network_names == '':
        return {}

    if network_names.startswith('{'):
        try:
            value = json.loads(network_names)
        except json.JSONDecodeError:
            raise exceptions.InvalidOverride(
                'Invalid syntax used for MESC_NETWORK_NAMES'
            )
        if isinstance(value, dict) and all(
            isinstance(k, str) and isinstance(v, str) for k, v in value.items()
        ):
            return value
        else:
            raise exceptions.InvalidOverride(
                'Invalid syntax used for MESC_NETWORK_NAMES'
            )
    else:
        try:
            pairs = [item.split('=') for item in network_names.split(' ')]
            return {key: value for key, value in pairs}
        except Exception:
            raise exceptions.InvalidOverride(
                'Invalid syntax used for MESC_NETWORK_NAMES'
            )


def env_endpoints() -> Mapping[str, Endpoint]:
    """get override endpoints"""
    endpoints: MutableMapping[str, Endpoint] = {}

    # get override
    raw_endpoints = os.environ.get('MESC_ENDPOINTS')
    if raw_endpoints is None or raw_endpoints == '':
        return endpoints

    # gather explicit endpoints
    pattern = r'^(?:(?P<name>[A-Za-z_-]+)(?::(?P<chain_id>\w+))?=\s*)?(?P<url>\S+)$'
    for item in raw_endpoints.split(' '):
        match = re.match(pattern, item)
        if match:
            chain_id = match.group('chain_id')
            url = match.group('url')
            name = match.group('name')
            if name is None:
                from urllib.parse import urlparse

                if not urlparse(url).scheme:
                    name = urlparse('http://' + url).hostname
                else:
                    name = urlparse(url).hostname
                if name is None:
                    raise Exception('could create name for endpoint')
                name = '.'.join(name.split('.')[:-1])

            endpoints[name] = {
                'name': name,
                'url': url,
                'chain_id': chain_id,
                'endpoint_metadata': {},
            }
        else:
            raise exceptions.InvalidOverride('Invalid syntax used for MESC_ENDPOINTS')

    # add implicit endpoints
    # implicit_endpoints = _collect_implicit_endpoints()
    # for endpoint_name, endpoint in implicit_endpoint.items():
    #     endpoints[name] = endpoint

    # change

    return endpoints


def _collect_implicit_endpoints(
    # endpoints: Mapping[str, Endpoint]
) -> Mapping[str, Endpoint]:
    """implicit endpoints are those defined only by a url
    - can be in MESC_DEFAULT_ENDPOINT, MESC_NETWORK_DEFAULTS, or MESC_PROFILES
    - implicit endpoints do not have a chain_id
    - if an endpoint already exists with that url, just use that endpoint
    """
    raw_endpoints: list[str] = []

    # global default
    default_endpoint = os.environ.get('MESC_DEFAULT_ENDPOINT')
    if default_endpoint is not None:
        raw_endpoints.append(default_endpoint)

    # env defaults
    network_defaults = env_network_defaults()
    for network, endpoint in network_defaults.items():
        raw_endpoints.append(endpoint)

    # profile defaults
    profiles = env_profiles()
    for name, profile in profiles.items():
        profile_default = profile['default_endpoint']
        if profile_default is not None:
            raw_endpoints.append(profile_default)
        for chain_id, endpoint_name in profile['network_defaults'].items():
            raw_endpoints.append(endpoint_name)

    # # process raw endpoints
    endpoints: Mapping[str, Endpoint] = {}
    # for endpoint in raw_endpoints:
    #     if _is_url(endpoint):
    #         pass

    return endpoints


def env_profiles() -> Mapping[str, Profile]:
    """get profile overrides"""
    raw_profiles = os.environ.get('MESC_PROFILES')
    if raw_profiles is None or raw_profiles == '':
        return {}

    profiles: MutableMapping[str, Profile] = {}
    for item in raw_profiles.split(' '):
        try:
            key, value = item.split('=')
        except ValueError:
            raise Exception('invalid value for MESC_PROFILES')
        subkeys = key.split('.')
        profiles.setdefault(
            subkeys[0],
            {
                'name': subkeys[0],
                'default_endpoint': None,
                'network_defaults': {},
                'use_mesc': True,
            },
        )
        if len(subkeys) == 2 and subkeys[1] == 'default_endpoint':
            profiles[subkeys[0]]['default_endpoint'] = value
        elif len(subkeys) == 3 and subkeys[1] == 'network_defaults':
            name, _, network = subkeys
            profiles[name]['network_defaults'][network] = value
        else:
            raise Exception('invalid value for MESC_PROFILES')

    return profiles


def env_global_metadata() -> Mapping[str, Any]:
    """get gloabl metadata override"""
    # load env override
    global_metadata = os.environ.get('MESC_GLOBAL_METADATA')
    if global_metadata is None or global_metadata == '':
        return {}

    # parse as JSON
    try:
        value = json.loads(global_metadata)
    except json.JSONDecodeError:
        raise Exception('invalid value for MESC_GLOBAL_METADATA')

    # validate content
    if isinstance(value, dict) and all(isinstance(k, str) for k in value.keys()):
        return value
    else:
        raise Exception('invalid value for MESC_GLOBAL_METADATA')


def env_endpoint_metadata() -> Mapping[str, Mapping[str, Any]]:
    """get endpoint metadata overrides"""
    # load env override
    endpoint_metadata = os.environ.get('MESC_ENDPOINT_METADATA')
    if endpoint_metadata is None or endpoint_metadata == '':
        return {}

    # parse as JSON
    try:
        value = json.loads(endpoint_metadata)
    except json.JSONDecodeError:
        raise Exception('invalid value for MESC_ENDPOINT_METADATA')

    if isinstance(value, dict) and all(
        isinstance(k, str)
        and isinstance(v, dict)
        and all(isinstance(subk, str) for subk in v.keys())
        for k, v in value.items()
    ):
        return value
    else:
        raise Exception('invalid value for MESC_ENDPOINT_METADATA')
