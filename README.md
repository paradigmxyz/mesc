
# Multiple Endpoint Single Configuration Standard (MESC)

MESC is a specification for how EVM tools can configure their RPC endpoints.

By following this specification, a user creates a single RPC configuration that can be used by all compliant tools in an OS-agnostic and language-agnostic way.

## Specification

The MESC specification is defined in [SPECIFICATION.md](./SPECIFICATION.md).

## Reference Implementations

Reference implementations are provided for each of the following:
- [cli](/cli) [TODO]
- [javascript](/javascript) [TODO]
- [python](/python) [TODO]
- [rust](/rust) [TODO]

These implementations provide a consistent language-agnostic interface while obeying the natural conventions of each language.

Additionally, the CLI implementation contains utilities for creating, modifying, and validating an environment's MESC configuration.

## Examples [TODO]

Examples are provided in the [`examples/`](/examples) directory.

These examples demonstrate how MESC can be used in a wide variety of circumstances.

## Tests [TODO]

Tests are provided in the [`tests/`](/tests) directory.

These tests validate that a MESC implementation is fully compliant with the specification.

