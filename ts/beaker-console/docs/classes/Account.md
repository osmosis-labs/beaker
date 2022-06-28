[beaker](../README.md) / Account

# Class: Account

## Table of contents

### Constructors

- [constructor](Account.md#constructor)

### Properties

- [signingClient](Account.md#signingclient)
- [wallet](Account.md#wallet)

### Methods

- [getBalance](Account.md#getbalance)

## Constructors

### constructor

• **new Account**(`wallet`, `signingClient`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `wallet` | `Secp256k1HdWallet` |
| `signingClient` | `SigningCosmWasmClient` |

#### Defined in

[src/account.ts:24](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/account.ts#L24)

## Properties

### signingClient

• **signingClient**: `SigningCosmWasmClient`

#### Defined in

[src/account.ts:21](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/account.ts#L21)

___

### wallet

• **wallet**: `Secp256k1HdWallet`

#### Defined in

[src/account.ts:22](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/account.ts#L22)

## Methods

### getBalance

▸ **getBalance**(`denom`): `Promise`<`Coin`\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `denom` | `string` |

#### Returns

`Promise`<`Coin`\>

#### Defined in

[src/account.ts:29](https://github.com/osmosis-labs/beaker/blob/fd6d200/ts/beaker-console/src/account.ts#L29)
