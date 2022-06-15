import type { CosmWasmClient, StdFee } from 'cosmwasm';
import type { Account } from './account';
import { mapObject, mapValues } from './utils';

type Msg = Record<string, unknown>;
export const getContracts = (client: CosmWasmClient, state: Object) => {
  const getContract = (address: string) => ({
    address,
    async query(qmsg: Msg) {
      return await client.queryContractSmart(address, qmsg);
    },
    execute(
      xmsg: Msg,
      senderAddress: string | null,
      fee: number | 'auto' | StdFee = 'auto',
    ) {
      return {
        async by(account: Account) {
          const _senderAddress =
            senderAddress || (await account.wallet.getAccounts())[0]?.address;

          if (!_senderAddress) {
            throw Error('Unable to get sender address');
          }

          return await account.signingClient.execute(
            _senderAddress,
            address,
            xmsg,
            fee,
          );
        },
      };
    },
  });

  return mapValues(
    state,
    (contractInfo: { addresses: Record<string, Record<string, string>> }) => {
      const addresses = contractInfo.addresses;
      const prefixLabel = (label: string) => `$${label}`;
      let contracts = mapObject(addresses, prefixLabel, getContract);

      // @ts-ignore
      if (contracts.$default) {
        contracts = {
          ...contracts,
          // @ts-ignore
          ...contracts.$default,
        };
      }
      return contracts;
    },
  );
};
