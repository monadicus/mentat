import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { NetworkIdentifier } from '../rosetta/models';
import { netIdStr } from './util';

export type AccountTracking = {
  accounts: string[];
  aliases: Record<string, string>;
};

/** map of netId to account tracking state */
export type AccountsState = Record<string, AccountTracking>;

const accounts = createSlice({
  name: 'accounts',
  initialState: ((): AccountsState => {
    try {
      const state: AccountsState = JSON.parse(
        localStorage.mentatAccountState || '{}'
      );

      // weak type checking for parsing state from local storage
      if (typeof state !== 'object' || Array.isArray(state)) return {};
      if (!Object.values(state).every(v => 'accounts' in v && 'aliases' in v))
        return {};
      return state;
    } catch (err) {
      console.warn('error parsing account aliases', err);
      return {};
    }
  })(),
  reducers: {
    addAccount: {
      reducer(
        state,
        action: PayloadAction<{
          address: string;
          network_identifier: NetworkIdentifier;
        }>
      ) {
        const { address, network_identifier } = action.payload;
        const key = netIdStr(network_identifier);
        state[key] ??= { accounts: [], aliases: {} };

        if (!state[key].accounts.includes(address)) {
          state[key].accounts.push(address);
          localStorage.mentatAccountState = JSON.stringify(state);
        }
      },
      prepare(network_identifier: NetworkIdentifier, address: string) {
        return { payload: { address, network_identifier } };
      },
    },

    removeAccount: {
      reducer(
        state,
        action: PayloadAction<{
          address: string;
          network_identifier: NetworkIdentifier;
        }>
      ) {
        const { address, network_identifier } = action.payload;
        const key = netIdStr(network_identifier);
        state[key] ??= { accounts: [], aliases: {} };

        const index = state[key].accounts.indexOf(address) ?? -1;
        if (index > -1) {
          state[key].accounts.splice(index, 1);
          localStorage.mentatAccountState = JSON.stringify(state);
        }
      },
      prepare(network_identifier: NetworkIdentifier, address: string) {
        return { payload: { address, network_identifier } };
      },
    },

    setAccountAlias: {
      reducer(
        state,
        action: PayloadAction<{
          address: string;
          alias: string;
          network_identifier: NetworkIdentifier;
        }>
      ) {
        const { address, alias, network_identifier } = action.payload;
        const key = netIdStr(network_identifier);
        state[key] ??= { accounts: [], aliases: {} };

        if (!alias) {
          // eslint-disable-next-line
          const { [address]: _removed, ...rest } = state[key].aliases ?? {};
          state[key].aliases = rest;
        } else state[key].aliases[address] = alias;
        localStorage.mentatAccountState = JSON.stringify(state);
      },
      prepare(
        network_identifier: NetworkIdentifier,
        address: string,
        alias: string
      ) {
        return { payload: { network_identifier, address, alias } };
      },
    },
  },
});

export const { addAccount, removeAccount, setAccountAlias } = accounts.actions;
export default accounts.reducer;
