import { createSlice, nanoid, PayloadAction } from '@reduxjs/toolkit';
import { RosettaError } from '../rosetta/models';

export interface IError extends RosettaError {
  id: string;
  activity: string;
  show: boolean;
}

type ErrorState = {
  errors: IError[];
};

const errors = createSlice({
  name: 'errors',
  initialState: { errors: [] } as ErrorState,
  reducers: {
    addError: {
      reducer(
        state,
        action: PayloadAction<{ error: RosettaError; activity: string }>
      ) {
        const { error, activity } = action.payload;
        state.errors.push({
          ...error,
          id: nanoid(),
          show: true,
          activity: activity ?? '',
        });
      },
      prepare(error: RosettaError, activity?: string) {
        return { payload: { error, activity } };
      },
    },
    dismissError(state, action: PayloadAction<string>) {
      const index = state.errors.findIndex(e => e.id === action.payload);
      if (index > -1) state.errors[index].show = false;
    },
    removeError(state, action: PayloadAction<string>) {
      const index = state.errors.findIndex(e => e.id === action.payload);
      if (index > -1) state.errors.splice(index, 1);
    },
  },
});

export const { addError, dismissError, removeError } = errors.actions;
export default errors.reducer;
