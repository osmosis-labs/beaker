import type { CodeDetails, Contract as ContractInfo, CosmWasmClient, StdFee } from 'cosmwasm';
import type { Account } from './account';
declare type Msg = Record<string, unknown>;
/**
 * Contract instance with baked-in client
 */
export declare class Contract {
    address: string;
    client: CosmWasmClient;
    constructor(address: string, client: CosmWasmClient);
    /**
     * Get contract info
     */
    getInfo(): Promise<ContractInfo>;
    /**
     * Get code details
     */
    getCode(): Promise<CodeDetails>;
    /**
     * Query the contract by passing query message
     * @returns query result
     */
    query(qmsg: Msg): Promise<unknown>;
    /**
     * Execute the contract.
     * example usage: `contract.execute(xmsg).by(signerAccount)`
     */
    execute(xmsg: Msg, senderAddress: string | null, fee?: number | 'auto' | StdFee): {
        by: (account: Account) => Promise<import("cosmwasm").ExecuteResult>;
    };
}
export declare const getContracts: (client: CosmWasmClient, state: Record<string, unknown>, sdk: {
    contracts: Record<string, Record<string, Function>>;
}) => Record<string, unknown>;
export {};
//# sourceMappingURL=contract.d.ts.map