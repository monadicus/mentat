import { createSlice, PayloadAction } from '@reduxjs/toolkit';

type AccountsState = {
  accounts: string[];
  aliases: Record<string, string>;
};

const accounts = createSlice({
  name: 'accounts',
  initialState: {
    accounts: ((localStorage.mentatAccounts as string) || '')
      .split(',')
      .filter(Boolean),
    aliases: (() => {
      try {
        return JSON.parse(localStorage.mentatAccountAliases || '{}');
      } catch (err) {
        console.warn('error parsing account aliases', err);
        return {};
      }
    })(),
  } as AccountsState,
  reducers: {
    addAccount(state, action: PayloadAction<string>) {
      if (!state.accounts.includes(action.payload)) {
        state.accounts.push(action.payload);
        localStorage.mentatAccounts = state.accounts.join(',');
      }
    },
    removeAccount(state, action: PayloadAction<string>) {
      const index = state.accounts.indexOf(action.payload);
      if (index > -1) {
        state.accounts.splice(index, 1);
        localStorage.mentatAccounts = state.accounts.join(',');
      }
    },
    setAccountAlias: {
      reducer(
        state,
        action: PayloadAction<{ address: string; alias: string }>
      ) {
        const { address, alias } = action.payload;
        if (!alias) {
          // eslint-disable-next-line
          const { [address]: _removed, ...rest } = state.aliases;
          state.aliases = rest;
        } else state.aliases[address] = alias;
      },
      prepare(address: string, alias: string) {
        return { payload: { address, alias } };
      },
    },
  },
});

export const { addAccount, removeAccount, setAccountAlias } = accounts.actions;
export default accounts.reducer;
