pub trait OpResponse {
    fn output_format(&self) -> String;
}

#[allow(dead_code)]
pub struct StoreCodeResponse {
    pub(crate) code_id: u64,
}

impl OpResponse for StoreCodeResponse {
    fn output_format(&self) -> String {
        [
            "",
            "  Code stored successfully!! 🎉 ",
            "    +",
            format!("    └── code_id: {}", self.code_id).as_str(),
            "",
        ]
        .join("\n")
    }
}

#[allow(dead_code)]
pub struct InstantiateResponse {
    pub(crate) code_id: u64,
    pub(crate) contract_address: String,
}

impl OpResponse for InstantiateResponse {
    fn output_format(&self) -> String {
        [
            "",
            "  Contract instantiated successfully!! 🎉 ",
            "    +",
            format!("    ├── code_id: {}", self.code_id).as_str(),
            format!("    └── contract_address: {}", self.contract_address).as_str(),
            "",
        ]
        .join("\n")
    }
}
