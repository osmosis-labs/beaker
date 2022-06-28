# global

* **`name`** : String  
  
   > 
   > Name of the project  
   > 
  
  

* **`gas_price`** : String  
  
   > 
   > Gas price used for calculating fee  
   > `fee = ceil(gas_limit * gas_price)`  
   > `gas_limit` will be simulated if left unchecked  
   > 
  
  

* **`gas_adjustment`** : f64  
  
   > 
   > Adjusting `gas_limit` from simulated gas as a safety factor to make sure gas_limit is enought for the tx.  
   > When user doesn't specify `gas_limit`, `gas_limit = simulated_gas * gas_adjustment`,  
   > while `simulated_gas` is simulated gas consumption for the tx.  
   > 
  
  

* **`account_prefix`** : String  
  
   > 
   > Prefix for the address  
   > 
  
  

* **`derivation_path`** : String  
  
   > 
   > BIP-32 derivation path used for creating account from mnemonic  
   > 
  
  

* **`networks`** : Map < String, Network >  
  
   > 
   > Map of the available network configuration to interact with via beaker  
   > 
  
  * **`chain_id`** : String  
    
     > 
     > Chain id used for defining which network you are operating on  
     > 
    
    
  
  * **`network_variant`** : NetworkVariant  
    
     > 
     > Network variant used to specify whether state file of the network should be tracked in vcs or not  
     > 
    
    * **`Local`** : NetworkVariant::Local  
      
       > 
       > Beaker's state of the network will not be shared with collaborator via vcs  
       > 
      
      
    
    * **`Shared`** : NetworkVariant::Shared  
      
       > 
       > Beaker's state of the network will be shared with collaborator via vcs  
       > 
      
      
  
  * **`grpc_endpoint`** : String  
    
     > 
     > Endpoint for grpc  
     > 
    
    
  
  * **`rpc_endpoint`** : String  
    
     > 
     > Endpoint for rpc  
     > 
    
    

* **`accounts`** : Map < String, Account >  
  
   > 
   > Predefined account used for interacting with the chain  
   > 
  
  * **`FromMnemonic`** : Account::FromMnemonic  
    
     > 
     > Used for specifying account from mnemonic, eg.  
     > `{ mnemonic = "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn" }`  
     > For testing only, for production or wallet with fair amount of coins on mainnet, don't specify these information in plain text  
     > 
    
    * **`mnemonic`** : String  
      
       > 
      
      
  
  * **`FromPrivateKey`** : Account::FromPrivateKey  
    
     > 
     > Used for specifying account from private key, eg.  
     > `{ private_key = "SNI8xBejBnTpB6JAPxCfCC2S4ZeCPQLmpCPGrrjkEgQ=" }`  
     > For testing only, for production or wallet with fair amount of coins on mainnet, don't specify these information in plain text  
     > 
    
    * **`private_key`** : String  
      
       > 
      
      

---

## Default Config

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
chain_id = 'osmo-test-4'
network_variant = 'Shared'
grpc_endpoint = 'https://grpc-test.osmosis.zone:9090'
rpc_endpoint = 'https://rpc-test.osmosis.zone'

[networks.mainnet]
chain_id = 'osmosis-1'
network_variant = 'Shared'
grpc_endpoint = 'https://grpc.osmosis.zone:9090'
rpc_endpoint = 'https://rpc.osmosis.zone'
[accounts.validator]
mnemonic = 'satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn'

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
```