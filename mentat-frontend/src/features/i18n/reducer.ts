import { createSlice } from '@reduxjs/toolkit';
import { Language, LANGUAGES } from './languages';

const i18n = createSlice({
  name: 'i18n',
  initialState: (localStorage.mentatLanguage as Language) || 'en_US',
  reducers: {
    setLanguage(state, action: { payload: Language }) {
      const lang = action.payload;
      if (!(lang in LANGUAGES)) return;

      state = localStorage.mentatLanguage = lang;
    },
  },
});

export const { setLanguage } = i18n.actions;
export default i18n.reducer;
