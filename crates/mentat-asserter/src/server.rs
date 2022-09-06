//! Validates that server data is correct.

use super::*;

/// [`supported_networks`] returns an error if there is an invalid
/// [`NetworkIdentifier`] or there is a duplicate.
pub fn supported_networks(networks: &[Option<NetworkIdentifier>]) -> AssertResult<()> {
    if networks.is_empty() {
        Err(ServerError::NoSupportedNetworks)?
    }

    let mut parsed = Vec::new();
    for network in networks {
        network_identifier(network.as_ref())
            .map_err(|e| format!("network identifier {network:?} is invalid: {e}",))?;

        if contains_network_identifier(&parsed, network.as_ref()) {
            Err(format!(
                "network identifier {network:?} is invalid: {}",
                ServerError::SupportedNetworksDuplicate
            ))?;
        }
        parsed.push(network.clone().unwrap());
    }

    Ok(())
}

impl Asserter {
    /// [`supported_network`] returns a boolean indicating if the
    /// [`NetworkIdentifier`] is allowed. This should be called after the
    /// [`NetworkIdentifier`] is asserted.
    pub fn supported_network(
        &self,
        request_network: Option<&NetworkIdentifier>,
    ) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        if !contains_network_identifier(&asserter.supported_networks, request_network) {
            Err(ServerError::RequestedNetworkNotSupported)?
        } else {
            Ok(())
        }
    }

    /// [`valid_supported_network`] returns an error if a [`NetworkIdentifier`]
    /// is not valid or not supported.
    pub fn valid_supported_network(
        &self,
        request_network: Option<&NetworkIdentifier>,
    ) -> AssertResult<()> {
        network_identifier(request_network)
            .map_err(|e| format!("network identifier {request_network:?} is invalid: {e}"))?;
        self.supported_network(request_network).map_err(|e| {
            format!("network identifier {request_network:?} is not supported: {e}").into()
        })
    }

    /// [`account_balance_request`] ensures that a [`AccountBalanceRequest`]
    /// is well-formatted.
    pub fn account_balance_request(
        &self,
        request: Option<&UncheckedAccountBalanceRequest>,
    ) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        let request = request.ok_or(ServerError::AccountBalanceRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;
        account_identifier(request.account_identifier.as_ref()).map_err(|e| {
            format!(
                "account identifier {:?} is invalid: {e}",
                request.account_identifier
            )
        })?;
        if let Some(c) = contains_duplicate_currency(
            &request
                .currencies
                .iter()
                .map(|i| i.as_ref())
                .collect::<Vec<_>>(),
        ) {
            Err(format!(
                "currency {c:?} is invalid: {}",
                ServerError::DuplicateCurrency
            ))?
        } else if request.block_identifier.is_none() {
            Ok(())
        } else if !asserter.historical_balance_lookup {
            Err(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported)?
        } else {
            partial_block_identifier(Some(request.block_identifier.as_ref().unwrap()))
        }
    }

    /// [`block_request`] ensures that a [`BlockRequest`]
    /// is well-formatted.
    pub fn block_request(&self, request: Option<&UncheckedBlockRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::BlockRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        partial_block_identifier(request.block_identifier.as_ref())
    }

    /// [`block_transaction_request`] ensures that a [`BlockTransactionRequest`]
    /// is well-formatted.
    pub fn block_transaction_request(
        &self,
        request: Option<&UncheckedBlockTransactionRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::BlockTransactionRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        block_identifier(request.block_identifier.as_ref()).map_err(|e| {
            format!(
                "block identifier {:?} is invalid: {e}",
                request.block_identifier
            )
        })?;
        transaction_identifier(request.transaction_identifier.as_ref())
    }

    /// [`construction_metadata_request`] ensures that a
    /// [`ConstructionMetadataRequest`] is well-formatted.
    pub fn construction_metadata_request(
        &self,
        request: Option<&UncheckedConstructionMetadataRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionMetadataRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;

        request.public_keys.iter().try_for_each(|k| {
            public_key(k.as_ref()).map_err(|e| format!("public key {k:?} is invalid: {e}").into())
        })
    }

    /// [`construction_submit_request`] ensures that a
    /// [`ConstructionSubmitRequest`] is well-formatted.
    pub fn construction_submit_request(
        &self,
        request: Option<&UncheckedConstructionSubmitRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionSubmitRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        if request.signed_transaction.is_empty() {
            Err(ServerError::ConstructionHashRequestSignedTxEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`mempool_transaction_request`] ensures that a
    /// [`MempoolTransactionRequest`] is well-formatted.
    pub fn mempool_transaction_request(
        &self,
        request: Option<&UncheckedMempoolTransactionRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::MempoolTransactionRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        transaction_identifier(request.transaction_identifier.as_ref())
    }

    /// [`metadata_request`] ensures that a [`MetadataRequest`]
    /// is well-formatted.
    pub fn metadata_request(&self, request: Option<&UncheckedMetadataRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        request.ok_or(ServerError::MetadataRequestIsNil)?;
        Ok(())
    }

    /// [`network_request`] ensures that a [`NetworkRequest`]
    /// is well-formatted.
    pub fn network_request(&self, request: Option<&UncheckedNetworkRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::NetworkRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())
    }

    /// [`construction_derive_request`] ensures that a
    /// [`ConstructionDeriveRequest`] is well-formatted.
    pub fn construction_derive_request(
        &self,
        request: Option<&UncheckedConstructionDeriveRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionDeriveRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        public_key(request.public_key.as_ref())
            .map_err(|e| format!("public key {:?} is invalid: {e}", request.public_key).into())
    }

    /// [`construction_preprocess_request`] ensures that a
    /// [`ConstructionPreprocessRequest`] is well-formatted.
    pub fn construction_preprocess_request(
        &self,
        request: Option<&UncheckedConstructionPreprocessRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionPreprocessRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.operations(&request.operations, true)
            .map_err(|e| format!("operations {:?} are invalid: {e}", request.operations))?;
        assert_unique_amounts(&request.max_fee)
            .map_err(|e| format!("max fee {:?} is invalid: {e}", request.max_fee))?;
        if matches!(request.suggested_fee_multiplier, Some(i) if i < 0.0) {
            Err(format!(
                "suggested fee multiplier {} is invalid: {}",
                request.suggested_fee_multiplier.unwrap(),
                ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
            ))?
        } else {
            Ok(())
        }
    }

    /// [`construction_payload_request`] ensures that a
    /// [`ConstructionPayloadsRequest`] is well-formatted.
    pub fn construction_payload_request(
        &self,
        request: Option<&UncheckedConstructionPayloadsRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionPayloadsRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.operations(&request.operations, true)
            .map_err(|e| format!("operations {:?} are invalid: {e}", request.operations))?;
        request.public_keys.iter().try_for_each(|k| {
            public_key(k.as_ref()).map_err(|e| format!("public key {k:?} is invalid: {e}",).into())
        })
    }

    /// [`construction_combine_request`] ensures that a
    /// [`ConstructionCombineRequest`] is well-formatted.
    pub fn construction_combine_request(
        &self,
        request: Option<&UncheckedConstructionCombineRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionCombineRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        if request.unsigned_transaction.is_empty() {
            Err(ServerError::ConstructionCombineRequestUnsignedTxEmpty)?
        } else {
            signatures(
                &request
                    .signatures
                    .iter()
                    .map(|i| i.as_ref())
                    .collect::<Vec<_>>(),
            )
            .map_err(|e| format!("signatures {:?} are invalid: {e}", request.signatures).into())
        }
    }

    /// [`construction_hash_request`] ensures that a [`ConstructionHashRequest`]
    /// is well-formatted.
    pub fn construction_hash_request(
        &self,
        request: Option<&UncheckedConstructionHashRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionHashRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        if request.signed_transaction.is_empty() {
            Err(ServerError::ConstructionHashRequestSignedTxEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`construction_parse_request`] ensures that a
    /// [`ConstructionParseRequest`] is well-formatted.
    pub fn construction_parse_request(
        &self,
        request: Option<&UncheckedConstructionParseRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionParseRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        if request.transaction.is_empty() {
            Err(ServerError::ConstructionParseRequestEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`valid_call_method`] returns an error if a [`CallRequest`] method
    /// is not valid.
    pub fn valid_call_method(&self, method: &str) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        if method.is_empty() {
            Err(ServerError::CallMethodEmpty)?
        }

        asserter
            .call_methods
            .get(method)
            .ok_or(ServerError::CallMethodUnsupported)?;
        Ok(())
    }

    /// [`call_request`] ensures that a [`CallRequest`]
    /// is well-formatted.
    pub fn call_request(&self, request: Option<&UncheckedCallRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::CallRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.valid_call_method(request.method.as_ref())
            .map_err(|e| format!("method {} is invalid: {e}", request.method).into())
    }

    /// [`account_coins_request`] ensures that a [`AccountCoinsRequest`]
    /// is well-formatted.
    pub fn account_coins_request(
        &self,
        request: Option<&UncheckedAccountCoinsRequest>,
    ) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        let request = request.ok_or(ServerError::AccountCoinsRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;

        account_identifier(request.account_identifier.as_ref()).map_err(|e| {
            format!(
                "account identifier {:?} is invalid: {e}",
                request.account_identifier
            )
        })?;

        if request.include_mempool && !asserter.mempool_coins {
            Err(ServerError::MempoolCoinsNotSupported)?
        }

        if let Some(c) = contains_duplicate_currency(
            &request
                .currencies
                .iter()
                .map(|i| i.as_ref())
                .collect::<Vec<_>>(),
        ) {
            Err(format!(
                "currency {c:?} is invalid: {}",
                ServerError::DuplicateCurrency
            ))?
        } else {
            Ok(())
        }
    }

    /// [`events_block_request`] ensures that a [`EventsBlocksRequest`]
    /// is well-formatted.
    pub fn events_block_request(
        &self,
        request: Option<&UncheckedEventsBlocksRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::EventsBlocksRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        if matches!(request.offset, Some(i) if i < 0) {
            Err(ServerError::OffsetIsNegative)?
        } else if matches!(request.limit, Some(i) if i < 0) {
            Err(ServerError::LimitIsNegative)?
        } else {
            Ok(())
        }
    }

    /// [`search_transactions_request`] ensures that a
    /// [`SearchTransactionsRequest`] is well-formatted.
    pub fn search_transactions_request(
        &self,
        request: Option<&UncheckedSearchTransactionsRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::SearchTransactionsRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;

        if request.operator.is_some() && !request.operator.as_ref().unwrap().valid() {
            Err(ServerError::OperatorInvalid)?;
        }

        if matches!(request.max_block, Some(i) if i < 0) {
            Err(ServerError::MaxBlockInvalid)?
        } else if matches!(request.offset, Some(i) if i < 0) {
            Err(ServerError::OffsetIsNegative)?
        } else if matches!(request.limit, Some(i) if i < 0) {
            Err(ServerError::LimitIsNegative)?
        }

        if request.transaction_identifier.is_some() {
            transaction_identifier(request.transaction_identifier.as_ref()).map_err(|e| {
                format!(
                    "transaction identifier {:?} is invalid: {e}",
                    request.transaction_identifier
                )
            })?;
        }

        if request.account_identifier.is_some() {
            account_identifier(request.account_identifier.as_ref()).map_err(|e| {
                format!(
                    "account identifier {:?} is invalid: {e}",
                    request.account_identifier
                )
            })?;
        }

        if request.coin_identifier.is_some() {
            coin_identifier(request.coin_identifier.as_ref()).map_err(|e| {
                format!(
                    "coin identifier {:?} is invalid: {e}",
                    request.coin_identifier
                )
            })?;
        }

        if request.currency.is_some() {
            currency(request.currency.as_ref())
                .map_err(|e| format!("currency {:?} is invalid: {e}", request.currency))?;
        }

        if request.status.is_some() {
            self.operation_status(request.status.as_ref(), false)
                .map_err(|e| format!("operation status {:?} is invalid: {e}", request.status))?;
        }

        if let Some(t) = &request.type_ {
            self.operation_type(t.clone())
                .map_err(|e| format!("operation type {t:?} is invalid: {e}"))?;
        }

        if matches!(&request.address, Some(a) if a.is_empty()) {
            Err(BlockError::AccountAddrMissing)?
        } else {
            Ok(())
        }
    }
}
