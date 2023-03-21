use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct OpsEntry {
    pub op: String,
    pub addr: Addr,
}

#[cw_serde]
pub enum OpExec {
    Eval { left: Uint128, right: Uint128 },
}

#[cw_serde]
pub struct OpEvalResult {
    pub result: Uint128,
}

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub ops: Vec<OpsEntry>,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    Eval {
        op: String,
        left: Uint128,
        right: Uint128,
    },
    RegisterOps {
        ops: Vec<OpsEntry>,
    },
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(OpContractResponse)]
    OpContract { op: String },
}

#[cw_serde]
pub struct OpContractResponse {
    pub addr: Addr,
}

/// MsgExecuteContractResponse returns execution result data.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct MsgExecuteContractResponse {
    /// Data contains base64-encoded bytes to returned from the contract
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
