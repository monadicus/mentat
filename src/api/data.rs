use super::*;

#[async_trait::async_trait]
pub trait DataApi: Send + Sync {
    /// This endpoint returns a list of NetworkIdentifiers that the Rosetta server supports.
    async fn network_list(
        &self,
        caller: Caller,
        data: MetadataRequest,
    ) -> Response<NetworkListResponse>;

    /// This endpoint returns the version information and allowed network-specific types for a NetworkIdentifier. Any NetworkIdentifier returned by /network/list should be accessible here. Because options are retrievable in the context of a NetworkIdentifier, it is possible to define unique options for each network.
    async fn network_options(
        &self,
        caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkOptionsResponse>;

    /// This endpoint returns the current status of the network requested. Any NetworkIdentifier returned by /network/list should be accessible here.
    async fn network_status(
        &self,
        caller: Caller,
        data: NetworkRequest,
    ) -> Response<NetworkStatusResponse>;

    /// Get an array of all AccountBalances for an AccountIdentifier and the BlockIdentifier at which the balance lookup was performed. The BlockIdentifier must always be returned because some consumers of account balance data need to know specifically at which block the balance was calculated to compare balances they compute from operations with the balance returned by the node. It is important to note that making a balance request for an account without populating the SubAccountIdentifier should not result in the balance of all possible SubAccountIdentifiers being returned. Rather, it should result in the balance pertaining to no SubAccountIdentifiers being returned (sometimes called the liquid balance). To get all balances associated with an account, it may be necessary to perform multiple balance requests with unique AccountIdentifiers. It is also possible to perform a historical balance lookup (if the server supports it) by passing in an optional BlockIdentifier.
    async fn account_balance(
        &self,
        caller: Caller,
        data: AccountBalanceRequest,
    ) -> Response<AccountBalanceResponse>;

    /// Get an array of all unspent coins for an AccountIdentifier and the BlockIdentifier at which the lookup was performed. If your implementation does not support coins (i.e. it is for an account-based blockchain), you do not need to implement this endpoint. If you implementation does support coins (i.e. it is fro a UTXO-based blockchain), you MUST also complete the /account/balance endpoint. It is important to note that making a coins request for an account without populating the SubAccountIdentifier should not result in the coins of all possible SubAccountIdentifiers being returned. Rather, it should result in the coins pertaining to no SubAccountIdentifiers being returned. To get all coins associated with an account, it may be necessary to perform multiple coin requests with unique AccountIdentifiers. Optionally, an implementation may choose to support updating an AccountIdentifier's unspent coins based on the contents of the mempool. Note, using this functionality breaks any guarantee of idempotency.
    async fn account_coins(
        &self,
        caller: Caller,
        data: AccountCoinsRequest,
    ) -> Response<AccountCoinsResponse>;

    /// Get a block by its Block Identifier. If transactions are returned in the same call to the node as fetching the block, the response should include these transactions in the Block object. If not, an array of Transaction Identifiers should be returned so /block/transaction fetches can be done to get all transaction information. When requesting a block by the hash component of the BlockIdentifier, this request MUST be idempotent: repeated invocations for the same hash-identified block must return the exact same block contents. No such restriction is imposed when requesting a block by height, given that a chain reorg event might cause the specific block at height n to be set to a different one.
    async fn block(&self, caller: Caller, data: BlockRequest) -> Response<BlockResponse>;

    /// Get a transaction in a block by its Transaction Identifier. This endpoint should only be used when querying a node for a block does not return all transactions contained within it. All transactions returned by this endpoint must be appended to any transactions returned by the /block method by consumers of this data. Fetching a transaction by hash is considered an Explorer Method (which is classified under the Future Work section). This method can be used to let consumers to paginate results when the block trasactions count is too big to be returned in a single BlockResponse. Calling this endpoint requires reference to a BlockIdentifier because transaction parsing can change depending on which block contains the transaction. For example, in Bitcoin it is necessary to know which block contains a transaction to determine the destination of fee payments. Without specifying a block identifier, the node would have to infer which block to use (which could change during a re-org). Implementations that require fetching previous transactions to populate the response (ex: Previous UTXOs in Bitcoin) may find it useful to run a cache within the Rosetta server in the /data directory (on a path that does not conflict with the node).
    async fn block_transaction(
        &self,
        caller: Caller,
        data: BlockTransactionRequest,
    ) -> Response<BlockTransactionResponse>;

    /// Get all Transaction Identifiers in the mempool
    async fn mempool(&self, caller: Caller, data: NetworkRequest) -> Response<MempoolResponse>;

    /// Make a Network-Specific Procedure Call
    async fn call(&self, _caller: Caller, data: CallRequest) -> Response<CallResponse>;

    /// Get a transaction in the mempool by its Transaction Identifier. This is a separate request than fetching a block transaction (/block/transaction) because some blockchain nodes need to know that a transaction query is for something in the mempool instead of a transaction in a block. Transactions may not be fully parsable until they are in a block (ex: may not be possible to determine the fee to pay before a transaction is executed). On this endpoint, it is ok that returned transactions are only estimates of what may actually be included in a block.
    async fn mempool_transaction(
        &self,
        caller: Caller,
        data: MempoolTransactionRequest,
    ) -> Response<MempoolTransactionResponse>;
}
