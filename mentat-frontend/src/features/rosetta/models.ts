export type NetworkIdentifier = {
  blockchain: string;
  network: string;
};

export type NetworkStatus = {
  current_block_timestamp: number;
  current_block_identifier: BlockIdentifier;
  genesis_block_identifier: BlockIdentifier;
  peers: unknown[];
};

export type BlockIdentifier = {
  index: number;
  hash: string;
};
