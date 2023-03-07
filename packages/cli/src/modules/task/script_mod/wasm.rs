use crate::{
    modules::wasm::{self},
    support::signer::SignerArgs,
    Context,
};
use anyhow::Context as _;
use rhai::plugin::*;
use rhai::{serde::to_dynamic, Dynamic, Map};

#[export_module]
pub(crate) mod commands {
    use rhai::EvalAltResult;

    use crate::WasmContext;

    const CONTEXT: WasmContext = WasmContext {};

    #[rhai_fn(return_raw)]
    pub fn deploy(
        contract_name: &str,
        label: &str,
        instantiate_msg: Map,
        signer: &str,
    ) -> Result<Dynamic, Box<EvalAltResult>> {
        let conf = CONTEXT.global_config().unwrap();
        let key = || {
            SignerArgs {
                signer_account: Some(signer.to_string()),
                signer_keyring: None,
                signer_mnemonic: None,
                signer_private_key: None,
            }
            .private_key(&conf)
            .with_context(|| format!("Failed to get private key for {}", signer))
            .unwrap()
        };

        wasm::ops::deploy::<WasmContext>(
            &CONTEXT,
            contract_name,
            label,
            Some(&serde_json::to_string(&instantiate_msg).unwrap()),
            &None,
            None,
            None.try_into().unwrap(),
            "local",
            &0,
            &crate::Gas::Auto {
                gas_price: "1000uosmo".parse().unwrap(),
                gas_adjustment: 1.5,
            },
            key(),
            key(),
            &true,
            &true,
            &None,
        )
        .map(|res| to_dynamic(res).unwrap())
        .map_err(|e| e.to_string().into())
    }
}
