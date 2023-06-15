# `beaker`

CosmWasm swiss-army knife configured for Osmosis by default, but trivial to make it work for other CosmWasm enabled chain.

Version: 0.1.5

## Subcommands

---

### `beaker new`

Create new workspace from boilerplate

Arguments:

* `<NAME>` Workspace name

* `-t / --target-dir <TARGET_DIR>`: Path to store generated workspace

* `-b / --branch <BRANCH>`: Template's branch, using main if not specified

---

### `beaker wasm`

Manipulating and interacting with CosmWasm contract

[\> `beaker wasm`'s subcommands](./beaker_wasm.md)

---

### `beaker key`

Managing key backed by system's secret store

[\> `beaker key`'s subcommands](./beaker_key.md)

---

### `beaker console`

Launch interactive console for interacting with the project

Arguments:

* `-n / --network <NETWORK>` (default: `local`)

---

### `beaker task`

Managing tasks for the project

[\> `beaker task`'s subcommands](./beaker_task.md)
