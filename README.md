# easy-rpc
![test](https://github.com/m1212e/easy-rpc/actions/workflows/test.yml/badge.svg)
[![coverage](https://codecov.io/gh/m1212e/easy-rpc/branch/main/graph/badge.svg?token=3OCL7W9E4L)](https://codecov.io/gh/m1212e/easy-rpc)

Web requests made easy.
See the docs at https://m1212e.github.io/easy-rpc-docs/

This repository contains README.md files to explain the important parts and modules throughout the repository. If you want to dive into the sourcecode, take a look at the [src](./src/) directory.

## Building
Build the project with cargo (rust toolchain must be installed)
```
cargo build
```
or for optimized binary
```
cargo build --release
```

## Run the binary
Navigate to an erpc project root (where the erpc.json is located) or any parent directory withing 100 levels and run the binary. Use flags as arguments for different functionality:
| Flag        | Mode                                                           |
| ----------- | -------------------------------------------------------------- |
| [None]      | runs the transpiler exactly once                               |
| -w          | runs the transpiler when a source file changes                 |
| -ls         | runs the transpiler with [-w] and the language server on stdio |

## License
easy-rpc is licensed unter [Apache 2.0 with the Commons Clause](https://github.com/m1212e/easy-rpc/blob/main/LICENSE). By contributing to easy-rpc you agree that your contribution will be licensed under its license.
