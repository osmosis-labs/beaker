#!/usr/bin/env node

async function run() {
  const repl = require('pretty-repl');
  const path = require('path');
  const chalk = (await import('chalk')).default;
  const { SigningCosmWasmClient, Secp256k1HdWallet } = require('cosmwasm');
  const { stringToPath } = require('@cosmjs/crypto');

  const [_node, _beakerConsole, root, network, confStr] = process.argv;
  const conf = JSON.parse(confStr);

  const { getState } = require(path.join(root, '.beaker'));
  const state = getState(network);

  const networkColor = (network) =>
    ({
      local: 'cyan',
      testnet: 'yellow',
      mainnet: 'red',
    }[network] || 'blue');

  const options = {
    prompt: chalk.green(
      `beaker[${chalk.italic[networkColor(network)](network)}] ◇ `,
    ),
  };

  const fromMnemonic = async (mnemonic) => {
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

  const accountName = Object.keys(conf.global.accounts);
  const _account = await Promise.all(
    Object.values(conf.global.accounts).map((a) => fromMnemonic(a.mnemonic)),
  );

  const account = Object.fromEntries(
    accountName.map((name, i) => [name, _account[i]]),
  );

  const r = repl.start(options);

  Object.defineProperty(r.context, 'state', {
    configurable: false,
    enumerable: true,
    value: state[network],
  });

  Object.defineProperty(r.context, 'conf', {
    configurable: false,
    enumerable: true,
    value: conf,
  });

  Object.defineProperty(r.context, 'account', {
    configurable: false,
    enumerable: true,
    value: account,
  });
}

run();
