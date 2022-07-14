# `beaker`

CosmWasm development tooling

Version: 0.0.6

Arguments:

* `--help`: Print help information

* `--version`: Print version information

## Subcommands

### `beaker new`

Create new workspace from boilerplate

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* ` <name>`Workspace name

* `-t/--target-dir <target-dir>`: Path to store generated workspace

* `-b/--branch <branch>`: Template's branch, using main if not specified

---

### `beaker wasm`

Manipulating and interacting with CosmWasm contract

[\> `beaker wasm`'s subcommands](./beaker_wasm.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information

---

### `beaker key`

Managing key backed by system's secret store

[\> `beaker key`'s subcommands](./beaker_key.md)

Arguments:

* `--help`: Print help information

* `--version`: Print version information

---

### `beaker console`

Launch interactive console for interacting with the project

Arguments:

* `--help`: Print help information

* `--version`: Print version information

* `-n/--network <network>` (default: `local`)