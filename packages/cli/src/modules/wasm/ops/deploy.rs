use crate::framework::Context;
use crate::modules::wasm::WasmConfig;
use crate::support::coin::Coins;
use crate::support::gas::Gas;
use anyhow::Result;

use cosmrs::crypto::secp256k1::SigningKey;

use super::build;
use super::instantiate;
use super::instantiate::InstantiateResponse;
use super::store_code;

#[allow(clippy::too_many_arguments)]
pub fn deploy<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    permit_only: &Option<String>,
    admin: Option<&String>,
    funds: Coins,
    network: &str,
    timeout_height: &u32,
    gas: &Gas,
    store_code_signing_key: SigningKey,
    instantiate_signing_key: SigningKey,
    no_rebuild: &bool,
    no_wasm_opt: &bool,
) -> Result<InstantiateResponse> {
    if !*no_rebuild {
        build(ctx, no_wasm_opt, &false)?;
    }
    store_code(
        ctx,
        contract_name,
        network,
        no_wasm_opt,
        permit_only,
        gas,
        timeout_height,
        store_code_signing_key,
    )?;
    instantiate(
        ctx,
        contract_name,
        label,
        raw,
        admin,
        // deploy command is not intended to use with the gov process
        true,
        true,
        funds,
        network,
        timeout_height,
        gas,
        instantiate_signing_key,
    )
}
