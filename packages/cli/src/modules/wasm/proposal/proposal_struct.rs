use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Code {
    repo: String,
    rust_flags: Option<String>,
    optimizer: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct StoreCodeProposal {
    title: String,
    description: String,
    code: Code,
}

#[cfg(test)]
mod tests {
    use crate::support::string::trim_indent;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn store_code_proposal_yaml() {
        let yaml = &trim_indent(
            r#"
                title: Proposal to allow DappName to be enabled in Osmosis
                description: |
                            A lengthy proposal description
                            goes here
                            we expect this to be many lines...
                code:
                    repo: https://github.com/osmosis-labs/beaker/templates/project
                    rust_flags: -C link-arg=-s
                    optimizer: workspace-optimizer:0.12.6
            "#,
        );

        let prop: StoreCodeProposal = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(
            prop,
            StoreCodeProposal {
                title: "Proposal to allow DappName to be enabled in Osmosis".to_string(),
                description: trim_indent(
                    r#"
                    A lengthy proposal description
                    goes here
                    we expect this to be many lines...
                    "#
                ),
                code: Code {
                    repo: "https://github.com/osmosis-labs/beaker/templates/project".to_string(),
                    rust_flags: Some("-C link-arg=-s".to_string()),
                    optimizer: Some("workspace-optimizer:0.12.6".to_string()),
                }
            }
        );
    }
}
