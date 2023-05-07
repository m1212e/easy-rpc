This package is used for integration tests. In aims to provide an example setup using the locally generated versions of the various components of easy-rpc. It uses npm as build tool.
You need other tools installed on you system to build the various dependencies of this project. Please see the docs of the various dependencies for installation instructions.

## Building
In most cases we want to verify that the project builds without any errors. To do this there are a few npm scripts available. Use `npm run build` to build the dependencies and the project itself. This ensures that the newest versions of the local code are built and used. For details on what the npm scripts do, see the [package.json](./package.json).