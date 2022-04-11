import { combineReducers } from 'redux';
import i18n from './features/i18n/reducer';

const reducer = combineReducers({
  i18n,
});

export type RootState = ReturnType<typeof reducer>;

export default reducer;
