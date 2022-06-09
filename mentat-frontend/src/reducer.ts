import { combineReducers } from 'redux';
import i18n from './features/i18n/reducer';
import sidenav from './features/sidenav/reducer';
import accounts from './features/accounts/reducer';
import rosetta from './features/rosetta/reducer';
import errors from './features/errors/reducer';
import mentat from './features/mentat/reducer';

const reducer = combineReducers({
  i18n,
  accounts,
  sidenav,
  rosetta,
  errors,
  mentat,
});

export type RootState = ReturnType<typeof reducer>;

export default reducer;
