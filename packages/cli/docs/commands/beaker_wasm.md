# `beaker wasm`

Manipulating and interacting with CosmWasm contract

## Subcommands

---

### `beaker wasm new`

Create new CosmWasm contract from boilerplate

Arguments:

* `<CONTRACT_NAME>` Contract name

* `-t / --target-dir <TARGET_DIR>`: Path to store generated contract

* `-v / --version <VERSION>`: Template's version, using main branch if not specified

---

### `beaker wasm build`

Build .wasm for storing contract code on the blockchain

Arguments:

* `--no-wasm-opt <NO_WASM_OPT>`: If set, the contract(s) will not be optimized by wasm-opt after build (only use in dev)

* `-a / --aarch64 <AARCH64>`: Option for m1 user for wasm optimization, FOR TESTING ONLY, PRODUCTION BUILD SHOULD USE INTEL BUILD

---

### `beaker wasm store-code`

Store .wasm on chain for later initialization

Arguments:

* `<CONTRACT_NAME>` Name of the contract to store

* `--no-wasm-opt <NO_WASM_OPT>`: If set, use non wasm-opt optimized wasm to store code (only use in dev)

* `--permit-instantiate-only <PERMIT_INSTANTIATE_ONLY>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm ts-gen`

Arguments:

* `<CONTRACT_NAME>` Name of the contract to store

* `--schema-gen-cmd <SCHEMA_GEN_CMD>`: Sschema generation command, default: `cargo schema`

* `--out-dir <OUT_DIR>`: Code output directory, ignore remaining ts build process if custom out_dir is specified

* `--node-package-manager <NODE_PACKAGE_MANAGER>`: Code output directory (default: `yarn`)

---

### `beaker wasm update-admin`

Update admin that can migrate contract

Arguments:

* `<CONTRACT_NAME>` Name of the contract to store

* `-l / --label <LABEL>`: Label for the instantiated contract for later reference (default: `default`)

* `--new-admin <NEW_ADMIN>`: Address of new admin

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm clear-admin`

Clear admin so no one can migrate contract

Arguments:

* `<CONTRACT_NAME>` Name of the contract to store

* `-l / --label <LABEL>`: Label for the instantiated contract for later reference (default: `default`)

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm instantiate`

Instanitate .wasm stored on chain

Arguments:

* `<CONTRACT_NAME>` Name of the contract to instantiate

* `-l / --label <LABEL>`: Label for the instantiated contract for later reference (default: `default`)

* `-r / --raw <RAW>`: Raw json string to use as instantiate msg

* `--admin <ADMIN>`: Specifying admin required for contract migration. Use "signer" for setting tx signer as admin. Use bech32 address (eg. "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks") for custom admin

* `-f / --funds <FUNDS>`: Funds to send to instantiated contract

* `--no-proposal-sync <NO_PROPOSAL_SYNC>`: Skip the check for proposal's updated code_id

* `-y / --yes <YES>`: Agree to all prompts

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm migrate`

Migrated instanitate contract to use other code stored on chain

Arguments:

* `<CONTRACT_NAME>` Name of the contract to instantiate

* `-l / --label <LABEL>`: Label for the instantiated contract for selcting migration target (default: `default`)

* `-r / --raw <RAW>`: Raw json string to use as instantiate msg

* `--no-proposal-sync <NO_PROPOSAL_SYNC>`: Skip the check for proposal's updated code_id

* `-y / --yes <YES>`: Agree to all prompts

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm deploy`

Build, Optimize, Store code, and instantiate contract

Arguments:

* `<CONTRACT_NAME>` Name of the contract to deploy

* `-l / --label <LABEL>`: Label for the instantiated contract for later reference (default: `default`)

* `-r / --raw <RAW>`: Raw json string to use as instantiate msg

* `--permit-instantiate-only <PERMIT_INSTANTIATE_ONLY>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `--admin <ADMIN>`: Specifying admin required for contract migration. Use "signer" for setting tx signer as admin. Use bech32 address (eg. "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks") for custom admin

* `-f / --funds <FUNDS>`: Funds to send to instantiated contract

* `--no-rebuild <NO_REBUILD>`: Use existing .wasm file to deploy if set to true

* `--no-wasm-opt <NO_WASM_OPT>`: If set, skip wasm-opt and store the unoptimized code (only use in dev)

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm upgrade`

Build, Optimize, Store code, and migrate contract

Arguments:

* `<CONTRACT_NAME>` Name of the contract to deploy

* `-l / --label <LABEL>`: Label for the instantiated contract for later reference (default: `default`)

* `-r / --raw <RAW>`: Raw json string to use as instantiate msg

* `--no-rebuild <NO_REBUILD>`: Use existing .wasm file to deploy if set to true

* `--no-wasm-opt <NO_WASM_OPT>`: If set, skip wasm-opt and store the unoptimized code (only use in dev)

* `--permit-instantiate-only <PERMIT_INSTANTIATE_ONLY>`: Restricting the code to be able to instantiate only by given address, no restriction by default

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm proposal`

[\> `beaker wasm proposal`'s subcommands](./beaker_wasm_proposal.md)

---

### `beaker wasm execute`

Execute contract messages

Arguments:

* `<CONTRACT_NAME>`

* `-l / --label <LABEL>` (default: `default`)

* `-r / --raw <RAW>`

* `-f / --funds <FUNDS>`

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch

---

### `beaker wasm query`

Query contract state

Arguments:

* `<CONTRACT_NAME>`

* `-l / --label <LABEL>` (default: `default`)

* `-r / --raw <RAW>`

* `-n / --network <NETWORK>`: Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config (default: `local`)

* `--gas <GAS>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <GAS_LIMIT>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <SIGNER_ACCOUNT>`: Specifies predefined account as a tx signer

* `--signer-keyring <SIGNER_KEYRING>`: Use the OS secure store as backend to securely store your key. To manage them, you can find more information [here](docs/commands/beaker_key.md)

* `--signer-mnemonic <SIGNER_MNEMONIC>`: Specifies mnemonic as a tx signer

* `--signer-private-key <SIGNER_PRIVATE_KEY>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t / --timeout-height <TIMEOUT_HEIGHT>`: Specifies a block timeout height to prevent the tx from being committed past a certain height (default: `0`)

* `-a / --account-sequence <ACCOUNT_SEQUENCE>`: Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain. This is useful if there is an account sequence mismatch