use crate::attrs_format;
use crate::modules::wasm::config::WasmConfig;
use crate::support::future::block;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::{framework::Context, support::cosmos::Client};
use anyhow::anyhow;
use anyhow::Context as _;
use anyhow::Result;
use cosmrs::AccountId;

use std::fs;

#[allow(clippy::too_many_arguments)]
pub fn query<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    network: &str,
) -> Result<QueryResponse> {
    let global_config = ctx.global_config()?;
    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone());
    let state = State::load_by_network(network_info, ctx.root()?)?;

    let contract = state
        .get_ref(network, contract_name)?
        .addresses()
        .get(label)
        .with_context(|| format!("Unable to retrieve contract for {contract_name}:{label}"))?
        .parse::<AccountId>()
        .map_err(|e| anyhow!(e))?;

    let query_msg = raw
        .map(|s| s.as_bytes().to_vec())
        .map(Ok)
        .unwrap_or_else(|| {
            let path = ctx
                .root()?
                .join("contracts")
                .join(contract_name)
                .join("query-msgs")
                .join(format!("{label}.json"));
            fs::read_to_string(&path)
                .with_context(|| format!("Unable to execute with `{}`", path.to_string_lossy()))
                .map(|s| s.as_bytes().to_vec())
        })?;

    block(async {
        let response = client.query_smart(contract.to_string(), query_msg).await?;
        let pretty_json_response = serde_json::to_string_pretty(
            &serde_json::from_slice::<serde_json::Value>(&response)
                .with_context(|| "Unable to deserialize response")?,
        )?;

        let query_response = QueryResponse {
            label: label.to_string(),
            contract_address: contract.to_string(),
            data: format!("\n{}", textwrap::indent(&pretty_json_response, "        ")),
        };

        query_response.log();

        Ok(query_response)
    })
}

#[allow(dead_code)]
pub struct QueryResponse {
    pub label: String,
    pub contract_address: String,
    pub data: String,
}

impl OpResponseDisplay for QueryResponse {
    fn headline() -> &'static str {
        "Succesffuly executed query!! 🎉 "
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | label, contract_address, data }
    }
}
