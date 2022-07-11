# `beaker key`

Managing key backed by system's secret store

Arguments:

* `--help`: Print help information

* `--version`: Print version information

## Subcommands

### `beaker key set`

Create new key or update existing key

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <name>`Name of the key to create or update

* ` <mnemonic>`Mnemonic string to store as an entry

* `-y/--yes`: Agree to all prompts

---

### `beaker key delete`

Delete existing key

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <name>`Name of the key to create or update

* `-y/--yes`: Agree to all prompts

---

### `beaker key address`

Get address from keyring's stored key

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <name>`Name of the key to create or update

---

### `beaker key generate`

Generate new mnemonic

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <name>`Name of the key to create or update

* `--show`: Show mnemonic in the console if set, keep it secret otherwise

* `-y/--yes`: Agree to all prompts