# wasm

* **`wasm`** : WasmConfig  
  
   > 
  
  * **`contract_dir`** : String  
    
     > 
     > Directory for storing contracts  
     > 
    
    
  
  * **`optimizer_version`** : String  
    
     > 
     > Version of rust-optimizer  
     > 
    
    
  
  * **`template_repos`** : HashMap < String, String >  
    
     > 
     > Reference to contract template repository  
     > 
    
    

---

## Default Config

```toml
[wasm]
contract_dir = 'contracts'
optimizer_version = '0.14.0'

[wasm.template_repos]
sylvia = 'https://github.com/osmosis-labs/cw-sylvia-template'
classic = 'https://github.com/osmosis-labs/cw-minimal-template'
```