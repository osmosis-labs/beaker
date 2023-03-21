# `beaker wasm proposal`

Arguments:

* `--help`: Print help information

* `--version`: Print version information

## Subcommands

### `beaker wasm proposal store-code`

Proposal for storing .wasm on chain for later initialization

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `--permit-instantiate-only <permit-instantiate-only>`: Restricting the code to be able to instantiate/migrate only by given address, no restriction by default

* `-p/--proposal <proposal>`: Path to proposal file, could be either yaml / toml format

* `--title <title>`: Proposal title (default: ``)

* `--description <description>`: Proposal decsription (default: ``)

* `--deposit <deposit>`: Proposal deposit to activate voting

* `--repo <repo>`: Public repository of the code (default: ``)

* `--rust-flags <rust-flags>`: RUST_FLAGS that passed while compiling to wasm If building with Beaker, it's usually "-C link-arg=-s"

* `--optimizer <optimizer>`: Type and version of the [optimizer](https://github.com/CosmWasm/rust-optimizer), either: rust-optimizer:<version> or workspace-optimizer:<version>. Beaker use workspace-optimizer, the version, if not manually configured, can be found in `wasm` config doc

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

### `beaker wasm proposal vote`

Vote for proposal

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `-o/--option <option>`: Vote option, one of: yes, no, no_with_veto, abstain

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

### `beaker wasm proposal query`

[\> `beaker wasm proposal query`'s subcommands](./beaker_wasm_proposal_query.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information