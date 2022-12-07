beaker

# beaker

## Table of contents

### Classes

- [Account](classes/Account.md)
- [Contract](classes/Contract.md)

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

[src/utils.ts:21](https://github.com/osmosis-labs/beaker/blob/47fee14/ts/beaker-console/src/utils.ts#L21)

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

[src/account.ts:92](https://github.com/osmosis-labs/beaker/blob/47fee14/ts/beaker-console/src/account.ts#L92)

___

### getContracts

▸ **getContracts**(`client`, `state`, `sdk`): `Record`<`string`, `unknown`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `client` | `CosmWasmClient` |
| `state` | `Record`<`string`, `unknown`\> |
| `sdk` | `Object` |
| `sdk.contracts` | `Record`<`string`, `Record`<`string`, `Function`\>\> |

#### Returns

`Record`<`string`, `unknown`\>

#### Defined in

[src/contract.ts:63](https://github.com/osmosis-labs/beaker/blob/47fee14/ts/beaker-console/src/contract.ts#L63)
