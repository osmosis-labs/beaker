use config::Map;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Debug)]
#[get = "pub"]
pub struct GlobalConfig {
    name: String,
    account_prefix: String,
    denom: String,
    derivation_path: String,
    accounts: Map<String, Account>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Account {
    FromMnemonic { mnemonic: String },
    FromPrivateKey { private_key: String },
}

// TODO: make no assumption about osmosis later
impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            account_prefix: "osmo".to_string(),
            denom: "uosmo".to_string(),
            derivation_path: "m/44'/118'/0'/0/0".to_string(),
            accounts: Map::from([
                ("validator".into(), Account::FromMnemonic { mnemonic: "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn".into() }),
                ("test1".into(), Account::FromMnemonic { mnemonic: "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius".into() })
            ]),
        }
    }
}
