import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { FaUserMinus, FaUserPlus } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { useParams } from 'react-router';
import { CoinAmount } from '../../components/Amount';
import {
  addAccount,
  removeAccount,
  setAccountAlias,
} from '../../features/accounts/reducer';
import {
  selectAccountAlias,
  selectIsAccountFollowed,
} from '../../features/accounts/selectors';
import { I18n } from '../../features/i18n/components';
import { useApi, useNetId } from '../../features/rosetta/hooks';
import { Account, BalanceResponse } from '../../features/rosetta/models';
import { selectCurrentBlock } from '../../features/rosetta/selectors';
import { AppDispatch } from '../../store';

export const AccountView = () => {
  const network_identifier = useNetId();
  const { address } = useParams();
  const account_identifier: Account = useMemo(() => ({ address }), [address]);
  const isFollowed = useSelector(selectIsAccountFollowed(address));
  const dispatch: AppDispatch = useDispatch();

  const toggleFollowed = useCallback(() => {
    dispatch(
      (isFollowed ? removeAccount : addAccount)(network_identifier, address)
    );
  }, [isFollowed, dispatch, address, network_identifier]);

  const block_identifier = useSelector(selectCurrentBlock);

  const [, coinsResp] = useApi<BalanceResponse>(
    '/account/balance',
    useMemo(
      () => ({
        network_identifier,
        account_identifier,
        block_identifier,
        currencies: [],
      }),
      [network_identifier, account_identifier, block_identifier]
    )
  );

  const alias = useSelector(selectAccountAlias(address));
  const [stateAlias, setStateAlias] = useState(alias);
  useEffect(() => setStateAlias(alias), [alias]);

  const setAlias = useCallback(() => {
    dispatch(setAccountAlias(network_identifier, address, stateAlias));
  }, [network_identifier, address, dispatch, stateAlias]);

  const onAliasChange: React.ChangeEventHandler<HTMLInputElement> = useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      setStateAlias(event.target.value);
    },
    []
  );

  return (
    <>
      <div>
        <button onClick={toggleFollowed}>
          {isFollowed ? 'Unfollow' : 'Follow'}
          {isFollowed ? <FaUserMinus /> : <FaUserPlus />}
        </button>
      </div>
      <div>
        Alias:{' '}
        <input
          onBlur={setAlias}
          value={stateAlias}
          onChange={onAliasChange}
          placeholder="not set"
        />
      </div>
      <div>
        <h3>
          <I18n name="views.account.balances_header" />
        </h3>
        <ul>
          {coinsResp?.balances?.map(b => (
            <li key={b.currency.symbol}>
              <CoinAmount amount={b} />
            </li>
          ))}
        </ul>
      </div>
      <pre>{JSON.stringify(coinsResp, null, 2)}</pre>
    </>
  );
};
