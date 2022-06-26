# workspace

* `workspace`  
  
  * `template`  
    Template reference for generating new project  
    
    * `name`  
      Name of the generated directory  
      
      
    * `repo`  
      Git repo to be used as a template  
      
      
    * `branch`  
      
      
    * `target_dir`  
      
      
    * `subfolder`  
      
      
    
  

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