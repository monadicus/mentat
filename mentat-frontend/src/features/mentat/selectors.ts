import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectMentatState = createSelector(
  selectRootState,
  state => state.mentat
);

export const selectHasBackend = createSelector(selectMentatState, mentat =>
  Boolean(mentat)
);

export const selectMentatStatus = createSelector(
  selectMentatState,
  state => state.status
);

export const selectMentatServers = createSelector(
  selectMentatStatus,
  status => status?.servers ?? {}
);

export const selectMentatHasServer = (id: string) =>
  createSelector(
    selectMentatServers,
    servers => Boolean(servers) && id in servers
  );
