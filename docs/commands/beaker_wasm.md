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

* `--permit-instantiate-only <permit-instantiate-only>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm ts-gen`

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `--schema-gen-cmd <schema-gen-cmd>`: Sschema generation command, default: `cargo schema`

* `--out-dir <out-dir>`: Code output directory, ignore remaining ts build process if custom out_dir is specified

* `--node-package-manager <node-package-manager>`: Code output directory (default: `yarn`)

---

### `beaker wasm update-admin`

Update admin that can migrate contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `--new-admin <new-admin>`: Address of new admin

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm clear-admin`

Clear admin so no one can migrate contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm instantiate`

Instanitate .wasm stored on chain

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to instantiate

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `--admin <admin>`: Specifying admin required for contract migration. Use "signer" for setting tx signer as admin. Use bech32 address (eg. "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks") for custom admin

* `-f/--funds <funds>`: Funds to send to instantiated contract

* `--no-proposal-sync`: Skip the check for proposal's updated code_id

* `-y/--yes`: Agree to all prompts

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm migrate`

Migrated instanitate contract to use other code stored on chain

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to instantiate

* `-l/--label <label>`: Label for the instantiated contract for selcting migration target (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `--no-proposal-sync`: Skip the check for proposal's updated code_id

* `-y/--yes`: Agree to all prompts

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm deploy`

Build, Optimize, Store code, and instantiate contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to deploy

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `--permit-instantiate-only <permit-instantiate-only>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `--admin <admin>`: Specifying admin required for contract migration. Use "signer" for setting tx signer as admin. Use bech32 address (eg. "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks") for custom admin

* `-f/--funds <funds>`: Funds to send to instantiated contract

* `--no-rebuild`: Use existing .wasm file to deploy if set to true

* `--no-wasm-opt`: If set, skip wasm-opt and store the unoptimized code (only use in dev)

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm upgrade`

Build, Optimize, Store code, and migrate contract

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to deploy

* `-l/--label <label>`: Label for the instantiated contract for later reference (default: `default`)

* `-r/--raw <raw>`: Raw json string to use as instantiate msg

* `--no-rebuild`: Use existing .wasm file to deploy if set to true

* `--no-wasm-opt`: If set, skip wasm-opt and store the unoptimized code (only use in dev)

* `--permit-instantiate-only <permit-instantiate-only>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm proposal`

[\> `beaker wasm proposal`'s subcommands](./beaker_wasm_proposal.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information

---

### `beaker wasm execute`

Execute contract messages

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`

* `-l/--label <label>` (default: `default`)

* `-r/--raw <raw>`

* `-f/--funds <funds>`

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm query`

Query contract state

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`

* `-l/--label <label>` (default: `default`)

* `-r/--raw <raw>`

* `-n/--network <network>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-keyring <signer-keyring>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a/--account-sequence <account-sequence>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch