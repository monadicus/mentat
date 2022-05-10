import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectAccountsState = createSelector(
  selectRootState,
  state => state.accounts
);

export const selectAccountsAliases = createSelector(
  selectAccountsState,
  state => state.aliases
);

export const selectAccountAlias = (account: string) =>
  createSelector(selectAccountsState, state => state.aliases[account] || '');

export const selectFollowedAccounts = createSelector(
  selectAccountsState,
  state => state.accounts
);

export const selectIsAccountFollowed = (account: string) =>
  createSelector(selectFollowedAccounts, accounts =>
    accounts.includes(account)
  );
