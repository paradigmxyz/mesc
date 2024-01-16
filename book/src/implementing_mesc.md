# Implementing MESC

One of MESC's goals is to have an implementation for every programming language in the cryptocurrency ecosystem.

If you are creating a new MESC implementation, you may find the following useful:

1. The main functionality that a MESC implementation provides is the [Core MESC Interface](https://github.com/paradigmxyz/mesc/blob/main/SPECIFICATION.md#reference-implementation). This is a set of functions for reading whatever MESC data a user has configured on their system.

2. The ability to **write** MESC configurations is useful, but should be considered a secondary goal. Users already have the ability to create MESC configurations either by 1) editing JSON by hand, 2) editing JSON programmatically, or 3) using the interactive [MESC cli tool](https://github.com/paradigmxyz/mesc/tree/main/cli).

3. A MESC implementation's compliance to the specification can be checked using the language-agnostic MESC test suite. Passing the test suite means that an implementation is complete. More details on the test suite can be found [here](https://github.com/paradigmxyz/mesc/tree/main/tests).

4. It is desirable to use the same names, types, and behaviors across each MESC implementation. This increases interoperability and makes MESC easier to learn and use. However it is also desirable to obey the common convetions of each programming language. Each MESC implementation must find a balance between language-agnostic conventions vs language-specific conventions.

5. If parts of the MESC specification are confusing or difficult to implement, it may be helpful to look at existing implementations. These currently exist for [python](https://github.com/paradigmxyz/mesc/tree/main/python) and [rust](https://github.com/paradigmxyz/mesc/tree/main/rust).

