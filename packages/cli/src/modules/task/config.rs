use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDataDocs)]
pub struct TaskConfig {
    /// path to the directory where tasks are stored
    pub tasks_path: String,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            tasks_path: "tasks".to_string(),
        }
    }
}
