// # Matching command line arguments
// matches the arguments passed to the script and returns a map of the arguments
let cli_args = match_args(["signer", "build_flags"]);

// expect build flags to be a comma separated list of flags
// - no_rebuild: don't rebuild the contract
// - no_wasm_opt: don't optimize the wasm
let build_flags = cli_args.build_flags.split(",");

let base_args = #{
    signer_account: cli_args.signer,
    no_rebuild: build_flags.contains("no_rebuild"),
    no_wasm_opt: build_flags.contains("no_wasm_opt"),
};

// with this setup, we can run commands like:
// $ beaker task run deploy -- --signer test1 --build_flags ""
// $ beaker task run deploy -- --signer test2 --build_flags "no_rebuild"
// $ beaker task run deploy -- --signer test3 --build_flags "no_rebuild,no_wasm_opt"


// deployment logic
let add_contract = wasm::deploy(merge(
    base_args,
    #{
        contract_name: "op-add",
        label: "add",
        msg: #{}
    }
));


let calculator_contract = wasm::deploy(merge(
    base_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            ops: [
                #{ op: "+", addr: add_contract.contract_address }
            ]
        }
    }
));

// query the registered add contract
let registered_add_contract = wasm::query(merge(
    base_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{ op_contract: #{ op: "+" } }
    }));

// ensure that the add contract was registered properly
@assert(registered_add_contract.data.addr == add_contract.contract_address);

// try evaluating an expression
let eval_response = wasm::execute(merge(
    base_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            "eval": #{ op: "+", left: "1", right: "2" }
        }
    }
));

@assert(eval_response.data.result == "3");