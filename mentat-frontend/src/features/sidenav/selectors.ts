import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectSideNavOpen = createSelector(
  selectRootState,
  state => state.sidenav
);
