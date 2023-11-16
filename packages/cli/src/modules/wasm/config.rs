use std::collections::HashMap;

use data_doc_derive::GetDataDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDataDocs)]
pub struct WasmConfig {
    /// Directory for storing contracts
    pub contract_dir: String,

    /// Reference to contract template repository
    pub template_repos: HashMap<String, String>,

    /// Version of rust-optimizer
    pub optimizer_version: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        let mut template_repo = HashMap::new();

        template_repo.insert(
            "classic".to_string(),
            "https://github.com/osmosis-labs/cw-minimal-template".to_string(),
        );

        template_repo.insert(
            "sylvia".to_string(),
            "https://github.com/osmosis-labs/cw-sylvia-template".to_string(),
        );

        Self {
            contract_dir: "contracts".to_string(),
            template_repos: template_repo,
            optimizer_version: "0.14.0".to_string(),
        }
    }
}
