use crate::modules::wasm;

use rhai::plugin::*;
use rhai::{Dynamic, Map};

#[export_module]
pub(crate) mod commands {
    use rhai::{
        serde::{from_dynamic, to_dynamic},
        EvalAltResult,
    };

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
            .map(|res| to_dynamic(res).unwrap())
            .map_err(|e| e.to_string().into())
    }
}
