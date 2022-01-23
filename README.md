# ns3-parallel

A Multitask Parallel Concurrent Executor for ns-3 (network simulator).

## Usage

Define your config struct and param struct, implement trait `BuildParam` for the first and trait `BuildCmd` for the second.

Then call the `ExecutorBuilder` to build a `Executor`. Then launch the tasks, wait for the results.

Examples see `examples/simple.rs`.

To run the example, you can first execute the script `setup-ns3.sh` then execute `cargo run --example simple` in the root directory.

## Maintainer

[@BobAnkh](https://github.com/BobAnkh)

## How to contribute

You should follow our [Code of Conduct](/CODE_OF_CONDUCT.md).

See [CONTRIBUTING GUIDELINES](/CONTRIBUTING.md) for contributing conventions.

Make sure to pass all the tests before submitting your code.

### Contributors

## LICENSE

[Apache-2.0](LICENSE) © BobAnkh
