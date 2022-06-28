[beaker](../README.md) / Account

# Class: Account

Account instance with baked-in client and utility methods

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

[src/account.ts:27](https://github.com/osmosis-labs/beaker/blob/c77da51/ts/beaker-console/src/account.ts#L27)

## Properties

### signingClient

• **signingClient**: `SigningCosmWasmClient`

#### Defined in

[src/account.ts:24](https://github.com/osmosis-labs/beaker/blob/c77da51/ts/beaker-console/src/account.ts#L24)

___

### wallet

• **wallet**: `Secp256k1HdWallet`

#### Defined in

[src/account.ts:25](https://github.com/osmosis-labs/beaker/blob/c77da51/ts/beaker-console/src/account.ts#L25)

## Methods

### getBalance

▸ **getBalance**(`denom`): `Promise`<`Coin`\>

Get balances for specific denom, only support native coin

#### Parameters

| Name | Type |
| :------ | :------ |
| `denom` | `string` |

#### Returns

`Promise`<`Coin`\>

#### Defined in

[src/account.ts:35](https://github.com/osmosis-labs/beaker/blob/c77da51/ts/beaker-console/src/account.ts#L35)
