# wasm

* **`wasm`** : WasmConfig  
  
   > 
  
  * **`contract_dir`** : String  
    
     > 
     > Directory for storing contracts  
     > 
    
    
  
  * **`template_repo`** : String  
    
     > 
     > Reference to contract template repository  
     > 
    
    
  
  * **`optimizer_version`** : String  
    
     > 
     > Version of rust-optimizer  
     > 
    
    

---

## Default Config

```toml
[wasm]
contract_dir = 'contracts'
template_repo = 'https://github.com/osmosis-labs/cw-minimal-template'
optimizer_version = '0.12.8'
```