use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct WasmConfig {
    pub contract_dir: String,
    pub template_repo: String,
    pub optimizer_version: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            contract_dir: "contracts".to_string(),
            template_repo: "InterWasm/cw-template".to_string(),
            optimizer_version: "0.12.6".to_string(),
        }
    }
}
