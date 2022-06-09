import { createSelector } from '@reduxjs/toolkit';
import { selectRootState } from '../../selectors';

export const selectErrorState = createSelector(
  selectRootState,
  state => state.errors
);

export const selectErrors = createSelector(
  selectErrorState,
  state => state.errors
);

export const selectErrorIds = createSelector(selectErrors, errors =>
  errors.map(e => e.id)
);

export const selectErrorById = (id: string) =>
  createSelector(selectErrors, errors => errors.find(e => e.id === id));
