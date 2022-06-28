import type { HttpEndpoint } from '@cosmjs/tendermint-rpc';
import { Coin, Secp256k1HdWallet, SigningCosmWasmClient } from 'cosmwasm';
declare type Config = {
    global: {
        account_prefix: string;
        derivation_path: string;
        networks: {
            [x: string]: {
                rpc_endpoint: string | HttpEndpoint;
            };
        };
        gas_price: string;
        accounts: Record<string, {
            mnemonic: string;
        }>;
    };
};
export declare class Account {
    signingClient: SigningCosmWasmClient;
    wallet: Secp256k1HdWallet;
    constructor(wallet: Secp256k1HdWallet, signingClient: SigningCosmWasmClient);
    getBalance(denom: string): Promise<Coin>;
}
export declare const fromMnemonic: (conf: Config, network: string | number, mnemonic: string) => Promise<Account>;
export declare const getAccounts: (conf: Config, network: string) => Promise<{
    [k: string]: Account | undefined;
}>;
export {};
//# sourceMappingURL=account.d.ts.map