import type { HttpEndpoint } from '@cosmjs/tendermint-rpc';
import { SigningCosmWasmClient } from 'cosmwasm';
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
export declare const fromMnemonic: (conf: Config, network: string | number, mnemonic: string) => Promise<SigningCosmWasmClient>;
export declare const getAccounts: (conf: Config, network: string) => Promise<{
    [k: string]: SigningCosmWasmClient | undefined;
}>;
export {};
//# sourceMappingURL=client.d.ts.map