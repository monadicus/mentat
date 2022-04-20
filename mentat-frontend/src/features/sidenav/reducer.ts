import { createSlice } from '@reduxjs/toolkit';

const sidenav = createSlice({
  name: 'sidenav',
  initialState: false,
  reducers: {
    setOpen() {
      return true;
    },
    setClosed() {
      return false;
    },
    toggleOpen(state) {
      return !state;
    },
  },
});

export const { setOpen, setClosed, toggleOpen } = sidenav.actions;
export default sidenav.reducer;
