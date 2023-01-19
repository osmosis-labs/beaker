use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use anyhow::Context;

use tendermint::abci::Event;
use tendermint_rpc::{endpoint::block_results, Client, HttpClient, Order};

pub fn read_wasm(
    root: PathBuf,
    contract_name: &str,
    no_wasm_opt: &bool,
) -> Result<Vec<u8>, anyhow::Error> {
    let wasm_path = if *no_wasm_opt {
        root.as_path()
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{contract_name}.wasm"))
    } else {
        root.as_path()
            .join("artifacts")
            .join(format!("{contract_name}.wasm"))
    };

    let wasm_path_str = &wasm_path.as_os_str().to_string_lossy();
    let f = File::open(&wasm_path).with_context(|| {
        format!(
            "`{wasm_path_str}` not found, please build and optimize the contract before store code`"
        )
    })?;
    let mut reader = BufReader::new(f);
    let mut wasm = Vec::new();
    reader.read_to_end(&mut wasm)?;
    Ok(wasm)
}

pub async fn get_code_id(rpc_endpoint: &str, proposal_id: &u64) -> Result<String, anyhow::Error> {
    let client = HttpClient::new(rpc_endpoint)?;
    let blocks_response = client
        .block_search(
            format!("active_proposal.proposal_id = {}", proposal_id)
                .parse()
                .unwrap(),
            1,
            1,
            Order::Descending,
        )
        .await?
        .blocks;

    let proposal_passed_block_height = blocks_response
        .first()
        .ok_or_else(|| {
            anyhow::anyhow!(format!(
                "block with `active_proposal.proposal_id = {proposal_id}` not found"
            ))
        })?
        .block
        .header()
        .height;

    let block_results: block_results::Response =
        client.block_results(proposal_passed_block_height).await?;

    extract_code_id_for_proposal(
        proposal_id,
        block_results.end_block_events.ok_or_else(|| {
            anyhow::anyhow!(format!(
                "code_id for proposal_id {proposal_id} not found: no end_block_events"
            ))
        })?,
    )
}

fn extract_code_id_for_proposal(
    prop_id: &u64,
    end_block_events: Vec<Event>,
) -> Result<String, anyhow::Error> {
    let mut code_id: Option<String> = None;
    for event in end_block_events {
        // keep `code_id` from store_code event if found
        if event.kind == "store_code" {
            code_id = event
                .attributes
                .iter()
                .find(|attr| attr.key == "code_id")
                .map(|attr| attr.value.to_string())
        }

        // active_proposal must be emitted after `store_code` due to how EndBlocker execute proposal
        // so if `active_proposal.proposal_id` match the expected proposal id, it should break the loop
        // if not break the loop, it might find another store_code event and use that event's code_id
        // which belong to later proposal that happens to execute on the same block
        if event.kind == "active_proposal"
            && event
                .attributes
                .iter()
                .any(|attr| attr.key == "proposal_id" && attr.value == prop_id.to_string())
        {
            break;
        }
    }
    code_id.ok_or_else(|| anyhow::anyhow!(format!("code_id for proposal_id {prop_id} not found")))
}

#[cfg(test)]
mod tests {

    use tendermint::abci::EventAttribute;

    use super::*;

    #[test]
    fn extract_code_id_from_single_proposal_exec_on_the_block() {
        let code_id = extract_code_id_for_proposal(
            &1,
            vec![
                Event {
                    kind: "store_code".to_string(),
                    attributes: vec![EventAttribute {
                        key: "code_id".parse().unwrap(),
                        value: "99".parse().unwrap(),
                        index: false,
                    }],
                },
                Event {
                    kind: "active_proposal".to_string(),
                    attributes: vec![EventAttribute {
                        key: "proposal_id".parse().unwrap(),
                        value: "1".parse().unwrap(),
                        index: false,
                    }],
                },
            ],
        )
        .unwrap();
        assert_eq!(code_id, "99");
    }

    #[test]
    fn extract_code_id_from_multiple_proposal_exec_on_the_block() {
        let end_block_events = vec![
            Event {
                kind: "store_code".to_string(),
                attributes: vec![EventAttribute {
                    key: "code_id".parse().unwrap(),
                    value: "111".parse().unwrap(),
                    index: false,
                }],
            },
            Event {
                kind: "active_proposal".to_string(),
                attributes: vec![EventAttribute {
                    key: "proposal_id".parse().unwrap(),
                    value: "1".parse().unwrap(),
                    index: false,
                }],
            },
            Event {
                kind: "store_code".to_string(),
                attributes: vec![EventAttribute {
                    key: "code_id".parse().unwrap(),
                    value: "999".parse().unwrap(),
                    index: false,
                }],
            },
            Event {
                kind: "active_proposal".to_string(),
                attributes: vec![EventAttribute {
                    key: "proposal_id".parse().unwrap(),
                    value: "9".parse().unwrap(),
                    index: false,
                }],
            },
        ];
        let code_id = extract_code_id_for_proposal(&1, end_block_events.clone()).unwrap();
        assert_eq!(code_id, "111");

        let code_id = extract_code_id_for_proposal(&9, end_block_events).unwrap();
        assert_eq!(code_id, "999");
    }
}
