import type { HttpEndpoint } from '@cosmjs/tendermint-rpc';
import { Coin, Secp256k1HdWallet, SigningCosmWasmClient } from 'cosmwasm';
declare type Config = {
    global: {
        account_prefix: any;
        derivation_path: string;
        networks: {
            [x: string]: {
                rpc_endpoint: string | HttpEndpoint;
            };
        };
        gas_price: any;
        accounts: Record<string, {
            mnemonic: string;
        }>;
    };
};
export declare type Account = {
    signingClient: SigningCosmWasmClient;
    wallet: Secp256k1HdWallet;
    getBalance: (denom: string) => Promise<Coin>;
};
export declare const fromMnemonic: (conf: Config, network: string | number, mnemonic: string) => Promise<Account>;
export declare const getAccounts: (conf: Config, network: string) => Promise<{
    [k: string]: Account | undefined;
}>;
export {};
//# sourceMappingURL=account.d.ts.map