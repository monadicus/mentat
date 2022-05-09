import React, { useMemo } from 'react';
import { useParams } from 'react-router';
import { useNetId } from '../../App';
import { useApi } from '../../features/rosetta/hooks';
import { Account } from '../../features/rosetta/models';

export const AccountView = () => {
  const network_identifier = useNetId();
  const { address } = useParams();
  const account_identifier: Account = useMemo(() => ({ address }), [address]);

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

  return (
    <>
      <pre>{JSON.stringify(coinsResp, null, 2)}</pre>
    </>
  );
};
