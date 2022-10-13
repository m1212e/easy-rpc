The src directory contains all sourcecode. Files providing configuration, scripts or similar do not belong here. Since this is a binary rust application the main entry into the program is located inside the [main.rs](./main.rs) file.

### util

The [util module](./util/) contains various util/helper functions, macros etc. which are not clearly assignable to a specific module and have a more general use.

### transpiler

The [transpiler module](./transpiler/) is used for converting the easy-rpc declarations into actual code of a target language.

## Note about module structure

The project uses `mod.rs` files to declare rust modules. They often only export sub-modules. If there are functions which belong to a more general part of a specific module, they can but not must be located inside the `mod.rs` files. There is no rule to follow on where to put functions, as long as things are tidy, clearly labeled and reasonably organized.

Most modules come with tests. These tests should be located in test modules/directories and are located within the module directory they belong to. They are named `tests`.
