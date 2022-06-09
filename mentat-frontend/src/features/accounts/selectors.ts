import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';
import { selectNetworkIdentifier } from '../rosetta/selectors';
import type { AccountTracking } from './reducer';
import { netIdStr } from './util';

export const selectAccountsState = createSelector(
  selectRootState,
  selectNetworkIdentifier,
  (state, netId): AccountTracking =>
    netId ? state.accounts[netIdStr(netId)] : null
);

export const selectAccountsAliases = createSelector(
  selectAccountsState,
  (state): Record<string, string> => state?.aliases ?? {}
);

export const selectAccountAlias = (account: string) =>
  createSelector(selectAccountsState, state => state?.aliases[account] || '');

export const selectFollowedAccounts = createSelector(
  selectAccountsState,
  (state): string[] => state?.accounts ?? []
);

export const selectIsAccountFollowed = (account: string) =>
  createSelector(selectFollowedAccounts, accounts =>
    accounts.includes(account)
  );
