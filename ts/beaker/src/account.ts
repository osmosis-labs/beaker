import { stringToPath } from '@cosmjs/crypto';
import type { HttpEndpoint } from '@cosmjs/tendermint-rpc';
import { Coin, Secp256k1HdWallet, SigningCosmWasmClient } from 'cosmwasm';

type Config = {
  global: {
    account_prefix: any;
    derivation_path: string;
    networks: { [x: string]: { rpc_endpoint: string | HttpEndpoint } };
    gas_price: any;
    accounts: Record<string, { mnemonic: string }>;
  };
};

export type Account = {
  signingClient: SigningCosmWasmClient;
  wallet: Secp256k1HdWallet;
  getBalance: (denom: string) => Promise<Coin>;
};

export const fromMnemonic = async (
  conf: Config,
  network: string | number,
  mnemonic: string,
): Promise<Account> => {
  const options = {
    prefix: conf.global.account_prefix,
    hdPaths: [stringToPath(conf.global.derivation_path)],
  };
  const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, options);

  const networkInfo = conf.global.networks[network];
  if (!networkInfo) {
    throw Error(`network info for ${network} not found in the config`);
  }

  const signingClient = await SigningCosmWasmClient.connectWithSigner(
    networkInfo.rpc_endpoint,
    wallet,
    { gasPrice: conf.global.gas_price },
  );
  return {
    signingClient,
    wallet,
    async getBalance(denom) {
      const accounts = await wallet.getAccounts();
      const address = accounts[0]?.address;

      if (!address) {
        throw Error(`No account not found from: ${accounts}`);
      }

      return await signingClient.getBalance(address, denom);
    },
  };
};

export const getAccounts = async (conf: Config, network: string) => {
  const accountName = Object.keys(conf.global.accounts);
  const account = await Promise.all(
    Object.values(conf.global.accounts).map((a) =>
      fromMnemonic(conf, network, a.mnemonic),
    ),
  );

  return Object.fromEntries(accountName.map((name, i) => [name, account[i]]));
};
