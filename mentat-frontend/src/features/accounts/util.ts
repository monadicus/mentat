import { NetworkIdentifier } from '../rosetta/models';

export const netIdStr = (netId: NetworkIdentifier) =>
  `${netId.blockchain}.${netId.network}${
    netId.sub_network_identifier?.network
      ? '.' + netId.sub_network_identifier.network
      : ''
  }`;
