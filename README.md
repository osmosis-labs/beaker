# protostar-sdk

SDK for developing custom CosmWasm development tooling.

---

CosmWasm is multi-chain smart contract platform building for Cosmos ecosystem. While being powerful as it is, a lot of things needs to be wired manually. To provide seamless CosmWasm development experience, Protostar is designed to be a swiss-army knife for building custom devtool for CosmWasm on each chain.

Protostar is designed to be modular and customizable. Since each chain has their own custom module and the nature of smart contract interaction of each chain could be different, thus allowing custom feature onto Protostar is essential.

The default set of modules in Protostar will mirroring defualt modules in Cosmos.

All the defualt modules and custom modules needs to be registered to the engine in order construct the interface.


With that composability, we can use those logic to expose to any kind client. Being a backend for CLI, IDE, Web interface, you name it.


Modules can have all kind of functionality, for example:


### meta
- configuration

### contract
- contract template
- contract compilation
- contract optimization
- contract store code & init (deployment; with flags)
- contract admin and migration
- contract workspace


### security
- multisig

### testing & automation
- gas estimation
- scripting (+ metascripting, in rust / js)
- cosmwasm-vm testing
- integration testing
- custom browser with custom wallet
- state seeding

### client
- js/ts/wasm client generation
- frontend dev support


Each module is designed to be imported and customized rather than forked.

Protostar also provides, create for using in cosmwasm contract itself to help testing / client generation / everything needed to make the entire development experience seamless.

Hot reloading? Dev Server?
