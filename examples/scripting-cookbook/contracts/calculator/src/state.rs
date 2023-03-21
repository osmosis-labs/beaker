use cosmwasm_std::Addr;
use cw_storage_plus::Map;

/// This is a map from operator name to the address of the contract that implements it.
pub const OPS_REGISTRY: Map<&str, Addr> = Map::new("ops_registry");
