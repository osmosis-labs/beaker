use crate::support::template::Template;
use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, GetDataDocs)]
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
                Some("templates/project".to_string()),
                PathBuf::from("."),
            ),
        }
    }
}
