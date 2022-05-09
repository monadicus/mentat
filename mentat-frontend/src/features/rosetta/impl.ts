import { useMemo } from 'react';
import { ApiState, useApi } from './hooks';
import { NetworkIdentifier, NetworkStatusResponse } from './models';

export const useNetStatus = (
  netId: NetworkIdentifier
): [ApiState, NetworkStatusResponse | null] => {
  return useApi(
    '/network/status',
    useMemo(
      () => ({
        network_identifier: netId,
        metadata: {},
      }),
      [netId]
    )
  );
};
