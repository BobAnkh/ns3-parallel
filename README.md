# ns3-parallel

[![crates.io](https://img.shields.io/crates/v/ns3-parallel.svg)](https://crates.io/crates/ns3-parallel)
[![LICENSE Apache-2.0](https://img.shields.io/github/license/BobAnkh/ns3-parallel?logo=Apache)](https://github.com/BobAnkh/ns3-parallel/[blob/main/LICENSE)
[![docs.rs](https://img.shields.io/badge/docs.rs-ns3--parallel-blue)](https://docs.rs/ns3-parallel)

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

<table>
<tr>
    <td align="center" style="word-wrap: break-word; width: 150.0; height: 150.0">
        <a href=https://github.com/BobAnkh>
            <img src=https://avatars.githubusercontent.com/u/44333669?v=4 width="100;"  style="border-radius:50%;align-items:center;justify-content:center;overflow:hidden;padding-top:10px" alt=BobAnkh/>
            <br />
            <sub style="font-size:14px"><b>BobAnkh</b></sub>
        </a>
    </td>
</tr>
</table>

## LICENSE

[Apache-2.0](LICENSE) © BobAnkh
