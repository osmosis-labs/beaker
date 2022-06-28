import { stringToPath } from '@cosmjs/crypto';
import type { HttpEndpoint } from '@cosmjs/tendermint-rpc';
import {
  Coin,
  GasPrice,
  Secp256k1HdWallet,
  SigningCosmWasmClient,
} from 'cosmwasm';

type Config = {
  global: {
    account_prefix: string;
    derivation_path: string;
    networks: { [x: string]: { rpc_endpoint: string | HttpEndpoint } };
    gas_price: string;
    accounts: Record<string, { mnemonic: string }>;
  };
};

export class Account {
  signingClient: SigningCosmWasmClient;
  wallet: Secp256k1HdWallet;

  constructor(wallet: Secp256k1HdWallet, signingClient: SigningCosmWasmClient) {
    this.wallet = wallet;
    this.signingClient = signingClient;
  }

  async getBalance(denom: string): Promise<Coin> {
    const accounts = await this.wallet.getAccounts();
    const address = accounts[0]?.address;

    if (!address) {
      throw Error(`No account not found from: ${accounts}`);
    }

    return await this.signingClient.getBalance(address, denom);
  }
}

export const fromMnemonic = async (
  conf: Config,
  network: string | number,
  mnemonic: string,
): Promise<Account> => {
  if (typeof conf.global.account_prefix !== 'string') {
    throw Error('`account_prefix` must be string');
  }

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
    { gasPrice: GasPrice.fromString(conf.global.gas_price) },
  );
  return new Account(wallet, signingClient);
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
