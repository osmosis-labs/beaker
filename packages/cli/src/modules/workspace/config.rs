use crate::support::template::Template;
use derive_get_docs::GetDocs;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, GetDocs)]
pub struct WorkspaceConfig {
    /// Template reference for generating new project
    pub template: Template,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            template: Template::new(
                "workspace-template".to_string(),
                "https://github.com/osmosis-labs/beaker.git".to_string(),
                "main".to_string(),
                PathBuf::from("."),
                Some("templates/project".to_string()),
            ),
        }
    }
}