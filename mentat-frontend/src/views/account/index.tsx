import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { FaUserMinus, FaUserPlus } from 'react-icons/fa';
import { useDispatch, useSelector } from 'react-redux';
import { useParams } from 'react-router';
import {
  addAccount,
  removeAccount,
  setAccountAlias,
} from '../../features/accounts/reducer';
import {
  selectAccountAlias,
  selectIsAccountFollowed,
} from '../../features/accounts/selectors';
import { useApi, useNetId } from '../../features/rosetta/hooks';
import { Account } from '../../features/rosetta/models';
import { AppDispatch } from '../../store';

export const AccountView = () => {
  const network_identifier = useNetId();
  const { address } = useParams();
  const account_identifier: Account = useMemo(() => ({ address }), [address]);
  const isFollowed = useSelector(selectIsAccountFollowed(address));
  const dispatch: AppDispatch = useDispatch();

  const toggleFollowed = useCallback(() => {
    dispatch((isFollowed ? removeAccount : addAccount)(address));
  }, [isFollowed, dispatch, address]);

  const [, coinsResp] = useApi(
    '/account/coins',
    useMemo(
      () => ({
        network_identifier,
        account_identifier,
        currencies: [],
      }),
      [network_identifier, account_identifier]
    )
  );

  const alias = useSelector(selectAccountAlias(address));
  const [stateAlias, setStateAlias] = useState(alias);
  useEffect(() => setStateAlias(alias), [alias]);

  const setAlias = useCallback(() => {
    dispatch(setAccountAlias(address, stateAlias));
  }, [address, dispatch, stateAlias]);

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
      <pre>{JSON.stringify(coinsResp, null, 2)}</pre>
    </>
  );
};
