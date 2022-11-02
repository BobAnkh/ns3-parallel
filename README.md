# ns3-parallel

[![github-repo](https://img.shields.io/badge/github-BobAnkh/ns3--parallel-f5dc23?logo=github)](https://github.com/BobAnkh/ns3-parallel)
[![crates.io](https://img.shields.io/crates/v/ns3-parallel.svg?logo=rust)](https://crates.io/crates/ns3-parallel)
[![docs.rs](https://img.shields.io/badge/docs.rs-ns3--parallel-blue?logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/ns3-parallel)
[![LICENSE Apache-2.0](https://img.shields.io/github/license/BobAnkh/ns3-parallel?logo=Apache)](https://github.com/BobAnkh/ns3-parallel/blob/main/LICENSE)

A Multitask Parallel Concurrent Executor for ns-3 (network simulator).

## Usage

Define your config struct and param struct, implement trait `BuildParam` for the first and trait `BuildCmd` for the second.

Then call the `ExecutorBuilder` to build a `Executor`. Then launch the tasks, wait for the results.

Examples see `examples/simple.rs`.

To run the example, you can first execute the script `setup-ns3.sh` then execute `cargo run --example simple` in the root directory.

Currently support 4 config file formats: toml, ron, json, yaml. Example config files can see `config.toml` and `config.ron` under root. **Welcome contributions for any new config format**.

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
