# Migration

## v0.0.x -> v0.1.x

0.1.0 makes additional template assumptions:

1. use programmatic approach for fo `ts-codegen` so it expects [`ts/sdk/scripts/codegen.js`](https://github.com/osmosis-labs/beaker/blob/v0.1.0/templates/project/ts/sdk/scripts/codegen.js) to exist in the project directory
2. since 1. use `@cosmwasm/ts-codegen`, you need to [add it as dev dependency](https://github.com/osmosis-labs/beaker/blob/v0.1.0/templates/project/ts/sdk/package.json#L23) so that beaker can all the script metioned in 1.
