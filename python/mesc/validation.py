from __future__ import annotations

from typing_extensions import Any
from typing import Sequence

from .exceptions import InvalidConfig
from .types import rpc_config_types, endpoint_types, profile_types
from . import network_utils


def is_valid(config: Any) -> bool:
    try:
        validate(config)
        return True
    except InvalidConfig:
        return False


def validate(config: Any) -> None:
    # all fields present with the correct types
    if not isinstance(config, dict):
        raise InvalidConfig('RpcConfig must be a dict')
    for field, field_type in rpc_config_types.items():
        if field not in config:
            raise InvalidConfig('RpcConfig is missing field: ' + str(field))
        value = config[field]
        _check_type('RpcConfig', None, field, field_type, value)
        if isinstance(value, dict):
            _check_str_keys(field, value)
    for name, endpoint in config['endpoints'].items():
        for field, field_type in endpoint_types.items():
            if field not in endpoint:
                raise InvalidConfig(
                    'Endpoint ' + str(name) + ' is missing field: ' + str(field)
                )
            value = endpoint[field]
            _check_type('Endpoint', name, field, field_type, value)
            if isinstance(value, dict):
                _check_str_keys(field, value)
    for name, profile in config['profiles'].items():
        for field, field_type in profile_types.items():
            if field not in profile:
                raise InvalidConfig(
                    'Profile ' + str(name) + ' is missing field: ' + str(field)
                )
            value = profile[field]
            _check_type('Profile', name, field, field_type, value)
            if isinstance(value, dict):
                _check_str_keys(field, value)
    for field in ['network_defaults', 'network_names']:
        for value in config[field].values():
            if not isinstance(value, str):
                raise InvalidConfig(
                    'Entries in '
                    + str(field)
                    + ' must be strings, but there is a value of type '
                    + str(type(value))
                )
    for name, profile in config['profiles'].items():
        for field in ['network_defaults', 'network_names']:
            for value in config[field].values():
                if not isinstance(value, str):
                    raise InvalidConfig(
                        'Entries in '
                        + str(field)
                        + ' must be strings, but in the profile '
                        + name
                        + ' there is a value of type '
                        + str(type(value))
                    )

    # referenced endpoints exit
    endpoint = config['default_endpoint']
    if endpoint is not None and endpoint not in config['endpoints']:
        raise InvalidConfig(
            'Referenced default_endpoint does not exist: ' + str(endpoint)
        )
    for endpoint in config['network_defaults'].values():
        if endpoint not in config['endpoints']:
            raise InvalidConfig(
                'Referenced endpoint in network_defaults does not exist: '
                + str(endpoint)
            )
    for name, profile in config['profiles'].items():
        endpoint = profile['default_endpoint']
        if endpoint is not None and endpoint not in config['endpoints']:
            raise InvalidConfig(
                'In profile '
                + name
                + ' referenced endpoint does not exist: '
                + str(endpoint)
            )
        for endpoint in profile['network_defaults'].values():
            if endpoint not in config['endpoints']:
                raise InvalidConfig(
                    'In profile '
                    + name
                    + ' referenced endpoint does not exist: '
                    + str(endpoint)
                )

    # default endpoints of each network actually use that specified network
    for chain_id, endpoint_name in config['network_defaults'].items():
        if not network_utils.chain_ids_equal(
            chain_id, config['endpoints'][endpoint_name]['chain_id']
        ):
            raise InvalidConfig(
                'Endpoint is set as the default endpoint of network '
                + chain_id
                + ", but the endpoint's chain_id is different "
                + config['endpoints'][endpoint_name]['chain_id']
            )
    for profile_name, profile in config['profiles'].items():
        for chain_id, endpoint_name in profile['network_defaults'].items():
            if not network_utils.chain_ids_equal(
                chain_id, config['endpoints'][endpoint_name]['chain_id']
            ):
                raise InvalidConfig(
                    'Endpoint is set as the default endpoint of network '
                    + chain_id
                    + ' in profile '
                    + profile_name
                    + ", but the endpoint's chain_id is different "
                    + config['endpoints'][endpoint_name]['chain_id']
                )

    # endpoint map keys match endpoint name fields
    for endpoint_name, endpoint in config['endpoints'].items():
        if endpoint['name'] != endpoint_name:
            raise InvalidConfig(
                'Endpoint does not match endpoint mapping key: '
                + endpoint['name']
                + ' != '
                + endpoint_name
            )

    # profile map keys match profile name fields
    for profile_name, profile in config['profiles'].items():
        if profile['name'] != profile_name:
            raise InvalidConfig(
                'Profile does not match profile mapping key: '
                + profile['name']
                + ' != '
                + profile_name
            )

    # chain_id's are valid
    for chain_id in config['network_defaults'].keys():
        if not network_utils.is_chain_id(chain_id):
            raise Exception(
                'Invalid chain_id used in network_defaults: ' + str(chain_id)
            )
    for profile_name, profile in config['profiles'].items():
        for chain_id in profile['network_defaults'].keys():
            if not network_utils.is_chain_id(chain_id):
                raise Exception(
                    'Invalid chain_id used in profile '
                    + profile_name
                    + ' network_defaults: '
                    + str(chain_id)
                )
    for endpoint in config['endpoints'].values():
        chain_id = endpoint['chain_id']
        if chain_id is not None and not network_utils.is_chain_id(chain_id):
            raise Exception(
                'Invalid chain_id used in endpoint '
                + endpoint['name']
                + ': '
                + str(chain_id)
            )

    # no duplicate default network entries using decimal vs hex
    ensure_no_chain_id_collisions(
        list(config['network_defaults'].keys()), 'network defaults'
    )
    for profile_name, profile in config['profiles'].items():
        ensure_no_chain_id_collisions(
            list(config['network_defaults'].keys()), 'profile ' + profile_name
        )


def ensure_no_chain_id_collisions(chain_ids: Sequence[str], name: str) -> None:
    hex_numbers = set()
    for chain_id in chain_ids:
        as_hex = network_utils.chain_id_to_standard_hex(chain_id)
        if as_hex in hex_numbers:
            raise Exception(
                'chain_id collision, '
                + str(name)
                + ' has multiple decimal/hex values for chain_id: '
                + str(chain_id)
            )
        else:
            hex_numbers.add(as_hex)


def _check_type(
    datatype: str,
    name: str | None,
    field: str,
    field_type: type | tuple[type, ...],
    value: str,
) -> None:
    if not isinstance(value, field_type):
        if name is not None:
            display = datatype + ' ' + name
        else:
            display = datatype
        raise InvalidConfig(
            display
            + ' invalid type for field: '
            + str(field)
            + ', it should have type '
            + str(type(field_type))
            + ' but instead has type '
            + str(type(value))
        )


def _check_str_keys(field: Any, value: Any) -> None:
    for key in value.keys():
        if not isinstance(key, str):
            raise InvalidConfig(
                'Each key within '
                + field
                + ' must be a string, instead '
                + key
                + ' is a '
                + str(type(value))
            )
