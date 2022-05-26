use config::Map;
use getset::Getters;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Getters, Debug)]
#[get = "pub"]
pub struct GlobalConfig {
    name: String,
    account_prefix: String,
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
            derivation_path: "m/44'/118'/0'/0/0".to_string(),
            accounts: Map::from([
                ("validator".into(), Account::FromMnemonic { mnemonic: "satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn".into() }),
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
