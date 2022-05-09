export type NetworkIdentifier = {
  blockchain: string;
  network: string;
};

export type NetworkStatusResponse = {
  current_block_timestamp: number;
  current_block_identifier: BlockIdentifier;
  genesis_block_identifier: BlockIdentifier;
  peers: { peer_id: string; metadata: Record<string, unknown> }[];
};

export type BlockIdentifier = {
  index: number;
  hash: string;
};

export type TransactionIdentifier = {
  hash: string;
};

export type OperationIdentifier = {
  index: number;
  network_index: number;
};

export type Account = {
  address: string;
  sub_account: {
    address: string;
    metadata: Record<string, unknown>;
  };
  metadata: Record<string, unknown>;
};

export type Currency = {
  symbol: string;
  decimals: number;
};

export type Amount = {
  value: string;
  currency: Currency;
};

export type CoinIdentifier = {
  identifier: string;
};

export type Operation = {
  operation_identifier: OperationIdentifier;
  type: string;
  status: string;
  account?: Account;
  amount?: Amount;
  coin_change?: { coin_identifier: CoinIdentifier; coin_action: string };
  metadata: Record<string, unknown>;
};

export type Transaction = {
  transaction_identifier: TransactionIdentifier;
  operations: Operation[];
};

export type BlockResponse = {
  block: {
    block_identifier: BlockIdentifier;
    parent_block_identifier: BlockIdentifier;
    timestamp: number;
    transactions: Transaction[];
  };
};
