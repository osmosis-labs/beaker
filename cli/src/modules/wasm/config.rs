use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct WasmConfig {
    pub contract_dir: String,
    pub template_repo: String,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            contract_dir: "contracts".to_string(),
            template_repo: "InterWasm/cw-template".to_string(),
        }
    }
}
