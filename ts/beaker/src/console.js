#!/usr/bin/env node

const {
  SigningCosmWasmClient,
  CosmWasmClient,
  Secp256k1HdWallet,
} = require('cosmwasm');
const { stringToPath } = require('@cosmjs/crypto');

const networkColor = (network) =>
  ({
    local: 'cyan',
    testnet: 'yellow',
    mainnet: 'red',
  }[network] || 'blue');

const fromMnemonic = async (conf, network, mnemonic) => {
  const options = {
    prefix: conf.global.account_prefix,
    hdPaths: [stringToPath(conf.global.derivation_path)],
  };
  const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, options);
  const signingClient = await SigningCosmWasmClient.connectWithSigner(
    conf.global.networks[network].rpc_endpoint,
    wallet,
    { gasPrice: conf.global.gas_price },
  );
  return signingClient;
};

const getAccounts = async (conf, network) => {
  const accountName = Object.keys(conf.global.accounts);
  const account = await Promise.all(
    Object.values(conf.global.accounts).map((a) =>
      fromMnemonic(conf, network, a.mnemonic),
    ),
  );

  return Object.fromEntries(accountName.map((name, i) => [name, account[i]]));
};

const id = (x) => x;
const mapObject = (o, f, g) =>
  Object.fromEntries(Object.entries(o).map(([k, v]) => [f(k), g(v)]));
const mapValues = (o, g) => mapObject(o, id, g);

const getContracts = (client, state) => {
  const getContract = (address) => ({
    address,
    async query(qmsg) {
      return await client.queryContractSmart(address, qmsg);
    },
    execute(xmsg, senderAddress = null, fee = 'auto') {
      return {
        async by(sigingClient) {
          const _senderAddress =
            senderAddress ||
            (await sigingClient.signer.getAccounts())[0].address;
          return await sigingClient.execute(_senderAddress, address, xmsg, fee);
        },
      };
    },
  });

  return mapValues(state, (contractInfo) => {
    const addresses = contractInfo.addresses;
    const prefixLabel = (label) => `$${label}`;
    let contracts = mapObject(addresses, prefixLabel, getContract);
    if (contracts.$default) {
      contracts = {
        ...contracts,
        ...contracts.$default,
      };
    }
    return contracts;
  });
};

const extendWith = (properties) => (context) => {
  Object.entries(properties).forEach(([k, v]) => {
    Object.defineProperty(context, k, {
      configurable: false,
      enumerable: true,
      value: v,
    });
  });
};

async function run() {
  const repl = require('pretty-repl');
  const path = require('path');
  const chalk = (await import('chalk')).default;

  const [_node, _beakerConsole, root, network, confStr] = process.argv;
  const conf = JSON.parse(confStr);

  const { getState } = require(path.join(root, '.beaker'));
  const state = getState(network)[network];

  const client = await CosmWasmClient.connect(
    conf.global.networks[network].rpc_endpoint,
  );

  const contract = getContracts(client, state);

  const options = {
    prompt: chalk.green(
      `beaker[${chalk.italic[networkColor(network)](network)}] ◇ `,
    ),
  };

  const account = await getAccounts(conf, network);

  const r = repl.start(options);

  const initializeContext = extendWith({
    state,
    conf,
    client,
    account,
    contract,
  });

  initializeContext(r.context);

  r.on('reset', initializeContext);
}

run();
