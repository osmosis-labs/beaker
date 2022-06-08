use crate::{attrs_format, support::ops_response::OpResponseDisplay};

#[allow(dead_code)]
pub struct ProposeStoreCodeResponse {
    pub(crate) proposal_id: u64,
    pub(crate) deposit_amount: String,
}

impl OpResponseDisplay for ProposeStoreCodeResponse {
    fn headline() -> &'static str {
        "Store code proposal has been submitted!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | proposal_id, deposit_amount }
    }
}
