use crate::modules::wasm;

use rhai::plugin::*;
use rhai::Map;

#[export_module]
pub(crate) mod commands {
    use rhai::{
        serde::{from_dynamic, to_dynamic},
        Dynamic, EvalAltResult,
    };
    use serde::Serialize;

    use crate::WasmContext;

    const CONTEXT: WasmContext = WasmContext {};

    #[rhai_fn(return_raw)]
    pub fn deploy(mut cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        if cmd_args.contains_key("msg") {
            let msg = from_dynamic::<Map>(&cmd_args["msg"])?;
            let msg_json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
            cmd_args.insert("raw".into(), msg_json.into());
        }

        let mut cmd = Map::new();
        cmd.insert("Deploy".into(), cmd_args.into());

        wasm::entrypoint::deploy(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }

    #[derive(Serialize)]
    struct DeserializedQueryResponse {
        pub label: String,
        pub contract_address: String,
        pub data: Map,
    }

    #[rhai_fn(return_raw)]
    pub fn query(mut cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        if cmd_args.contains_key("msg") {
            let msg = from_dynamic::<Map>(&cmd_args["msg"])?;
            let msg_json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
            cmd_args.insert("raw".into(), msg_json.into());
        }

        let mut cmd = Map::new();
        cmd.insert("Query".into(), cmd_args.into());

        wasm::entrypoint::query(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(|res| {
                Ok(DeserializedQueryResponse {
                    label: res.label,
                    contract_address: res.contract_address,
                    data: serde_json::from_str(&res.data).map_err(|e| e.to_string())?,
                })
            })
            .and_then(to_dynamic)
    }

    #[rhai_fn(return_raw)]
    pub fn build(cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut cmd = Map::new();
        cmd.insert("Build".into(), cmd_args.into());

        wasm::entrypoint::build(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }

    #[rhai_fn(return_raw)]
    pub fn store_code(cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut cmd = Map::new();
        cmd.insert("StoreCode".into(), cmd_args.into());

        wasm::entrypoint::store_code(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }

    #[rhai_fn(return_raw)]
    pub fn update_admin(cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut cmd = Map::new();
        cmd.insert("UpdateAdmin".into(), cmd_args.into());

        wasm::entrypoint::update_admin(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }

    #[rhai_fn(return_raw)]
    pub fn clear_admin(cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        let mut cmd = Map::new();
        cmd.insert("ClearAdmin".into(), cmd_args.into());

        wasm::entrypoint::clear_admin(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }

    #[rhai_fn(return_raw)]
    pub fn instantiate(mut cmd_args: Map) -> Result<Dynamic, Box<EvalAltResult>> {
        if cmd_args.contains_key("msg") {
            let msg = from_dynamic::<Map>(&cmd_args["msg"])?;
            let msg_json = serde_json::to_string(&msg).map_err(|e| e.to_string())?;
            cmd_args.insert("raw".into(), msg_json.into());
        }

        let mut cmd = Map::new();
        cmd.insert("Instantiate".into(), cmd_args.into());

        wasm::entrypoint::instantiate(CONTEXT, &from_dynamic(&to_dynamic(cmd)?)?)
            .map_err(|e| e.to_string().into())
            .and_then(to_dynamic)
    }
}
