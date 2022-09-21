use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDataDocs)]
pub struct WasmConfig {
    /// Directory for storing contracts
    pub contract_dir: String,

    /// Reference to contract template repository
    pub template_repo: String,

    /// Version of rust-optimizer
    pub optimizer_version: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            contract_dir: "contracts".to_string(),
            template_repo: "https://github.com/osmosis-labs/cw-minimal-template".to_string(),
            optimizer_version: "0.12.8".to_string(),
        }
    }
}
