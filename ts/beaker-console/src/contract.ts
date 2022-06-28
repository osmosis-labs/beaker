import type {
  CodeDetails,
  Contract as ContractInfo,
  CosmWasmClient,
  ExecuteResult,
  StdFee,
} from 'cosmwasm';
import type { Account } from './account';
import { mapObject, mapValues } from './utils';

type Msg = Record<string, unknown>;

/**
 * Contract instance with baked-in client
 */
export class Contract {
  address: string;
  client: CosmWasmClient;

  constructor(address: string, client: CosmWasmClient) {
    this.address = address;
    this.client = client;
  }

  /**
   * Get contract info
   */
  async getInfo(): Promise<ContractInfo> {
    return await this.client.getContract(this.address);
  }

  /**
   * Get code details
   */
  async getCode(): Promise<CodeDetails> {
    return this.client.getCodeDetails((await this.getInfo()).codeId);
  }

  /**
   * Query the contract by passing query message
   * @returns query result
   */
  async query(qmsg: Msg): Promise<unknown> {
    return this.client.queryContractSmart(this.address, qmsg);
  }

  /**
   * Execute the contract.
   * example usage: `contract.execute(xmsg).by(signerAccount)`
   */
  execute(
    xmsg: Msg,
    senderAddress: string | null,
    fee: number | 'auto' | StdFee = 'auto',
  ) {
    return {
      by: async (account: Account): Promise<ExecuteResult> => {
        const _senderAddress =
          senderAddress || (await account.wallet.getAccounts())[0]?.address;

        if (!_senderAddress) {
          throw Error('Unable to get sender address');
        }

        return await account.signingClient.execute(
          _senderAddress,
          this.address,
          xmsg,
          fee,
        );
      },
    };
  }
}

export const getContracts = (
  client: CosmWasmClient,
  state: Record<string, unknown>,
) => {
  return mapValues(
    state,
    (contractInfo: { addresses: Record<string, Record<string, string>> }) => {
      const addresses = contractInfo.addresses;
      const prefixLabel = (label: string) => `$${label}`;
      let contracts = mapObject(
        addresses,
        prefixLabel,
        (addr: string) => new Contract(addr, client),
      );

      if (typeof contracts['$default'] === 'object' && contracts['$default']) {
        contracts = {
          ...contracts,
          ...contracts['$default'],
        };

        Object.setPrototypeOf(contracts, Contract.prototype);
      }
      return contracts;
    },
  );
};
