use std::{fs, path::PathBuf};

use anyhow::{Context as _, Result};
use config::Map;
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::framework::config::{Network, NetworkVariant};

pub const STATE_DIR: &str = ".beaker";
pub const STATE_FILE_LOCAL: &str = "state.local.json";
pub const STATE_FILE_SHARED: &str = "state.json";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default, Getters)]
#[get = "pub"]
pub struct WasmRef {
    code_id: Option<u64>,
    proposal_store_code_id: Option<u64>,
    addresses: Map<String, String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct State(Map<String, Map<String, WasmRef>>);

impl State {
    pub fn get_ref(&self, network: &str, contract_name: &str) -> Result<WasmRef> {
        let State(m) = self;
        let network_m = m
            .get(network)
            .with_context(|| format!("No state found for network `{network}`"))?;
        network_m.get(contract_name).cloned().with_context(|| {
            format!("No state found for contract name `{contract_name}` on network `{network}`")
        })
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let path_str = path.to_string_lossy();
        let content = fs::read_to_string(path)
            .with_context(|| format!("Unable to read from `{path_str}`"))?;
        serde_json::from_str(&content)
            .with_context(|| format!("Unable to serialize state file `{path_str}`"))
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let path_str = path.to_string_lossy();
        let content =
            serde_json::to_string_pretty(self).with_context(|| "Unable to serialize to json")?;
        fs::write(&path, content).with_context(|| format!("Unabel to write to `{path_str}`"))
    }

    pub fn load_by_network(network: Network, root: PathBuf) -> Result<Self> {
        Self::load(&root.join(STATE_DIR).join(match network.network_variant() {
            NetworkVariant::Local => STATE_FILE_LOCAL,
            NetworkVariant::Shared => STATE_FILE_SHARED,
        }))
    }

    pub fn update_state_file(
        network_variant: &NetworkVariant,
        root: PathBuf,
        f: &(dyn Fn(&State) -> State),
    ) -> Result<State> {
        let state_dir = &root.join(STATE_DIR);
        let state_file = &state_dir.join(match network_variant {
            NetworkVariant::Local => STATE_FILE_LOCAL,
            NetworkVariant::Shared => STATE_FILE_SHARED,
        });
        fs::create_dir_all(state_dir)?;
        let s = State::load(state_file).unwrap_or_default();

        let s = f(&s);
        s.save(state_file)?;

        Ok(s)
    }

    pub fn update_address(
        &self,
        chain_id: &str,
        contract_name: &str,
        label: &str,
        address: &str,
    ) -> Self {
        let State(m) = self;
        let mut m = m.clone();
        m.entry(chain_id.to_string()).and_modify(|contracts| {
            contracts
                .entry(contract_name.to_string())
                .and_modify(|wasm_ref| {
                    wasm_ref
                        .addresses
                        .entry(label.to_string())
                        .and_modify(|a| *a = address.to_string())
                        .or_insert_with(|| address.to_string());
                });
        });

        State(m)
    }

    pub fn update_code_id(&self, network: &str, contract_name: &str, code_id: &u64) -> Self {
        let State(m) = self;
        let mut m = m.clone();

        m.entry(network.to_string())
            .and_modify(|contracts| {
                contracts
                    .entry(contract_name.to_string())
                    .and_modify(|wasm_ref| {
                        *wasm_ref = WasmRef {
                            code_id: Some(*code_id),
                            ..wasm_ref.clone()
                        };
                    })
                    .or_insert_with(|| WasmRef {
                        code_id: Some(*code_id),
                        addresses: Map::new(),
                        proposal_store_code_id: None,
                    });
            })
            .or_insert_with(|| {
                Map::from([(
                    contract_name.to_string(),
                    WasmRef {
                        code_id: Some(*code_id),
                        addresses: Map::new(),
                        proposal_store_code_id: None,
                    },
                )])
            });

        State(m)
    }

