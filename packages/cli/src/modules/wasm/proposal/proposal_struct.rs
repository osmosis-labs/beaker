use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};

const PROPOSAL_GROUP: &str = "detailed-proposal";

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

    /// Unpin code on upload
    #[clap(long, group = PROPOSAL_GROUP, default_value="false")]
    #[serde(default)]
    pub unpin_code: bool,
}

impl StoreCodeProposal {
    pub fn description_with_metadata(&self) -> Result<String> {
        Ok(vec![self.description.trim()].join("\n"))
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
            unpin_code: true,
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
                unpin_code: true
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
                unpin_code = true
            "#,
        );

        let prop: StoreCodeProposal = toml::from_str(toml_str).unwrap();

        assert_eq!(prop, proposal_fixture());
    }
}
