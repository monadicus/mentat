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
        network_identifier(network.as_ref())?;

        if contains_network_identifier(&parsed, network.as_ref()) {
            Err(format!(
                "{}: {network:?}",
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
            Err(format!(
                "{}: {request_network:?}",
                ServerError::RequestedNetworkNotSupported
            ))?
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
        network_identifier(request_network)?;
        self.supported_network(request_network)
    }

    /// [`account_balance_request`] ensures that a [`AccountBalanceRequest`]
    /// is well-formatted.
    pub fn account_balance_request(
        &self,
        request: Option<&NullableAccountBalanceRequest>,
    ) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        let request = request.ok_or(ServerError::AccountBalanceRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;
        account_identifier(request.account_identifier.as_ref())?;
        if let Some(c) = contains_duplicate_currency(
            &request
                .currencies
                .iter()
                .map(|i| i.as_ref())
                .collect::<Vec<_>>(),
        ) {
            Err(format!("{}: {c:?}", ServerError::DuplicateCurrency))?
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
    pub fn block_request(&self, request: Option<&NullableBlockRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::BlockRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        partial_block_identifier(request.block_identifier.as_ref())
    }

    /// [`block_transaction_request`] ensures that a [`BlockTransactionRequest`]
    /// is well-formatted.
    pub fn block_transaction_request(
        &self,
        request: Option<&NullableBlockTransactionRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::BlockTransactionRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        block_identifier(request.block_identifier.as_ref())?;
        transaction_identifier(request.transaction_identifier.as_ref())
    }

    /// [`construction_metadata_request`] ensures that a
    /// [`ConstructionMetadataRequest`] is well-formatted.
    pub fn construction_metadata_request(
        &self,
        request: Option<&NullableConstructionMetadataRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionMetadataRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;

        request
            .public_keys
            .iter()
            .try_for_each(|k| public_key(k.as_ref()))
    }

    /// [`construction_submit_request`] ensures that a
    /// [`ConstructionSubmitRequest`] is well-formatted.
    pub fn construction_submit_request(
        &self,
        request: Option<&NullableConstructionSubmitRequest>,
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
        request: Option<&NullableMempoolTransactionRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::MempoolTransactionRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        transaction_identifier(request.transaction_identifier.as_ref())
    }

    /// [`metadata_request`] ensures that a [`MetadataRequest`]
    /// is well-formatted.
    pub fn metadata_request(&self, request: Option<&NullableMetadataRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        request.ok_or(ServerError::MetadataRequestIsNil)?;
        Ok(())
    }

    /// [`network_request`] ensures that a [`NetworkRequest`]
    /// is well-formatted.
    pub fn network_request(&self, request: Option<&NullableNetworkRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::NetworkRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())
    }

    /// [`construction_derive_request`] ensures that a
    /// [`ConstructionDeriveRequest`] is well-formatted.
    pub fn construction_derive_request(
        &self,
        request: Option<&NullableConstructionDeriveRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionDeriveRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        public_key(request.public_key.as_ref())
    }

    /// [`construction_preprocess_request`] ensures that a
    /// [`ConstructionPreprocessRequest`] is well-formatted.
    pub fn construction_preprocess_request(
        &self,
        request: Option<&NullableConstructionPreprocessRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionPreprocessRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.operations(&request.operations, true)?;
        assert_unique_amounts(&request.max_fee)
            .map_err(|e| format!("{e}: duplicate max fee currency found"))?;
        if matches!(request.suggested_fee_multiplier, Some(i) if i < 0.0) {
            Err(format!(
                "{}: {}",
                ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
                request.suggested_fee_multiplier.unwrap()
            ))?
        } else {
            Ok(())
        }
    }

    /// [`construction_payload_request`] ensures that a
    /// [`ConstructionPayloadsRequest`] is well-formatted.
    pub fn construction_payload_request(
        &self,
        request: Option<&NullableConstructionPayloadsRequest>,
    ) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::ConstructionPayloadsRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.operations(&request.operations, true)?;
        request
            .public_keys
            .iter()
            .try_for_each(|k| public_key(k.as_ref()))
    }

    /// [`construction_combine_request`] ensures that a
    /// [`ConstructionCombineRequest`] is well-formatted.
    pub fn construction_combine_request(
        &self,
        request: Option<&NullableConstructionCombineRequest>,
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
        }
    }

    /// [`construction_hash_request`] ensures that a [`ConstructionHashRequest`]
    /// is well-formatted.
    pub fn construction_hash_request(
        &self,
        request: Option<&NullableConstructionHashRequest>,
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
        request: Option<&NullableConstructionParseRequest>,
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
            .ok_or_else(|| format!("{}: {method}", ServerError::CallMethodUnsupported))?;
        Ok(())
    }

    /// [`call_request`] ensures that a [`CallRequest`]
    /// is well-formatted.
    pub fn call_request(&self, request: Option<&NullableCallRequest>) -> AssertResult<()> {
        self.request.as_ref().ok_or(AsserterError::NotInitialized)?;
        let request = request.ok_or(ServerError::CallRequestIsNil)?;
        self.valid_supported_network(request.network_identifier.as_ref())?;
        self.valid_call_method(request.method.as_ref())
    }

    /// [`account_coins_request`] ensures that a [`AccountCoinsRequest`]
    /// is well-formatted.
    pub fn account_coins_request(
        &self,
        request: Option<&NullableAccountCoinsRequest>,
    ) -> AssertResult<()> {
        let asserter = self.request.as_ref().ok_or(AsserterError::NotInitialized)?;

        let request = request.ok_or(ServerError::AccountCoinsRequestIsNil)?;

        self.valid_supported_network(request.network_identifier.as_ref())?;

        account_identifier(request.account_identifier.as_ref())?;

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
            Err(format!("{}: {c:?}", ServerError::DuplicateCurrency))?
        } else {
            Ok(())
        }
    }

    /// [`events_block_request`] ensures that a [`EventsBlocksRequest`]
    /// is well-formatted.
    pub fn events_block_request(
        &self,
        request: Option<&NullableEventsBlocksRequest>,
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
        request: Option<&NullableSearchTransactionsRequest>,
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
            transaction_identifier(request.transaction_identifier.as_ref())?;
        }

        if request.account_identifier.is_some() {
            account_identifier(request.account_identifier.as_ref())?;
        }

        if request.coin_identifier.is_some() {
            coin_identifier(request.coin_identifier.as_ref())?;
        }

        if request.currency.is_some() {
            currency(request.currency.as_ref())?;
        }

        if request.status.is_some() {
            self.operation_status(request.status.as_ref(), false)?;
        }

        if let Some(t) = &request.type_ {
            self.operation_type(t.clone())?;
        }

        if matches!(&request.address, Some(a) if a.is_empty()) {
            Err(BlockError::AccountAddrMissing)?
        } else {
            Ok(())
        }
    }
}
