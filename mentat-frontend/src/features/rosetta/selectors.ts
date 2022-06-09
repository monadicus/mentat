import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectRosettaState = createSelector(
  selectRootState,
  state => state.rosetta
);

export const selectNetworkState = createSelector(
  selectRosettaState,
  state => state.network
);

export const selectNetworkStatus = createSelector(
  selectNetworkState,
  state => state.status
);

export const selectCurrentBlock = createSelector(
  selectNetworkStatus,
  status => status?.current_block_identifier ?? null
);

export const selectNetworkOptions = createSelector(
  selectNetworkState,
  state => state.options
);

export const selectNetworkIdentifier = createSelector(
  selectNetworkState,
  state => state.identifier
);

export const selectNetworkVersions = createSelector(
  selectNetworkOptions,
  options => options?.version ?? null
);
