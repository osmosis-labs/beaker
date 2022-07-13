use crate::framework::Context;
use crate::modules::wasm::WasmConfig;
use crate::support::gas::Gas;
use anyhow::Result;

use cosmrs::crypto::secp256k1::SigningKey;

use super::migrate::MigrateResponse;
use super::store_code;
use super::{build, migrate};

#[allow(clippy::too_many_arguments)]
pub fn upgrade<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    network: &str,
    timeout_height: &u32,
    gas: &Gas,
    store_code_signing_key: SigningKey,
    instantiate_signing_key: SigningKey,
    no_rebuild: &bool,
    no_wasm_opt: &bool,
) -> Result<MigrateResponse> {
    if !*no_rebuild {
        build(ctx, no_wasm_opt, &false)?;
    }
    store_code(
        ctx,
        contract_name,
        network,
        no_wasm_opt,
        &None,
        gas,
        timeout_height,
        store_code_signing_key,
    )?;
    migrate(
        ctx,
        contract_name,
        label,
        raw,
        // upgrade command is not intended to use with the gov process
        true,
        true,
        network,
        timeout_height,
        gas,
        instantiate_signing_key,
    )
}
