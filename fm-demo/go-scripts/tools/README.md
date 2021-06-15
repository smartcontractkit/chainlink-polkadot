# Tools

Here are some QoL tools used by the framework.

## `tools.go`

Basic import for the [ginkgo test framework](https://github.com/onsi/ginkgo)

## `compile_contracts.py`

A proof of concept script to conveniently compile solidity source and generate golang bindings.

Run with `python3 ./tools/compile_contracts.py`

This will:

1. Install a local version of `hardhat`
2. Use `hardhat` to compile solidity source code
3. Use `abigen` to generate golang bindings for the compiled contracts
4. Cleanup `hardhat` installation and files

