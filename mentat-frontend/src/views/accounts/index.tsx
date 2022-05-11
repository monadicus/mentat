import React from 'react';
import { useSelector } from 'react-redux';
import { NavLink } from 'react-router-dom';
import {
  selectAccountAlias,
  selectFollowedAccounts,
} from '../../features/accounts/selectors';
import { I18n } from '../../features/i18n/components';
import { useLinkRoute } from '../../features/rosetta/hooks';

const AccountEntry: React.FC<{ address: string }> = ({ address }) => {
  const alias = useSelector(selectAccountAlias(address));
  return (
    <NavLink key={address} to={useLinkRoute('accounts', address)}>
      {alias || address}
    </NavLink>
  );
};

export const AccountsView = () => {
  const followedAccounts = useSelector(selectFollowedAccounts);

  return (
    <>
      <h3>
        <I18n name="views.accounts.followed_accounts_header" />
      </h3>
      {followedAccounts.length === 0 ? (
        <I18n name="views.accounts.no_followed_accounts_note" />
      ) : (
        <ul>
          {followedAccounts.map(addr => (
            <li key={addr}>
              <AccountEntry address={addr} />
            </li>
          ))}
        </ul>
      )}
    </>
  );
};
