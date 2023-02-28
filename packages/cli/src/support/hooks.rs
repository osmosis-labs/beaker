use super::{future::block, state::State, wasm::get_code_id};
use crate::{framework::config::Network, Context, WasmConfig};
use anyhow::Context as _;
use console::style;
use dialoguer::Confirm;

pub fn use_code_id<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    network: &str,
    network_info: &Network,
    state: State,
    contract_name: &str,
    no_proposal_sync: bool,
    yes: bool,
) -> Result<u64, anyhow::Error> {
    let code_id = block(async {
        let wasm_ref = state.get_ref(network, contract_name)?;

        let current_code_id = wasm_ref
            .code_id()
            .with_context(|| format!("Unable to retrieve code_id for {contract_name}"))?;

        if no_proposal_sync {
            return anyhow::Ok(current_code_id);
        }

        let proposal_id = wasm_ref
            .proposal()
            .store_code()
            .with_context(|| style(format!(
                "Proposal store code not found for contract `{contract_name}` on network `{network}`. \n\n\
                  Use {} option to ignore proposal syncing.\n",
                style("`--no-proposal-sync`").yellow().italic())))?;

        match get_code_id(network_info.rpc_endpoint(), &proposal_id).await {
            Ok(code_id_from_proposal) => {
                let code_id_from_proposal = code_id_from_proposal.parse().with_context(|| {
                    format!(
                        "unable to parse code_id from proposal: {}",
                        code_id_from_proposal
                    )
                })?;

                // code_id from proposal found but no new update
                if code_id_from_proposal == current_code_id {
                    anyhow::Ok(current_code_id)
                } else {
                    println!();
                    println!(
                        "  Found updated {} from proposal with {} {} :",
                        style("code_id").bold(),
                        style("proposal_id").bold(),
                        proposal_id
                    );
                    println!();
                    println!(
                        "{}",
                        style(format!(
                            "    {}: {} → {}",
                            style("~  code_id").yellow(),
                            style(current_code_id).red(),
                            style(code_id_from_proposal).green()
                        ))
                        .yellow()
                    );

                    println!();

                    if yes
                        || Confirm::new()
                            .with_prompt("> Do you want to update `code_id`?")
                            .interact()?
                    {
                        State::update_state_file(
                            network_info.network_variant(),
                            ctx.root()?,
                            &|s: &State| -> State {
                                s.update_code_id(network, contract_name, &code_id_from_proposal)
                            },
                        )?;
                        anyhow::Ok(code_id_from_proposal)
                    } else {
                        anyhow::Ok(current_code_id)
                    }
                }
            }
            Err(_) => anyhow::Ok(current_code_id),
        }
    })?;
    Ok(code_id)
}
