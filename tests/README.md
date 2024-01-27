
# MESC Tests

These tests measure the degree to which a MESC implementation is compliant with the MESC specification.

These tests are language-agnostic, allowing the same tests to be run against each MESC implementation.

## Usage

1. Install `pytest` (and `pytest-xdist` for parallel testing):
`pip install pytest pytest-xdist`


2. Then run one of the following commands:

|description | command |
| --- | --- |
| run tests | `pytest test.py` |
| run test in parallel (much faster) | `pytest test.py -n auto` |
| run tests in debug mode (helpful for debugging) | `pytest test.py --pdb` |
| run tests for just one adapter | `pytest test.py --adapters adapters/python` |


## Adapters

Each MESC implementation has an adapter that receives a test as input and prints the result as output. Adapters are located in the [adapters](https://github.com/paradigmxyz/mesc/tree/main/tests/adapters) directory.

To make a custom adapter:
1. adapter should be a script that takes a JSON [`MescQuery`](https://github.com/paradigmxyz/mesc/blob/5cb1086c237f2b38c3a3095466823f3d64a62052/python/mesc/types.py#L115) as its single argument
2. the adapter should run the query, and then print the result as JSON
3. if the config loading or the query fails, simply print the word `FAIL`

The adapter should never crash upon failure, just print the word `FAIL`


## Files
- `adapters/` contains a test adapter for each MESC implementation
- `conftest.py` configuration file for pytest
- `generate.py` generates all of the MESC test cases
- `test.py` packages MESC tests into form usable by pypi

