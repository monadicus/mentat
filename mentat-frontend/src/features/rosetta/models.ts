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

export type RosettaError = {
  code: number;
  message: string;
  description: string;
  retriable: boolean;
  details: Record<string, unknown>;
};

export type NetworkOptionsResponse = {
  version: {
    rosetta_version: string;
    node_version: string;
    middleware_version: string;
    metadata: Record<string, unknown>;
  };
  allow: {
    operation_statuses: { status: string; successful: boolean }[];
    operation_types: string[];
    errors: RosettaError[];
    historical_balance_lookup: boolean;
    timestamp_start_index: number;
    call_methods: string[];
    balance_exemptions: {
      sub_account_address: string;
      currency: Currency;
      exemption_type: string;
    }[];
    mempool_coins: boolean;
    block_hash_case: string;
    transaction_hash_case: string;
  };
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
  sub_account?: {
    address: string;
    metadata: Record<string, unknown>;
  };
  metadata?: Record<string, unknown>;
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

export type BalanceResponse = {
  block_identifier: BlockIdentifier;
  balances: Amount[];
  metadata: Record<string, unknown>;
};
