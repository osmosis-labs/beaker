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

* `--title <title>`: Proposal title

* `-d/--description <description>`: Proposal decsription

* `--deposit <deposit>`: Proposal deposit to activate voting

* `-n/--network <network>`

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height

---

### `beaker wasm proposal vote`

Vote for proposal

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <contract-name>`Name of the contract to store

* `-o/--option <option>`: Vote option, one of: yes, no, no_with_veto, abstain

* `-n/--network <network>`

* `--gas <gas>`: Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`

* `--gas-limit <gas-limit>`: Limit to how much gas amount allowed to be consumed

* `--signer-account <signer-account>`: Specifies predefined account as a tx signer

* `--signer-mnemonic <signer-mnemonic>`: Specifies mnemonic as a tx signer

* `--signer-private-key <signer-private-key>`: Specifies private_key as a tx signer (base64 encoded string)

* `-t/--timeout-height <timeout-height>`: Specifies a block timeout height to prevent the tx from being committed past a certain height

---

### `beaker wasm proposal query`

[\> `beaker wasm proposal query`'s subcommands](./beaker_wasm_proposal_query.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information