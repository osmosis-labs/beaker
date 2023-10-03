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
       > Git repo url to be used as template  
       > 
      
      
    
    * **`branch`** : String  
      
       > 
       > Brance of the repo to be used as template  
       > 
      
      
    
    * **`subfolder`** : Option < String >  
      
       > 
       > Subfolder of the repo to be used as template, use root of the repo if not specified  
       > 
      
      
    
    * **`target_dir`** : PathBuf  
      
       > 
       > Target directory for generating code from template to take place  
       > 
      
      

---

## Default Config

```toml
[workspace.template]
name = 'workspace-template'
repo = 'https://github.com/osmosis-labs/beaker.git'
branch = 'main'
subfolder = 'templates/project'
target_dir = '.'
```