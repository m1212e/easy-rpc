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

Navigate to an erpc project root (where the erpc.json is located) or any parent directory withing 100 levels and run the binary. Use flags as arguments for different modes to run:
| Flag | Mode |
| ----------- | -------------------------------------------------------------- |
| [None] | runs the transpiler exactly once |
| -w | runs the transpiler when a source file changes |
| -ls | runs the transpiler with [-w] and the language server on stdio |

In case you do not want to place the binary inside or at a parent directory you can use the -p flag to set a relative or absolute path to run the transpiler in:

```
# relative to current dir
./easy-rpc -p ./frontend
# or absolute
./easy-rpc -p /my/absolute/path/frontend
```

## License

easy-rpc is licensed unter [Apache 2.0 with the Commons Clause](https://github.com/m1212e/easy-rpc/blob/main/LICENSE). By contributing to easy-rpc you agree that your contribution will be licensed under its license.
