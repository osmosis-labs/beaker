#!/usr/bin/env node

async function run() {
  const repl = require('pretty-repl');
  const path = require('path');
  const chalk = (await import('chalk')).default;

  const [_node, _beakerConsole, root, network] = process.argv;

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

  const r = repl.start(options);

  Object.defineProperty(r.context, 'state', {
    configurable: false,
    enumerable: true,
    value: state[network],
  });
}

run();
