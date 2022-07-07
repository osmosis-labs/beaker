use cosmrs::tendermint::abci::Event;
use tendermint_rpc::{endpoint::block_results, Client, HttpClient, Order};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = HttpClient::new("http://127.0.0.1:26657")?;

    let code_id_prop1 = get_code_id(client.clone(), 1).await?;
    let code_id_prop2 = get_code_id(client, 2).await?;

    dbg!(code_id_prop1, code_id_prop2);

    Ok(())
}

async fn get_code_id(client: HttpClient, prop_id: u64) -> Result<String, anyhow::Error> {
    let blocks_response = client
        .block_search(
            format!("active_proposal.proposal_id = {}", prop_id)
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
                "block with `active_proposal.proposal_id = {prop_id}` not found"
            ))
        })?
        .block
        .header()
        .height;
    let block_results: block_results::Response =
        client.block_results(proposal_passed_block_height).await?;

    // println!("\n\nproposal_id: {}", prop_id);
    // println!("{:#?}", proposal_passed_block_height);

    extract_code_id_for_proposal(prop_id, block_results.end_block_events.unwrap())
}

fn extract_code_id_for_proposal(
    prop_id: u64,
    end_block_events: Vec<Event>,
) -> Result<String, anyhow::Error> {
    let mut code_id: Option<String> = None;
    for event in end_block_events {
        // for a in event.clone().attributes {
        //     println!("{}.{} = {}", event.type_str, a.key, a.value);
        // }

        // keep `code_id` from store_code event if found
        if event.type_str == "store_code" {
            code_id = event
                .attributes
                .iter()
                .find(|attr| attr.key == "code_id".parse().unwrap())
                .map(|attr| attr.value.to_string())
        }

        // active_proposal must be emitted after `store_code` due to how EndBlocker execute proposal
        // so if `active_proposal.proposal_id` match the expected proposal id, it should break the loop
        // if not break the loop, it might find another store_code event and use that event's code_id
        // which belong to later proposal that happens to execute on the same block
        if event.type_str == "active_proposal"
            && event.attributes.iter().any(|attr| {
                attr.key == "proposal_id".parse().unwrap()
                    && attr.value == prop_id.to_string().parse().unwrap()
            })
        {
            break;
        }
    }
    code_id.ok_or_else(|| anyhow::anyhow!(format!("wh")))
}
