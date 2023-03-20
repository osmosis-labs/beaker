// # Matching command line arguments
// matches the arguments passed to the script and returns a map of the arguments
let cli_args = match_args(["signer", "build_flags", "op"]);

// expect build flags to be a comma separated list of flags
// - no_rebuild: don't rebuild the contract
// - no_wasm_opt: don't optimize the wasm
let build_flags = cli_args.build_flags.split(",");

let base_args = #{
    signer_account: cli_args.signer,
    no_rebuild: build_flags.contains("no_rebuild"),
    no_wasm_opt: build_flags.contains("no_wasm_opt"),
};

let op_contract = wasm::deploy(merge(
    base_args,
    #{
        contract_name: "op-" + cli_args.op,
        label: cli_args.op,
        msg: #{}
    }
));

wasm::execute(merge(
    base_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            register_ops: #{
                ops: [
                    #{ op: "*", addr: op_contract.contract_address }
                ]
            }
        }
    }
));

let eval_response = wasm::execute(merge(
    base_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            eval: #{
                op: "*",
                a: "2",
                b: "3"
            }
        }
    }
));

@assert(eval_response.data.result == "6");