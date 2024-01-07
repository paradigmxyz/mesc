from __future__ import annotations


class MescDisabled(Exception):
    """raised when MESC data is requested even though MESC is disabled"""


class MissingEndpoint(Exception):
    """raised when attempting to load an endpoint that does not exist"""


class InvalidOverride(Exception):
    """raised when override data is not valid"""


class LoadError(Exception):
    """raised when unable to load MESC config data"""


class InvalidConfig(Exception):
    """raised when MESC config data is invalid"""
