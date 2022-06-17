# Beaker

<p style="text-align: center;">
    <img src="https://github.com/osmosis-labs/beaker/workflows/CI/badge.svg?branch=main">
    <a href="https://github.com/osmosis-labs/beaker/blob/main/LICENSE-APACHE"><img src="https://img.shields.io/badge/license-APACHE-blue.svg"></a>
    <a href="https://github.com/osmosis-labs/beaker/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
    <a href="https://deps.rs/repo/github/osmosis-labs/beaker"><img src="https://deps.rs/repo/github/osmosis-labs/beaker/status.svg"></a>
</p>


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

## License

The crates in this repository are licensed under either of the following licenses, at your discretion.

    Apache License Version 2.0 (LICENSE-APACHE or apache.org license link)
    MIT license (LICENSE-MIT or opensource.org license link)

Unless you explicitly state otherwise, any contribution submitted for inclusion in this library by you shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.