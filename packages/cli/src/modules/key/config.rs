use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDataDocs)]
pub struct KeyConfig {
    /// Name of the service used as namespace for system's keyring
    pub service: String,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            service: "beaker".to_string(),
        }
    }
}
