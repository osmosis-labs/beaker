# `beaker wasm`

Manipulating and interacting with CosmWasm contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

## Subcommands

### `beaker wasm new`

Create new CosmWasm contract from boilerplate

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Contract name

* `-t/--target-dir <target-dir>`: Path to store generated contract

* `-v/--version <version>`: Template's version, using main branch if not specified

---

### `beaker wasm build`

Build .wasm for storing contract code on the blockchain

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* `--no-wasm-opt`: If set, the contract(s) will not be optimized by wasm-opt after build (only use in dev)

* `-a/--aarch64`: Option for m1 user for wasm optimization, FOR TESTING ONLY, PRODUCTION BUILD SHOULD USE INTEL BUILD

---

### `beaker wasm store-code`

Store .wasm on chain for later initialization

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `--no-wasm-opt`: If set, use non wasm-opt optimized wasm to store code (only use in dev)

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

---

### `beaker wasm instantiate`

Instanitate .wasm stored on chain

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to instantiate

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `-f/--funds <funds>`: Funds to send to instantiated contract

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

---

### `beaker wasm deploy`

Build, Optimize, Store code, and instantiate contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to deploy

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `-f/--funds <funds>`: Funds to send to instantiated contract

* `--no-rebuild`: Use existing .wasm file to deploy if set to true

* `--no-wasm-opt`: If set, skip wasm-opt and store the unoptimized code (only use in dev)

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

---

### `beaker wasm proposal`

[\> `beaker wasm proposal`'s subcommands](./beaker_wasm_proposal.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information