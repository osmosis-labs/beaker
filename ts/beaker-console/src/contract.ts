import { pascal } from 'case';
import type {
  CodeDetails,
  Contract as ContractInfo,
  CosmWasmClient,
  StdFee,
} from 'cosmwasm';
import type { Account } from './account';
import { mapKV, mapObject } from './utils';

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
      by: (account: Account) =>
        executor(account, this.address)(xmsg, senderAddress, fee),
    };
  }
}

export const getContracts = (
  client: CosmWasmClient,
  state: Record<string, unknown>,
  /* eslint-disable */
  // @ts-ignore
  sdk: { contracts: Record<string, Record<string, Function>> },
) => {
  return mapKV(
    state,
    (
      contractName: string,
      contractInfo: { addresses: Record<string, Record<string, string>> },
    ) => {
      const addresses = contractInfo.addresses;
      const prefixLabel = (label: string) => `$${label}`;

      const pascalContractName = pascal(contractName);
      const contractSdk = errorIfNotFound(
        sdk.contracts[pascalContractName],
        `"${pascalContractName}" not found in sdk`,
      );

      const contractQueryClient = warnIfNotFound(
        contractSdk[`${pascalContractName}QueryClient`],
        `"${pascalContractName}QueryClient" not found in "${contractName}" contract's sdk. This may caused by empty QueryMsg variant.`,
      );

      const contractClient = warnIfNotFound(
        contractSdk[`${pascalContractName}Client`],
        `"${pascalContractName}Client" not found in "${contractName}" contract's sdk. This may caused by empty ExecuteMsg variant.`,
        true,
      );

      let contracts = mapObject(
        addresses,
        prefixLabel,
        // (addr: string) => ,
        (addr: string) => ({
          ...new Contract(addr, client),
          /* eslint-disable */
          // @ts-ignore
          ...evalOrElse(
            contractQueryClient,
            (cqc: any) => new cqc(client, addr),
            {},
          ),
          signer: (account: Account) => {
            return {
              /* eslint-disable */
              // @ts-ignore
              ...evalOrElse(
                contractClient,
                (cq: any) =>
                  new cq(account.signingClient, account.address, addr),
                {},
              ),
              execute: executor(account, addr),
            };
          },
        }),
      );

      if (typeof contracts['$default'] === 'object' && contracts['$default']) {
        contracts = {
          ...contracts,
          ...contracts['$default'],
        };

        Object.setPrototypeOf(contracts, Contract.prototype);
      }
      return [contractName, contracts];
    },
  );
};

const executor =
  (account: Account, contractAddress: string) =>
  async (
    msg: Msg,
    senderAddress: string | null,
    fee: number | 'auto' | StdFee = 'auto',
  ) => {
    const _senderAddress =
      senderAddress || (await account.wallet.getAccounts())[0]?.address;

    if (!_senderAddress) {
      throw Error('Unable to get sender address');
    }

    return await account.signingClient.execute(
      _senderAddress,
      contractAddress,
      msg,
      fee,
    );
  };

const errorIfNotFound = <T>(object: T | undefined, msg: string) => {
  if (object === undefined) {
    throw Error(msg);
  } else {
    return object;
  }
};

const warnIfNotFound = <T>(
  object: T | undefined,
  msg: string,
  last: boolean = false,
) => {
  if (object === undefined) {
    process.stdout.clearLine(0, () => {
      process.stdout.cursorTo(0, () => {
        console.log('\u001B[33m[WARN] ' + msg);

        if (last) {
          process.stdout.emit('resize'); // a hack to get prompt back after all warnings
        }
      });
    });
    return object;
  } else {
    return object;
  }
};

const evalOrElse = <T, U>(
  object: T | undefined,
  f: (object: T) => U,
  orElse: U,
) => {
  if (object !== undefined) {
    return f(object);
  } else {
    return orElse;
  }
};
