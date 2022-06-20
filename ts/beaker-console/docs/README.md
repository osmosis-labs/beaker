beaker

# beaker

## Table of contents

### Functions

- [extendWith](README.md#extendwith)
- [getAccounts](README.md#getaccounts)
- [getContracts](README.md#getcontracts)

## Functions

### extendWith

▸ **extendWith**(`properties`): (`context`: `Record`<`string`, `unknown`\>) => `void`

#### Parameters

| Name | Type |
| :------ | :------ |
| `properties` | `Record`<`string`, `unknown`\> |

#### Returns

`fn`

▸ (`context`): `void`

##### Parameters

| Name | Type |
| :------ | :------ |
| `context` | `Record`<`string`, `unknown`\> |

##### Returns

`void`

#### Defined in

[src/utils.ts:15](https://github.com/osmosis-labs/beaker/blob/9716da3/ts/beaker/src/utils.ts#L15)

___

### getAccounts

▸ **getAccounts**(`conf`, `network`): `Promise`<{ `[k: string]`: `T`;  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `conf` | `Config` |
| `network` | `string` |

#### Returns

`Promise`<{ `[k: string]`: `T`;  }\>

#### Defined in

[src/account.ts:67](https://github.com/osmosis-labs/beaker/blob/9716da3/ts/beaker/src/account.ts#L67)

___

### getContracts

▸ **getContracts**(`client`, `state`): `Record`<`string`, `unknown`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `client` | `CosmWasmClient` |
| `state` | `Record`<`string`, `unknown`\> |

#### Returns

`Record`<`string`, `unknown`\>

#### Defined in

[src/contract.ts:6](https://github.com/osmosis-labs/beaker/blob/9716da3/ts/beaker/src/contract.ts#L6)
