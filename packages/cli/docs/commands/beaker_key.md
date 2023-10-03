# `beaker key`

Managing key backed by system's secret store

## Subcommands

---

### `beaker key set`

Create new key or update existing key

Arguments:

* `<NAME>` Name of the key to create or update

* `<MNEMONIC>` Mnemonic string to store as an entry

* `-y / --yes <YES>`: Agree to all prompts

---

### `beaker key delete`

Delete existing key

Arguments:

* `<NAME>` Name of the key to create or update

* `-y / --yes <YES>`: Agree to all prompts

---

### `beaker key address`

Get address from keyring's stored key

Arguments:

* `<NAME>` Name of the key to create or update

---

### `beaker key generate`

Generate new mnemonic

Arguments:

* `<NAME>` Name of the key to create or update

* `--show <SHOW>`: Show mnemonic in the console if set, keep it secret otherwise

* `-y / --yes <YES>`: Agree to all prompts