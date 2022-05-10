import { combineReducers } from 'redux';
import i18n from './features/i18n/reducer';
import sidenav from './features/sidenav/reducer';
import accounts from './features/accounts/reducer';

const reducer = combineReducers({
  i18n,
  accounts,
  sidenav,
});

export type RootState = ReturnType<typeof reducer>;

export default reducer;
