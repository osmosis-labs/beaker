# Beaker

CosmWasm development tooling.


## Getting Started

```
$ beaker new counter-dapp
$ cd counter-dapp
```

```
$ beaker wasm new counter
```

```
$ mkdir contracts/counter/instantiate-msgs
$ echo '{ "count": 0 }' > contracts/counter/instantiate-msgs/default.json
$ beaker wasm deploy counter --signer-account test1 --no-wasm-opt
```
```
$ npx create-next-app@latest --ts 
$ mkdir counter-frontend/beaker-state
$ ln -s "$(pwd)"/.beaker/state*.json counter-frontend/beaker-state
```
