use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Eval { left: Uint128, right: Uint128 },
}

#[cw_serde]
pub struct EvalResult {
    pub result: Uint128,
}

impl EvalResult {
    pub fn new(result: Uint128) -> Self {
        Self { result }
    }
}
