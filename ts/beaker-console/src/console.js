#!/usr/bin/env node

/* eslint-disable */

const { CosmWasmClient } = require('cosmwasm');
const { execSync, spawnSync } = require('child_process');
const { getContracts, getAccounts, extendWith } = require('../dist');
const prompts = require('prompts');

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
  const fs = require('fs');
  const chalk = (await import('chalk')).default;

  const [_node, _beakerConsole, root, network, confStr] = process.argv;
  const conf = JSON.parse(confStr);

  const state = () => {
    const local = fs.readFileSync(
      path.join(root, '.beaker', 'state.local.json'),
    );
    const shared = fs.readFileSync(path.join(root, '.beaker', 'state.json'));
    const _state = { ...JSON.parse(local), ...JSON.parse(shared) };
    return _state[network] || {};
  };

  const client = await CosmWasmClient.connect(
    conf.global.networks[network].rpc_endpoint,
  );

  let sdk;
  try {
    sdk = require(path.join(root, 'ts', 'sdk'));
  } catch (e) {
    const { gen } = await prompts([
      {
        type: 'confirm',
        name: 'gen',
        message:
          "Project's Typescript SDK seems to be missing, would you like to generate?",
        initial: true,
      },
    ]);

    if (gen) {
      const contracts = Object.keys(state());
      contracts.forEach((c) => {
        spawnSync('beaker', ['wasm', 'ts-gen', c], {
          shell: true,
          stdio: 'inherit',
        });
      });
      sdk = require(path.join(root, 'ts', 'sdk'));
    }
  }

  const options = {
    prompt: chalk.green(
      `beaker[${chalk.italic[networkColor(network)](network)}] ◇ `,
    ),
  };

  const r = repl.start(options);

  const initializeContext = async (ctx) => {
    const _state = state();
    const account = await getAccounts(conf, network);
    const contract = getContracts(client, _state, sdk);
    return extendWith({
      state: _state,
      conf,
      client,
      sdk,
      ...(conf.console.account_namespace ? { account } : account),
      ...(conf.console.contract_namespace ? { contract } : contract),
    })(ctx);
  };

  await initializeContext(r.context);

  const beakerCommand = async (replServer, prefixCmd, args) => {
    replServer.clearBufferedCommand();
    const [contract, options] = args.split(' -- ');

    const cmd = `${prefixCmd} ${contract || ''} ${options || ''}`;

    console.log('command: ', cmd);

    try {
      execSync(cmd, { stdio: 'inherit' });
    } catch (e) {
      console.error(e);
    }

    await initializeContext(replServer.context);
    replServer.displayPrompt();
  };

  r.defineCommand('build', {
    help: 'Build contract without leaving console (use only for development)',
    async action(args) {
      await beakerCommand(this, 'beaker wasm build --no-wasm-opt', args);
    },
  });

  r.defineCommand('storeCode', {
    help: 'Store code without leaving console (use only for development)',
    async action(args) {
      await beakerCommand(this, 'beaker wasm store-code --no-wasm-opt', args);
    },
  });

  r.defineCommand('instantiate', {
    help: 'Instantiate contract without leaving console (use only for development)',
    async action(args) {
      await beakerCommand(this, 'beaker wasm instantiate', args);
    },
  });

  r.defineCommand('deploy', {
    help: 'Deploy contract without leaving console',
    async action(args) {
      await beakerCommand(this, 'beaker wasm deploy --no-wasm-opt', args);
    },
  });

  r.setupHistory(
    path.join(require('os').homedir(), CONSOLE_HISTORY_FILE),
    (e) => e,
  );

  r.on('reset', initializeContext);
}

run();
