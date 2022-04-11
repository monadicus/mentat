import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectLanguage = createSelector(
  selectRootState,
  state => state.i18n
);
