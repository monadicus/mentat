import { NetworkIdentifier } from '../rosetta/models';

export const netIdStr = (netId: NetworkIdentifier) =>
  `${netId.blockchain}.${netId.network}`;
