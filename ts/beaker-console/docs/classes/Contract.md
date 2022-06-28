[beaker](../README.md) / Contract

# Class: Contract

## Table of contents

### Constructors

- [constructor](Contract.md#constructor)

### Properties

- [address](Contract.md#address)
- [client](Contract.md#client)

### Methods

- [execute](Contract.md#execute)
- [getCode](Contract.md#getcode)
- [getInfo](Contract.md#getinfo)
- [query](Contract.md#query)

## Constructors

### constructor

• **new Contract**(`address`, `client`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `address` | `string` |
| `client` | `CosmWasmClient` |

#### Defined in

[src/contract.ts:11](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L11)

## Properties

### address

• **address**: `string`

#### Defined in

[src/contract.ts:8](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L8)

___

### client

• **client**: `CosmWasmClient`

#### Defined in

[src/contract.ts:9](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L9)

## Methods

### execute

▸ **execute**(`xmsg`, `senderAddress`, `fee?`): `Object`

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `xmsg` | `Msg` | `undefined` |
| `senderAddress` | ``null`` \| `string` | `undefined` |
| `fee` | `number` \| `StdFee` \| ``"auto"`` | `'auto'` |

#### Returns

`Object`

| Name | Type |
| :------ | :------ |
| `by` | (`account`: [`Account`](Account.md)) => `Promise`<`ExecuteResult`\> |

#### Defined in

[src/contract.ts:28](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L28)

___

### getCode

▸ **getCode**(): `Promise`<`CodeDetails`\>

#### Returns

`Promise`<`CodeDetails`\>

#### Defined in

[src/contract.ts:20](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L20)

___

### getInfo

▸ **getInfo**(): `Promise`<`Contract`\>

#### Returns

`Promise`<`Contract`\>

#### Defined in

[src/contract.ts:16](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L16)

___

### query

▸ **query**(`qmsg`): `Promise`<`any`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `qmsg` | `Msg` |

#### Returns

`Promise`<`any`\>

#### Defined in

[src/contract.ts:24](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/contract.ts#L24)
