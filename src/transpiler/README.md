The transpiler module is used for converting the easy-rpc declarations into actual code of a target language.

### generator
The [generator module](./generator/) provides functions to generate code from already parsed declatations.

### parser
The [parser module](./parser/) provides functions to parse erpc declarations.

The [mod.rs](./mod.rs) file provides a ``run`` function which can be called to run the transpiler on an input directory. It parses the config, source and target paths and generates files all on itself. The transpiler is one of the core features of this program and should be able to work decoupled and on its own, without the need of any input besides the target directory. Besides that [mod.rs](./mod.rs) defines an ERPCError type which unifies all types of errors which can occur during running the transpiler.

[config.rs](./mod.rs) provides methods and types for parsing the configuration files easy-rpc expects. It can parse the config.json which is located in every easy-rpc project and the roles.json which is located at every easy-rpc source directory. See the docs for examples on where and how these config files are used.