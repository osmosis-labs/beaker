<a href="https://docs.osmosis.zone/developing/dapps/get_started/">
    <img src="assets/beaker.png" alt="Beaker logo" title="Beaker" align="right" height="60" />
</a>

# Beaker

![crates.io](https://img.shields.io/crates/v/beaker.svg)

Beaker makes it easy to scaffold a new cosmwasm app, with all of the dependencies for osmosis hooked up, and a sample front-end at the ready.

## Getting Started

Install beaker with `cargo install beaker`

```bash
$ beaker new counter-dapp
$ cd counter-dapp
```

```bash
$ beaker wasm new counter
```

```bash
$ mkdir contracts/counter/instantiate-msgs
$ echo '{ "count": 0 }' > contracts/counter/instantiate-msgs/default.json
$ beaker wasm deploy counter --signer-account test1 --no-wasm-opt
```
```bash
$ npx create-next-app@latest --ts 
$ mkdir counter-frontend/beaker-state
$ ln -s "$(pwd)"/.beaker/state*.json counter-frontend/beaker-state
```
