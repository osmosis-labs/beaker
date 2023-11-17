# Beaker Configuration
The following list is the configuration references for beaker which can be used in `Beaker.toml`.

- [global](./global.md)
- [workspace](./workspace.md)
- [wasm](./wasm.md)
- [console](./console.md)
---

# Default Config
```toml
name = ''
gas_price = '0.025uosmo'
gas_adjustment = 1.3
account_prefix = 'osmo'
derivation_path = '''m/44'/118'/0'/0/0'''
[networks.local]
chain_id = 'localosmosis'
network_variant = 'Local'
grpc_endpoint = 'http://localhost:9090'
rpc_endpoint = 'http://localhost:26657'

[networks.testnet]
chain_id = 'osmo-test-5'
network_variant = 'Shared'
grpc_endpoint = 'https://grpc.osmotest5.osmosis.zone'
rpc_endpoint = 'https://rpc.osmotest5.osmosis.zone'

[networks.mainnet]
chain_id = 'osmosis-1'
network_variant = 'Shared'
grpc_endpoint = 'https://grpc.osmosis.zone:9090'
rpc_endpoint = 'https://rpc.osmosis.zone:443'
[accounts.validator]
mnemonic = 'bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort'

[accounts.test1]
mnemonic = 'notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius'

[accounts.test2]
mnemonic = 'quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty'

[accounts.test3]
mnemonic = 'symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb'

[accounts.test4]
mnemonic = 'bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty'

[accounts.test5]
mnemonic = 'second render cat sing soup reward cluster island bench diet lumber grocery repeat balcony perfect diesel stumble piano distance caught occur example ozone loyal'

[accounts.test6]
mnemonic = 'spatial forest elevator battle also spoon fun skirt flight initial nasty transfer glory palm drama gossip remove fan joke shove label dune debate quick'

[accounts.test7]
mnemonic = 'noble width taxi input there patrol clown public spell aunt wish punch moment will misery eight excess arena pen turtle minimum grain vague inmate'

[accounts.test8]
mnemonic = 'cream sport mango believe inhale text fish rely elegant below earth april wall rug ritual blossom cherry detail length blind digital proof identify ride'

[accounts.test9]
mnemonic = 'index light average senior silent limit usual local involve delay update rack cause inmate wall render magnet common feature laundry exact casual resource hundred'

[accounts.test10]
mnemonic = 'prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder'


# workspace

[workspace.template]
name = 'workspace-template'
repo = 'https://github.com/osmosis-labs/beaker.git'
branch = 'main'
subfolder = 'templates/project'
target_dir = '.'


# wasm

[wasm]
contract_dir = 'contracts'
optimizer_version = '0.14.0'

[wasm.template_repos]
classic = 'https://github.com/osmosis-labs/cw-minimal-template'
sylvia = 'https://github.com/osmosis-labs/cw-sylvia-template'


# console

[console]
account_namespace = true
contract_namespace = true

```