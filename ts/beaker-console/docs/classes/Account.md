[beaker](../README.md) / Account

# Class: Account

## Table of contents

### Constructors

- [constructor](Account.md#constructor)

### Properties

- [address](Account.md#address)
- [signingClient](Account.md#signingclient)
- [wallet](Account.md#wallet)

### Methods

- [getBalance](Account.md#getbalance)
- [withDerivedAddress](Account.md#withderivedaddress)

## Constructors

### constructor

• **new Account**(`wallet`, `signingClient`, `address`)

#### Parameters

| Name | Type |
| :------ | :------ |
| `wallet` | `Secp256k1HdWallet` |
| `signingClient` | `SigningCosmWasmClient` |
| `address` | `string` |

#### Defined in

[src/account.ts:28](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L28)

## Properties

### address

• **address**: `string`

#### Defined in

[src/account.ts:26](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L26)

___

### signingClient

• **signingClient**: `SigningCosmWasmClient`

#### Defined in

[src/account.ts:24](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L24)

___

### wallet

• **wallet**: `Secp256k1HdWallet`

#### Defined in

[src/account.ts:25](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L25)

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

[src/account.ts:52](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L52)

___

### withDerivedAddress

▸ `Static` **withDerivedAddress**(`wallet`, `signingClient`): `Promise`<[`Account`](Account.md)\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `wallet` | `Secp256k1HdWallet` |
| `signingClient` | `SigningCosmWasmClient` |

#### Returns

`Promise`<[`Account`](Account.md)\>

#### Defined in

[src/account.ts:38](https://github.com/osmosis-labs/beaker/blob/213f82c/ts/beaker-console/src/account.ts#L38)