    pub fn update_proposal_store_code_id(
        &self,
        network: &str,
        contract_name: &str,
        proposal_store_code_id: &u64,
    ) -> Self {
        let State(m) = self;
        let mut m = m.clone();

        m.entry(network.to_string())
            .and_modify(|contracts| {
                contracts
                    .entry(contract_name.to_string())
                    .and_modify(|wasm_ref| {
                        *wasm_ref = WasmRef {
                            proposal_store_code_id: Some(*proposal_store_code_id),
                            ..wasm_ref.clone()
                        };
                    })
                    .or_insert_with(|| WasmRef {
                        code_id: None,
                        proposal_store_code_id: Some(*proposal_store_code_id),
                        addresses: Map::new(),
                    });
            })
            .or_insert_with(|| {
                Map::from([(
                    contract_name.to_string(),
                    WasmRef {
                        proposal_store_code_id: Some(*proposal_store_code_id),
                        code_id: None,
                        addresses: Map::new(),
                    },
                )])
            });

        State(m)
    }
}

#[cfg(test)]
mod tests {

    use assert_fs::TempDir;

    use super::*;
    use std::{fs, path::Path};

    fn state_file(temp_dir: &TempDir) -> PathBuf {
        temp_dir
            .to_path_buf()
            .as_path()
            .join(Path::new("state.json"))
    }
    fn setup(temp_dir: &TempDir, content: &str) -> PathBuf {
        let path = state_file(temp_dir);
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn load_single_contract() {
        let temp = assert_fs::TempDir::new().unwrap();
        let path = setup(
            &temp,
            r#"
        {
            "localosmosis": {
                "counter": {
                    "code_id": 1,
                    "addresses": {}
                }
            }
        }
        "#,
        );

        assert_eq!(
            State::load(&path).unwrap(),
            State(Map::from([(
                "localosmosis".to_string(),
                Map::from([(
                    "counter".to_string(),
                    WasmRef {
                        code_id: Some(1),
                        proposal_store_code_id: None,
                        addresses: Map::new(),
                    },
                )]),
            )]))
        );
    }

    #[test]
    fn load_multiple_contracts() {
        let temp = assert_fs::TempDir::new().unwrap();
        let path = setup(
            &temp,
            r#"
        {
            "localosmosis": {
                "counter": {
                    "code_id": 1,
                    "addresses": {}
                },
                "multiplier": {
                    "code_id": 5,
                    "addresses": {}
                }
            }
        }
        "#,
        );

        assert_eq!(
            State::load(&path).unwrap(),
            State(Map::from([(
                "localosmosis".to_string(),
                Map::from([
                    (
                        "counter".to_string(),
                        WasmRef {
                            code_id: Some(1),
                            proposal_store_code_id: None,
                            addresses: Map::new(),
                        },
                    ),
                    (
                        "multiplier".to_string(),
                        WasmRef {
                            code_id: Some(5),
                            proposal_store_code_id: None,
                            addresses: Map::new(),
                        },
                    )
                ]),
            )]))
        );
    }

    #[test]
    fn save_and_load() {
        let temp = assert_fs::TempDir::new().unwrap();
        let path = state_file(&temp);

        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([
                (
                    "counter".to_string(),
                    WasmRef {
                        code_id: Some(1),
                        proposal_store_code_id: None,
                        addresses: Map::new(),
                    },
                ),
                (
                    "multiplier".to_string(),
                    WasmRef {
                        code_id: Some(5),
                        proposal_store_code_id: None,
                        addresses: Map::new(),
                    },
                ),
            ]),
        )]));

        state.save(&path).unwrap();
        let loaded_state = State::load(&path).unwrap();

        assert_eq!(state, loaded_state);
    }

    #[test]
    fn update_code_id_test() {
        let empty_state = State(Map::new());
        let updated_state = empty_state.update_code_id("localosmosis", "counter", &1);

        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(1),
                    proposal_store_code_id: None,
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        let updated_state = empty_state.update_code_id("localosmosis", "counter", &99);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(99),
                    proposal_store_code_id: None,
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        // update existing
        let updated_state = updated_state.update_code_id("localosmosis", "counter", &112);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(112),
                    proposal_store_code_id: None,
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        // update with new contract but same chain id
        let updated_state = updated_state.update_code_id("localosmosis", "multiplier", &666);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([
                (
                    "counter".to_string(),
                    WasmRef {
                        code_id: Some(112),
                        proposal_store_code_id: None,
                        addresses: Map::new(),
                    },
                ),
                (
                    "multiplier".to_string(),
                    WasmRef {
                        code_id: Some(666),
                        proposal_store_code_id: None,
                        addresses: Map::new(),
                    },
                ),
            ]),
        )]));

        assert_eq!(updated_state, state);
    }

    #[test]
    fn update_proposal_id_test() {
        let empty_state = State(Map::new());
        let updated_state =
            empty_state.update_proposal_store_code_id("localosmosis", "counter", &1);

        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: None,
                    proposal_store_code_id: Some(1),
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        let updated_state =
            empty_state.update_proposal_store_code_id("localosmosis", "counter", &99);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: None,
                    proposal_store_code_id: Some(99),
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        // update existing
        let updated_state =
            updated_state.update_proposal_store_code_id("localosmosis", "counter", &112);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: None,
                    proposal_store_code_id: Some(112),
                    addresses: Map::new(),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        // update with new contract but same chain id
        let updated_state =
            updated_state.update_proposal_store_code_id("localosmosis", "multiplier", &666);
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([
                (
                    "counter".to_string(),
                    WasmRef {
                        code_id: None,
                        proposal_store_code_id: Some(112),
                        addresses: Map::new(),
                    },
                ),
                (
                    "multiplier".to_string(),
                    WasmRef {
                        code_id: None,
                        proposal_store_code_id: Some(666),
                        addresses: Map::new(),
                    },
                ),
            ]),
        )]));

        assert_eq!(updated_state, state);
    }

    #[test]
    fn update_address_test() {
        // No code id, no update, since contract_name `counter` doesn't exist
        let empty_state = State(Map::new());
        let updated_state = empty_state.update_address(
            "localosmosis",
            "counter",
            "default",
            "osmo1252netaxc2c0n4g4zm428d75gkl0dplrksd32g35yfylldu66nzqjtjn85",
        );

        assert_eq!(updated_state, empty_state);

        // Need to add code_id first, else you can't update address
        let updated_state = empty_state.update_code_id("localosmosis", "counter", &1);
        let updated_state = updated_state.update_address(
            "localosmosis",
            "counter",
            "default",
            "osmo1252netaxc2c0n4g4zm428d75gkl0dplrksd32g35yfylldu66nzqjtjn85",
        );

        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(1),
                    proposal_store_code_id: None,
                    addresses: Map::from([(
                        "default".to_string(),
                        "osmo1252netaxc2c0n4g4zm428d75gkl0dplrksd32g35yfylldu66nzqjtjn85"
                            .to_string(),
                    )]),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        let updated_state = updated_state.update_address(
            "localosmosis",
            "counter",
            "hello",
            "osmo1warl5pyfkxzd8v8megu8nt25gu8u07km0ncekk5969m3w8eg6wcqr9m700",
        );
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(1),
                    proposal_store_code_id: None,
                    addresses: Map::from([
                        (
                            "default".to_string(),
                            "osmo1252netaxc2c0n4g4zm428d75gkl0dplrksd32g35yfylldu66nzqjtjn85"
                                .to_string(),
                        ),
                        (
                            "hello".to_string(),
                            "osmo1warl5pyfkxzd8v8megu8nt25gu8u07km0ncekk5969m3w8eg6wcqr9m700"
                                .to_string(),
                        ),
                    ]),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);

        // update existing
        let updated_state = updated_state.update_address(
            "localosmosis",
            "counter",
            "default",
            "osmo1warl5pyfkxzd8v8megu8nt25gu8u07km0ncekk5969m3w8eg6wcqr9m700",
        );
        let state = State(Map::from([(
            "localosmosis".to_string(),
            Map::from([(
                "counter".to_string(),
                WasmRef {
                    code_id: Some(1),
                    proposal_store_code_id: None,
                    addresses: Map::from([
                        (
                            "default".to_string(),
                            "osmo1warl5pyfkxzd8v8megu8nt25gu8u07km0ncekk5969m3w8eg6wcqr9m700"
                                .to_string(),
                        ),
                        (
                            "hello".to_string(),
                            "osmo1warl5pyfkxzd8v8megu8nt25gu8u07km0ncekk5969m3w8eg6wcqr9m700"
                                .to_string(),
                        ),
                    ]),
                },
            )]),
        )]));

        assert_eq!(updated_state, state);
    }
}
