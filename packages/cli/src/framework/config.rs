use config::Map;
use data_doc_derive::GetDataDocs;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Debug, Clone, GetDataDocs)]
#[get = "pub"]
pub struct GlobalConfig {
    /// Name of the project
    name: String,

    /// Gas price used for calculating fee
    /// `fee = ceil(gas_limit * gas_price)`
    /// `gas_limit` will be simulated if left unchecked
    gas_price: String,

    /// Adjusting `gas_limit` from simulated gas as a safety factor to make sure gas_limit is enought for the tx.
    /// When user doesn't specify `gas_limit`, `gas_limit = simulated_gas * gas_adjustment`,
    /// while `simulated_gas` is simulated gas consumption for the tx.
    gas_adjustment: f64,

    /// Prefix for the address
    account_prefix: String,

    /// BIP-32 derivation path used for creating account from mnemonic
    derivation_path: String,

    /// Map of the available network configuration to interact with via tesseract
    networks: Map<String, Network>,

    /// Predefined account used for interacting with the chain
    accounts: Map<String, Account>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, GetDataDocs)]
pub enum NetworkVariant {
    /// tesseract's state of the network will not be shared with collaborator via vcs
    Local,

    /// tesseract's state of the network will be shared with collaborator via vcs
    Shared,
}

#[derive(Serialize, Deserialize, Clone, Debug, Getters, GetDataDocs)]
#[get = "pub"]
pub struct Network {
    /// Chain id used for defining which network you are operating on
    chain_id: String,

    /// Network variant used to specify whether state file of the network should be tracked in vcs or not
    network_variant: NetworkVariant,

    /// Endpoint for grpc
    grpc_endpoint: String,

    /// Endpoint for rpc
    rpc_endpoint: String,

    /// Endpoint for rest
    rest_endpoint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, GetDataDocs)]
#[serde(untagged)]
pub enum Account {
    /// Used for specifying account from mnemonic, eg.
    /// `{ mnemonic = "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn" }`
    /// For testing only, for production or wallet with fair amount of coins on mainnet, don't specify these information in plain text
    FromMnemonic { mnemonic: String },

    /// Used for specifying account from private key, eg.
    /// `{ private_key = "SNI8xBejBnTpB6JAPxCfCC2S4ZeCPQLmpCPGrrjkEgQ=" }`
    /// For testing only, for production or wallet with fair amount of coins on mainnet, don't specify these information in plain text
    FromPrivateKey { private_key: String },
}

// TODO: make no assumption about osmosis later
impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            gas_price: "0.0002qube".to_string(),
            gas_adjustment: 1.0,
            account_prefix: "qube".to_string(),
            derivation_path: "m/44'/60'/0'/0/0".to_string(),
            networks: Map::from([
                (
                    "local".into(),
                    Network {
                        chain_id: "quadrate_5120-1".into(),
                        network_variant: NetworkVariant::Local,
                        grpc_endpoint: "http://localhost:9090".into(),
                        rpc_endpoint: "http://localhost:26657".into(),
                        rest_endpoint: "http://localhost:1317".into()
                    }
                ),
                (
                    "devnet".into(),
                    Network {
                        chain_id: "quadrate_5120-1".into(),
                        network_variant: NetworkVariant::Local,
                        grpc_endpoint: "http://45.83.123.247:9090".into(),
                        rpc_endpoint: "http://45.83.123.247:26657".into(),
                        rest_endpoint: "http://45.83.123.247:1317".into()
                    }
                ),
            ]),
            accounts: Map::from([
                ("validator".into(), Account::FromMnemonic { mnemonic: "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort".into() }),
                ("test1".into(), Account::FromMnemonic { mnemonic: "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius".into() }),
                ("test2".into(), Account::FromMnemonic { mnemonic: "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty".into() }),
                ("test3".into(), Account::FromMnemonic { mnemonic: "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb".into() }),
                ("test4".into(), Account::FromMnemonic { mnemonic: "bounce success option birth apple portion aunt rural episode solution hockey pencil lend session cause hedgehog slender journey system canvas decorate razor catch empty".into() }),
                ("test5".into(), Account::FromMnemonic { mnemonic: "second render cat sing soup reward cluster island bench diet lumber grocery repeat balcony perfect diesel stumble piano distance caught occur example ozone loyal".into() }),
                ("test6".into(), Account::FromMnemonic { mnemonic: "spatial forest elevator battle also spoon fun skirt flight initial nasty transfer glory palm drama gossip remove fan joke shove label dune debate quick".into() }),
                ("test7".into(), Account::FromMnemonic { mnemonic: "noble width taxi input there patrol clown public spell aunt wish punch moment will misery eight excess arena pen turtle minimum grain vague inmate".into() }),
                ("test8".into(), Account::FromMnemonic { mnemonic: "cream sport mango believe inhale text fish rely elegant below earth april wall rug ritual blossom cherry detail length blind digital proof identify ride".into() }),
                ("test9".into(), Account::FromMnemonic { mnemonic: "index light average senior silent limit usual local involve delay update rack cause inmate wall render magnet common feature laundry exact casual resource hundred".into() }),
                ("test10".into(), Account::FromMnemonic { mnemonic: "prefer forget visit mistake mixture feel eyebrow autumn shop pair address airport diesel street pass vague innocent poem method awful require hurry unhappy shoulder".into() })
            ]),
        }
    }
}
