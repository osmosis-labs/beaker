/// build shared args for all commands
fn from(cli_args) {
    // expect build flags to be a comma separated list of flags
    // - no_rebuild: don't rebuild the contract
    // - no_wasm_opt: don't optimize the wasm
    let build_flags = cli_args.build_flags.split(",");

    #{
        signer_account: cli_args.signer,
        no_rebuild: build_flags.contains("no_rebuild"),
        no_wasm_opt: build_flags.contains("no_wasm_opt"),
    }
}

