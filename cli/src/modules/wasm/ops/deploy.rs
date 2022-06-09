use crate::framework::Context;
use crate::modules::wasm::WasmConfig;
use anyhow::Result;

use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Fee;

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
    network: &str,
    timeout_height: &u32,
    fee: &Fee,
    store_code_signing_key: SigningKey,
    instantiate_signing_key: SigningKey,
    no_rebuild: &bool,
) -> Result<InstantiateResponse> {
    if !*no_rebuild {
        build(ctx, &true, &false)?;
    }
    store_code(
        ctx,
        contract_name,
        network,
        fee,
        timeout_height,
        store_code_signing_key,
    )?;
    instantiate(
        ctx,
        contract_name,
        label,
        raw,
        network,
        timeout_height,
        fee,
        instantiate_signing_key,
    )
}
