
# MESC Tests

These tests measure the degree to which a MESC implementation is compliant with the MESC specification.

These tests are language-agnostic, allowing the same tests to be run against each MESC implementation.

## Usage
1. `generate.py` generates tests in the `generated/` directory
2. `run.py adapters/<ADAPTOR>` runs the tests for the given adapter

## Adapters

Each MESC implementation has an adapter that allows running the tests using the test runner.

To make a custom adapter:
1. adapter should be a script that takes a JSON `Query` as its single argument
2. the adapter should run the query and output the result as JSON
3. if the config loading or the query fails, simply output the word `FAIL`

The adapter should never crash upon failure, just print the word `FAIL`

