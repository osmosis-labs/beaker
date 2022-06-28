# workspace

* **`workspace`** : WorkspaceConfig  
  
   > 
  
  * **`template`** : Template  
    
     > 
     > Template reference for generating new project  
     > 
    
    * **`name`** : String  
      
       > 
       > Name of the generated directory  
       > 
      
      
    
    * **`repo`** : String  
      
       > 
       > Git repo to be used as a template  
       > 
      
      
    
    * **`branch`** : String  
      
       > 
      
      
    
    * **`target_dir`** : PathBuf  
      
       > 
      
      
    
    * **`subfolder`** : Option < String >  
      
       > 
      
      

---

## Default Config

```toml
[workspace.template]
name = 'workspace-template'
repo = 'https://github.com/osmosis-labs/beaker.git'
branch = 'main'
target_dir = '.'
subfolder = 'templates/project'
```