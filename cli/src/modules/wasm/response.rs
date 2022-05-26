use crate::{attrs_format, support::ops_response::OpResponseDisplay};

#[allow(dead_code)]
pub struct StoreCodeResponse {
    pub(crate) code_id: u64,
}

impl OpResponseDisplay for StoreCodeResponse {
    fn headline() -> &'static str {
        "Code stored successfully!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | code_id }
    }
}

#[allow(dead_code)]
pub struct InstantiateResponse {
    pub(crate) label: String,
    pub(crate) contract_address: String,
    pub(crate) code_id: u64,
    pub(crate) creator: String,
    pub(crate) admin: String,
}

impl OpResponseDisplay for InstantiateResponse {
    fn headline() -> &'static str {
        "Contract instantiated successfully!! 🎉 "
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | label, contract_address, code_id, creator, admin }
    }
}
