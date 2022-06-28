[beaker](../README.md) / Contract

# Class: Contract

Contract instance with baked-in client

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

[src/contract.ts:20](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L20)

## Properties

### address

• **address**: `string`

#### Defined in

[src/contract.ts:17](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L17)

___

### client

• **client**: `CosmWasmClient`

#### Defined in

[src/contract.ts:18](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L18)

## Methods

### execute

▸ **execute**(`xmsg`, `senderAddress`, `fee?`): `Object`

Execute the contract
usage: `contract.execute(xmsg).by(signerAccount)`

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

[src/contract.ts:52](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L52)

___

### getCode

▸ **getCode**(): `Promise`<`CodeDetails`\>

Get code details

#### Returns

`Promise`<`CodeDetails`\>

#### Defined in

[src/contract.ts:35](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L35)

___

### getInfo

▸ **getInfo**(): `Promise`<`Contract`\>

Get contract info

#### Returns

`Promise`<`Contract`\>

#### Defined in

[src/contract.ts:28](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L28)

___

### query

▸ **query**(`qmsg`): `Promise`<`unknown`\>

Query the contract by passing query message

**`params`** qmsg the query message

#### Parameters

| Name | Type |
| :------ | :------ |
| `qmsg` | `Msg` |

#### Returns

`Promise`<`unknown`\>

query result

#### Defined in

[src/contract.ts:44](https://github.com/osmosis-labs/beaker/blob/e6fd0d1/ts/beaker-console/src/contract.ts#L44)
