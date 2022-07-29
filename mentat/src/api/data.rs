//! Houses the traits for the Rosetta Data API.
//! These traits are easily overridable for custom
//! implementations.
use super::*;

/// Trait to define the endpoints necessary for the Rosetta Data API.
#[axum::async_trait]
pub trait DataApi: Default {
    /// This endpoint returns a list of
    /// [`crate::identifiers::NetworkIdentifier`]s that the Rosetta
    /// server supports.
    async fn network_list(
        &self,
        _caller: Caller,
        _data: MetadataRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<NetworkListResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint returns the version information and allowed
    /// network-specific types for a [`crate::identifiers::NetworkIdentifier`].
    /// Any [`crate::identifiers::NetworkIdentifier`] returned by
    /// /network/list should be accessible here. Because options are
    /// retrievable in the context of a
    /// [`crate::identifiers::NetworkIdentifier`], it is possible to define
    /// unique options for each network.
    async fn network_options(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<NetworkOptionsResponse> {
        MentatError::not_implemented()
    }

    /// This endpoint returns the current status of the network requested. Any
    /// [`crate::identifiers::NetworkIdentifier`] returned by /network/list
    /// should be accessible here.
    async fn network_status(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<NetworkStatusResponse> {
        MentatError::not_implemented()
    }

    /// Get an array of all AccountBalances for an
    /// [`crate::identifiers::AccountIdentifier`] and the
    /// [`crate::identifiers::BlockIdentifier`] at which the balance lookup was
    /// performed. The [`crate::identifiers::BlockIdentifier`] must always
    /// be returned because some consumers of account balance __data need to
    /// know specifically at which block the balance was calculated to
    /// compare balances they compute from operations with the balance
    /// returned by the node. It is important to note that making a balance
    /// request for an account without populating the [`crate::identifiers::
    /// SubAccountIdentifier`] should not result in the balance of all
    /// possible [`crate::identifiers::SubAccountIdentifier`]s being returned.
    /// Rather, it should result in the balance pertaining to no
    /// [`crate::identifiers::SubAccountIdentifier`]s being returned
    /// (sometimes called the liquid balance). To get all
    /// balances associated with an account, it may be necessary to perform
    /// multiple balance requests with unique
    /// [`crate::identifiers::AccountIdentifier`]s. It is also possible to
    /// perform a historical balance lookup (if the server supports it) by
    /// passing in an optional [`crate::identifiers::BlockIdentifier`].
    async fn account_balance(
        &self,
        _caller: Caller,
        _data: AccountBalanceRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<AccountBalanceResponse> {
        MentatError::not_implemented()
    }

    /// Get an array of all unspent coins for an
    /// [`crate::identifiers::AccountIdentifier`] and the
    /// [`crate::identifiers::BlockIdentifier`] at which the lookup was
    /// performed. If your implementation does not support coins (i.e. it is
    /// for an account-based blockchain), you do not need to implement this
    /// endpoint. If you implementation does support coins (i.e. it is fro a
    /// UTXO-based blockchain), you MUST also complete the /account/balance
    /// endpoint. It is important to note that making a coins request for an
    /// account without populating the
    /// [`crate::identifiers::SubAccountIdentifier`] should not result in the
    /// coins of all possible [`crate::identifiers::SubAccountIdentifier`]s
    /// being returned. Rather, it should result in the coins pertaining to
    /// no [`crate::identifiers::SubAccountIdentifier`]s being returned. To
    /// get all coins associated with an account, it may be necessary to
    /// perform multiple coin requests with unique
    /// [`crate::identifiers::AccountIdentifier]`s. Optionally, an
    /// implementation may choose to support updating an
    /// [`crate::identifiers::AccountIdentifier`]'s unspent coins based on the
    /// contents of the mempool. Note, using this functionality breaks any
    /// guarantee of idempotency.
    async fn account_coins(
        &self,
        _caller: Caller,
        _data: AccountCoinsRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<AccountCoinsResponse> {
        MentatError::not_implemented()
    }

    /// Get a block by its [`crate::identifiers::BlockIdentifier`]. If
    /// transactions are returned in the same call to the node as fetching
    /// the block, the response should include these transactions in the
    /// Block object. If not, an array of [`crate::identifiers::
    /// TransactionIdentifier`]s should be returned so /block/transaction
    /// fetches can be done to get all transaction information. When
    /// requesting a block by the hash component of the
    /// [`crate::identifiers::BlockIdentifier`], this request MUST be
    /// idempotent: repeated invocations for the same hash-identified block
    /// must return the exact same block contents. No such restriction is
    /// imposed when requesting a block by height, given that a chain reorg
    /// event might cause the specific block at height n to be set to a
    /// different one.
    async fn block(
        &self,
        _caller: Caller,
        _data: BlockRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<BlockResponse> {
        MentatError::not_implemented()
    }

    /// Get a transaction in a block by its
    /// [`crate::identifiers::TransactionIdentifier`]. This endpoint should
    /// only be used when querying a node for a block does not return all
    /// transactions contained within it. All transactions returned
    /// by this endpoint must be appended to any transactions returned by the
    /// /block method by consumers of this __data. Fetching a transaction by
    /// hash is considered an Explorer Method (which is classified under the
    /// Future Work section). This method can be used to let consumers to
    /// paginate results when the block transactions count is too big to be
    /// returned in a single [`BlockResponse`]. Calling this endpoint requires
    /// reference to a [`crate::identifiers::BlockIdentifier`] because
    /// transaction parsing can change depending on which block contains the
    /// transaction. For example, in Bitcoin it is necessary to know which
    /// block contains a transaction to determine the destination of fee
    /// payments. Without specifying a block identifier, the node would have
    /// to infer which block to use (which could change during a re-org).
    /// Implementations that require fetching previous transactions to
    /// populate the response (ex: Previous UTXOs in Bitcoin) may find it
    /// useful to run a cache within the Rosetta server in the /__data
    /// directory (on a path that does not conflict with the node).
    async fn block_transaction(
        &self,
        _caller: Caller,
        _data: BlockTransactionRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<BlockTransactionResponse> {
        MentatError::not_implemented()
    }

    /// Get all [`crate::identifiers::TransactionIdentifier`]s in the mempool
    async fn mempool(
        &self,
        _caller: Caller,
        _data: NetworkRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<MempoolResponse> {
        MentatError::not_implemented()
    }

    /// Get a transaction in the mempool by its
    /// [`crate::identifiers::TransactionIdentifier`]. This is a separate
    /// request than fetching a block transaction (/block/transaction)
    /// because some blockchain nodes need to know that a transaction query
    /// is for something in the mempool instead of a transaction in a block.
    /// Transactions may not be fully parsable until they are in a block
    /// (ex: may not be possible to determine the fee to pay
    /// before a transaction is executed). On this endpoint, it is ok that
    /// returned transactions are only estimates of what may actually be
    /// included in a block.
    async fn mempool_transaction(
        &self,
        _caller: Caller,
        _data: MempoolTransactionRequest,
        _rpc_caller: RpcCaller,
    ) -> Result<MempoolTransactionResponse> {
        MentatError::not_implemented()
    }
}

/// Trait to wrap the `DataApi`.
/// This trait helps to define default behavior for running the endpoints
/// on different modes.
#[axum::async_trait]
pub trait CallerDataApi: Clone + DataApi {
    /// This endpoint runs in both offline and online mode.
    async fn call_network_list(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableMetadataRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableNetworkListResponse> {
        asserter.metadata_request(data.as_ref())?;
        let resp = self
            .network_list(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        // if assert_resp {
        //     network_list_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }

    /// This endpoint runs in both offline and online mode.
    async fn call_network_options(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableNetworkRequest>,
        _mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableNetworkOptionsResponse> {
        asserter.network_request(data.as_ref())?;
        let resp = self
            .network_options(caller, data.unwrap().into(), rpc_caller)
            .await?
            .into();
        // if assert_resp {
        //     network_options_response(Some(&resp)).unwrap();
        // }
        Ok(Json(resp))
    }

    /// This endpoint only runs in online mode.
    async fn call_network_status(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableNetworkRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableNetworkStatusResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.network_request(data.as_ref())?;
            let resp = self
                .network_status(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     network_status_response(Some(&resp)).unwrap();
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_account_balance(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableAccountBalanceRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableAccountBalanceResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.account_balance_request(data.as_ref())?;
            // let ident = if assert_resp {
            //     data.as_ref().unwrap().block_identifier.clone()
            // } else {
            //     None
            // };
            let resp = self
                .account_balance(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     account_balance_response(ident.as_ref(), &resp).unwrap();
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_account_coins(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableAccountCoinsRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableAccountCoinsResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.account_coins_request(data.as_ref())?;
            let resp = self
                .account_coins(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     account_coins(&resp).unwrap();
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_block(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableBlockRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableBlockResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.block_request(data.as_ref())?;
            let resp: NullableBlockResponse = self
                .block(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     asserter.block(resp.block.as_ref()).unwrap();
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_block_transaction(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableBlockTransactionRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableBlockTransactionResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.block_transaction_request(data.as_ref())?;
            let resp: NullableBlockTransactionResponse = self
                .block_transaction(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     asserter.transaction(resp.transaction.as_ref()).unwrap();
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_mempool(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableNetworkRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableMempoolResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.network_request(data.as_ref())?;
            let resp: NullableMempoolResponse = self
                .mempool(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     mempool_transactions(&resp.transaction_identifiers).unwrap()
            // }
            Ok(Json(resp))
        }
    }

    /// This endpoint only runs in online mode.
    async fn call_mempool_transaction(
        &self,
        asserter: &Asserter,
        caller: Caller,
        data: Option<NullableMempoolTransactionRequest>,
        mode: &Mode,
        rpc_caller: RpcCaller,
    ) -> MentatResponse<NullableMempoolTransactionResponse> {
        if mode.is_offline() {
            MentatError::wrong_network(Some(mode))
        } else {
            asserter.mempool_transaction_request(data.as_ref())?;
            let resp: NullableMempoolTransactionResponse = self
                .mempool_transaction(caller, data.unwrap().into(), rpc_caller)
                .await?
                .into();
            // if assert_resp {
            //     asserter.transaction(resp.transaction.as_ref()).unwrap();
            // }
            Ok(Json(resp))
        }
    }
}
