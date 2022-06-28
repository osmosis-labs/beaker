import type { CodeDetails, Contract as ContractInfo, CosmWasmClient, ExecuteResult, StdFee } from 'cosmwasm';
import type { Account } from './account';
declare type Msg = Record<string, unknown>;
export declare class Contract {
    address: string;
    client: CosmWasmClient;
    constructor(address: string, client: CosmWasmClient);
    getInfo(): Promise<ContractInfo>;
    getCode(): Promise<CodeDetails>;
    query(qmsg: Msg): Promise<unknown>;
    execute(xmsg: Msg, senderAddress: string | null, fee?: number | 'auto' | StdFee): {
        by: (account: Account) => Promise<ExecuteResult>;
    };
}
export declare const getContracts: (client: CosmWasmClient, state: Record<string, unknown>) => Record<string, unknown>;
export {};
//# sourceMappingURL=contract.d.ts.map