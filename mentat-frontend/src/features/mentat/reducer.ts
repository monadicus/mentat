import { MentatStatus } from '../../../backend/types';
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export type MentatState = {
  status: MentatStatus;
};

declare global {
  interface Window {
    MENTAT: MentatState['status'];
  }
}

const mentat = createSlice({
  name: 'mentat',
  initialState: {
    status: window.MENTAT ?? null,
  } as MentatState,
  reducers: {
    setMentatStatus(state, action: PayloadAction<MentatStatus>) {
      state.status = action.payload;
    },
  },
});

export const { setMentatStatus } = mentat.actions;
export default mentat.reducer;
