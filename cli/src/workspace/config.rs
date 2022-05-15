use std::path::PathBuf;

use protostar_helper_template::Template;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub(crate) template: Template,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            template: Template::new(
                "workspace-template".to_string(),
                "iboss-ptk/protostar-sdk".to_string(),
                "main".to_string(),
                PathBuf::from("."),
                Some("templates/project".to_string()),
            ),
        }
    }
}
