use derive_get_docs::GetDocs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, GetDocs)]
pub struct WasmConfig {
    pub contract_dir: String,
    pub template_repo: String,
    pub optimizer_version: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            contract_dir: "contracts".to_string(),
            template_repo: "https://github.com/InterWasm/cw-template.git".to_string(),
            optimizer_version: "0.12.6".to_string(),
        }
    }
}
