import { useMemo } from 'react';
import { ApiState, useApi } from './hooks';
import { NetworkIdentifier, NetworkStatus } from './models';

export const useNetStatus = (
  netId: NetworkIdentifier
): [ApiState, NetworkStatus | null] => {
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
