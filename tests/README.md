
# MESC Tests

A language-agnostic set of tests is used to check whether each MESC implementation is compliant with the MESC specification.

## Usage

1. Install `pytest` with `pytest-xdist`: `pip install pytest pytest-xdist`

2. Go to the tests directory: `cd $MESC_REPO/mesc/tests`

3. Run one of these commands:

|description | command |
| --- | --- |
| run tests | `pytest` |
| run test in parallel mode (much faster) | `pytest -n auto` |
| run tests in debug mode (helpful for debugging) | `pytest --pdb` |
| run tests only tests that previously failed | `pytest --lf` |
| run tests for specific adapters only | `pytest --adapters adapters/python adapters/cli` |

By default, tests will run for all MESC implementations. If you do not have all of these implementations installed, you will need to use `--adapters` to select only the subset that that you have installed.


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
- `test_mesc.py` packages MESC tests into form usable by pypi

