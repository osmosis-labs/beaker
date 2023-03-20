#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg,
};
use cw2::set_contract_version;
use prost::Message;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MsgExecuteContractResponse, OpExec, QueryMsg};
use crate::state::OPS_REGISTRY;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:calculator";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SUBMSG_ID__EVAL: u64 = 1;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    msg.ops
        .iter()
        .try_for_each(|entry| OPS_REGISTRY.save(deps.storage, &entry.op, &entry.addr))?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Eval { op, left, right } => {
            let addr = OPS_REGISTRY.load(deps.storage, &op)?;
            let eval_msg = wasm_execute(addr, &OpExec::Eval { left, right }, vec![])?;

            Ok(Response::new().add_submessage(SubMsg::reply_on_success(eval_msg, SUBMSG_ID__EVAL)))
        }
        ExecuteMsg::RegisterOps { ops } => {
            ops.iter()
                .try_for_each(|entry| OPS_REGISTRY.save(deps.storage, &entry.op, &entry.addr))?;

            Ok(Response::new().add_attribute("method", "register_ops"))
        }
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OpContract { op } => {
            let addr = OPS_REGISTRY.load(deps.storage, &op)?;
            let res = crate::msg::OpContractResponse { addr };
            to_binary(&res)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if (msg.id) != SUBMSG_ID__EVAL {
        return Err(ContractError::InvalidReplyId { id: msg.id });
    }

    let res = msg
        .result
        .into_result()
        .map_err(StdError::generic_err)?
        .data
        .ok_or_else(|| StdError::generic_err("No data in reply"))?;

    let res = MsgExecuteContractResponse::decode(&mut res.as_slice())
        .map_err(|_e| StdError::generic_err("MsgExecuteContractResponse decode failed"))?;
    Ok(Response::new().set_data(res.data))
}
