import { useMemo } from 'react';
import { ApiState, useApi } from './hooks';
import {
  NetworkIdentifier,
  NetworkOptionsResponse,
  NetworkStatusResponse,
} from './models';

export const useNetStatus = (
  netId: NetworkIdentifier
): [ApiState, NetworkStatusResponse | null] => {
  return useApi<NetworkStatusResponse>(
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

export const useNetOptions = (
  netId: NetworkIdentifier
): [ApiState, NetworkOptionsResponse | null] => {
  return useApi<NetworkOptionsResponse>(
    '/network/options',
    useMemo(
      () => ({
        network_identifier: netId,
        metadata: {},
      }),
      [netId]
    )
  );
};
