#!/usr/bin/env node

const { CosmWasmClient } = require('cosmwasm');
const { getContracts, getAccounts, extendWith } = require('../dist');

const CONSOLE_HISTORY_FILE = '.beaker_console_history';

const networkColor = (network) =>
  ({
    local: 'cyan',
    testnet: 'yellow',
    mainnet: 'red',
  }[network] || 'blue');

async function run() {
  const repl = require('pretty-repl');
  const path = require('path');
  const chalk = (await import('chalk')).default;

  const [_node, _beakerConsole, root, network, confStr] = process.argv;
  const conf = JSON.parse(confStr);

  const { getState } = require(path.join(root, '.beaker'));
  const state = getState()[network] || {};

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

  r.setupHistory(
    path.join(require('os').homedir(), CONSOLE_HISTORY_FILE),
    (e) => e,
  );

  r.on('reset', initializeContext);
}

run();
