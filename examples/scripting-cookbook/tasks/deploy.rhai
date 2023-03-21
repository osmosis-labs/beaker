import "lib/shared_args" as shared_args;

// # Matching command line arguments
// matches the arguments passed to the script and returns a map of the arguments
let cli_args = match_args(["signer", "build_flags"]);

// use the shared_args library to create a base set of arguments
let shared_args = shared_args::from(cli_args);

// with this setup, we can run commands like:
// $ beaker task run deploy -- --signer test1 --build_flags ""
// $ beaker task run deploy -- --signer test2 --build_flags "no_rebuild"
// $ beaker task run deploy -- --signer test3 --build_flags "no_rebuild,no_wasm_opt"

// Deploy the add contract
let add_contract = wasm::deploy(merge(
    shared_args,
    #{
        contract_name: "op-add",
        label: "add",
        msg: #{}
    }
));

// Deploy the calculator contract and use the add contract as the first operator
let calculator_contract = wasm::deploy(merge(
    shared_args,
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
    shared_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{ op_contract: #{ op: "+" } }
    }));

// ensure that the add contract was registered properly
@assert(registered_add_contract.data.addr == add_contract.contract_address);

// try evaluating an expression
let eval_response = wasm::execute(merge(
    shared_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            "eval": #{ op: "+", left: "1", right: "2" }
        }
    }
));

@assert(eval_response.data.result == "3");