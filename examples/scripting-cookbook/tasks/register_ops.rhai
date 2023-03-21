import "lib/shared_args" as shared_args;

// # Matching command line arguments
// matches the arguments passed to the script and returns a map of the arguments
let cli_args = match_args(["signer", "build_flags", "op", "op_name"]);

let shared_args = shared_args::from(cli_args);

// Deploy op contract of choice
let op_contract = wasm::deploy(merge(
    shared_args,
    #{
        contract_name: "op-" + cli_args.op_name,
        label: cli_args.op_name,
        msg: #{}
    }
));

// register the op contract
wasm::execute(merge(
    shared_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{
            register_ops: #{
                ops: [
                    #{ op: cli_args.op, addr: op_contract.contract_address }
                ]
            }
        }
    }
));

// query the registered contract
let registered_add_contract = wasm::query(merge(
    shared_args,
    #{
        contract_name: "calculator",
        label: "calculator",
        msg: #{ op_contract: #{ op: cli_args.op } }
    }));

// ensure that the op contract was registered properly
@assert(registered_add_contract.data.addr == op_contract.contract_address);