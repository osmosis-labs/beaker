# Migration

## v0.0.x -> v0.1.x

0.1.0 makes additional template assumptions:

1. use programmatic approach for fo `ts-codegen` so it expects [`ts/sdk/scripts/codegen.js`](https://github.com/osmosis-labs/beaker/blob/v0.1.0/templates/project/ts/sdk/scripts/codegen.js) to exist in the project directory
2. [add `codegen` script](https://github.com/osmosis-labs/beaker/blob/75e11a4943af7ecc4169c1e5f800f5aa6978855c/templates/project/ts/sdk/package.json#L53) to `package.json`
3. since 1. use `@cosmwasm/ts-codegen`, you need to [add it as dev dependency](https://github.com/osmosis-labs/beaker/blob/v0.1.0/templates/project/ts/sdk/package.json#L23)
4. make sure `ts/sdk/index.ts` exports contracts as the following
```ts
export * as contracts from "./contracts";
```
