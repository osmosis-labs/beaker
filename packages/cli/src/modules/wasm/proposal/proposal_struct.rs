use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

const PROPOSAL_GROUP: &str = "detailed-proposal";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Parser)]
pub struct Code {
    /// Public repository of the code
    #[clap(long, group = PROPOSAL_GROUP, default_value="")]
    #[serde(default)]
    pub repo: String,

    /// RUST_FLAGS that passed while compiling to wasm
    /// If building with Beaker, it's usually "-C link-arg=-s"
    #[clap(long, group = PROPOSAL_GROUP)]
    pub rust_flags: Option<String>,

    /// Type and version of the [optimizer](https://github.com/CosmWasm/rust-optimizer), either:
    /// rust-optimizer:<version> or
    /// workspace-optimizer:<version>.
    /// Beaker use workspace-optimizer, the version, if not manually configured, can be found in `wasm` config doc.
    #[clap(long, group = PROPOSAL_GROUP)]
    pub optimizer: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Parser)]
#[clap(group = clap::ArgGroup::new(PROPOSAL_GROUP)
    .multiple(true)
    .conflicts_with("proposal"))]
pub struct StoreCodeProposal {
    /// Proposal title
    #[clap(long, group = PROPOSAL_GROUP, default_value="")]
    #[serde(default)]
    pub title: String,

    /// Proposal decsription
    #[clap(long, group = PROPOSAL_GROUP, default_value="")]
    #[serde(default)]
    pub description: String,

    /// Proposal deposit to activate voting
    #[clap(long, group = PROPOSAL_GROUP)]
    pub deposit: Option<String>,

    /// Metadata of the wasm to store
    #[clap(flatten)]
    #[serde(flatten)]
    pub code: Code,
}

impl StoreCodeProposal {
    pub fn description_with_metadata(&self) -> Result<String> {
        Ok(vec![
            self.description.trim(),
            "",
            "---",
            "",
            "[metadata]",
            toml::to_string::<Code>(&self.code)?.as_str().trim(),
        ]
        .join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use crate::support::string::trim_indent;

    use super::*;
    use pretty_assertions::assert_eq;

    fn proposal_fixture() -> StoreCodeProposal {
        StoreCodeProposal {
            title: "Proposal to allow DappName to be enabled in Osmosis".to_string(),
            description: trim_indent(
                r#"
            A lengthy proposal description
            goes here
            we expect this to be many lines...
            "#,
            ),
            deposit: Some("1000uosmo".to_string()),
            code: Code {
                repo: "https://github.com/osmosis-labs/beaker/templates/project".to_string(),
                rust_flags: Some("-C link-arg=-s".to_string()),
                optimizer: Some("workspace-optimizer:0.12.6".to_string()),
            },
        }
    }

    #[test]
    fn store_code_proposal_yaml() {
        let yaml = &trim_indent(
            r#"
                title: Proposal to allow DappName to be enabled in Osmosis
                description: |
                            A lengthy proposal description
                            goes here
                            we expect this to be many lines...
                deposit: 1000uosmo
                code:
                    repo: https://github.com/osmosis-labs/beaker/templates/project
                    rust_flags: -C link-arg=-s
                    optimizer: workspace-optimizer:0.12.6
            "#,
        );

        let prop: StoreCodeProposal = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(prop, proposal_fixture());
    }

    #[test]
    fn store_code_proposal_toml() {
        let toml_str = &trim_indent(
            r#"
                title = "Proposal to allow DappName to be enabled in Osmosis"

                description = '''
                A lengthy proposal description
                goes here
                we expect this to be many lines...
                '''

                deposit = "1000uosmo"

                [code]
                repo = "https://github.com/osmosis-labs/beaker/templates/project"
                rust_flags = "-C link-arg=-s"
                optimizer = "workspace-optimizer:0.12.6"
            "#,
        );

        let prop: StoreCodeProposal = toml::from_str(toml_str).unwrap();

        assert_eq!(prop, proposal_fixture());
    }
}
