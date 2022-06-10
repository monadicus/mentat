//! Validates that server data is correct.

use super::{
    account::{assert_unique_amounts, contains_duplicate_currency},
    asserter_tools::RequestAsserter,
    block::{
        account_identifier,
        block_identifier,
        currency,
        partial_block_identifier,
        transaction_identifier,
    },
    coin::coin_identifier,
    construction::{public_key, signatures},
    errors::{AssertResult, BlockError, ServerError},
    network::{contains_network_identifier, network_identifier},
};
use crate::{
    identifiers::NetworkIdentifier,
    macro_exports::{
        AccountBalanceRequest,
        AccountCoinsRequest,
        BlockRequest,
        BlockTransactionRequest,
        CallRequest,
        ConstructionCombineRequest,
        ConstructionDeriveRequest,
        ConstructionHashRequest,
        ConstructionMetadataRequest,
        ConstructionParseRequest,
        ConstructionPayloadsRequest,
        ConstructionPreprocessRequest,
        ConstructionSubmitRequest,
        EventsBlocksRequest,
        MempoolTransactionRequest,
        MetadataRequest,
        NetworkRequest,
        SearchTransactionsRequest,
    },
};

/// [`supported_networks`] returns an error if there is an invalid
/// [`NetworkIdentifier`] or there is a duplicate.
pub(crate) fn supported_networks(networks: &[NetworkIdentifier]) -> AssertResult<()> {
    if networks.is_empty() {
        Err(ServerError::NoSupportedNetworks)?
    }

    let mut parsed = Vec::new();
    for network in networks {
        network_identifier(network)?;

        if contains_network_identifier(networks, network) {
            Err(format!(
                "{}: {network:?}",
                ServerError::SupportedNetworksDuplicate
            ))?;
        }
        parsed.push(network);
    }

    Ok(())
}

impl RequestAsserter {
    /// [`supported_network`] returns a boolean indicating if the
    /// [`NetworkIdentifier`] is allowed. This should be called after the
    /// [`NetworkIdentifier`] is asserted.
    pub(crate) fn supported_network(
        &self,
        request_network: &NetworkIdentifier,
    ) -> AssertResult<()> {
        // TODO if self == nil
        if !contains_network_identifier(&self.supported_networks, request_network) {
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
    pub(crate) fn valid_supported_network(
        &self,
        request_network: &NetworkIdentifier,
    ) -> AssertResult<()> {
        network_identifier(request_network)?;
        self.supported_network(request_network)
    }

    /// [`account_balance_request`] ensures that a [`AccountBalanceRequest`]
    /// is well-formatted.
    pub(crate) fn account_balance_request(
        &self,
        request: &AccountBalanceRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        account_identifier(Some(&request.account_identifier))?;
        if let Some(c) =
            contains_duplicate_currency(request.currencies.as_ref().unwrap_or(&Vec::new()))
        {
            Err(format!("{}: {c:?}", ServerError::DuplicateCurrency))?
        } else if request.block_identifier.is_none() {
            Ok(())
        } else if !self.historical_balance_lookup {
            Err(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported)?
        } else {
            partial_block_identifier(request.block_identifier.as_ref().unwrap())
        }
    }

    /// [`block_request`] ensures that a [`BlockRequest`]
    /// is well-formatted.
    pub(crate) fn block_request(&self, request: &BlockRequest) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        partial_block_identifier(&request.block_identifier)
    }

    /// [`block_transaction_request`] ensures that a [`BlockTransactionRequest`]
    /// is well-formatted.
    pub(crate) fn block_transaction_request(
        &self,
        request: BlockTransactionRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        block_identifier(&request.block_identifier)?;
        transaction_identifier(&request.transaction_identifier)
    }

    /// [`construction_metadata_request`] ensures that a
    /// [`ConstructionMetadataRequest`] is well-formatted.
    pub(crate) fn construction_metadata_request(
        &self,
        request: &ConstructionMetadataRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;

        request
            .public_keys
            .iter()
            .flatten()
            .try_for_each(public_key)
    }

    /// [`construction_submit_request`] ensures that a
    /// [`ConstructionSubmitRequest`] is well-formatted.
    pub(crate) fn construction_submit_request(
        &self,
        request: &ConstructionSubmitRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        if request.signed_transaction.is_empty() {
            Err(ServerError::ConstructionHashRequestSignedTxEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`mempool_transaction_request`] ensures that a
    /// [`MempoolTransactionRequest`] is well-formatted.
    pub(crate) fn mempool_transaction_request(
        &self,
        request: &MempoolTransactionRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        transaction_identifier(&request.transaction_identifier)
    }

    /// [`metadata_request`] ensures that a [`MetadataRequest`]
    /// is well-formatted.
    pub(crate) fn metadata_request(&self, request: &MetadataRequest) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        Ok(())
    }

    /// [`network_request`] ensures that a [`NetworkRequest`]
    /// is well-formatted.
    pub(crate) fn network_request(&self, request: &NetworkRequest) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)
    }

    /// [`construction_derive_request`] ensures that a
    /// [`ConstructionDeriveRequest`] is well-formatted.
    pub(crate) fn construction_derive_request(
        &self,
        request: &ConstructionDeriveRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        public_key(&request.public_key)
    }

    /// [`construction_preprocess_request`] ensures that a
    /// [`ConstructionPreprocessRequest`] is well-formatted.
    pub(crate) fn construction_preprocess_request(
        &self,
        request: &ConstructionPreprocessRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        // self.operations(&request.operations, true)?;
        assert_unique_amounts(request.max_fee.as_ref().unwrap_or(&Vec::new()))
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
    pub(crate) fn construction_payload_request(
        &self,
        request: &ConstructionPayloadsRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // todo if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        // self.operations(&request.operations, true)?;
        request
            .public_keys
            .iter()
            .flatten()
            .try_for_each(public_key)
    }

    /// [`construction_combine_request`] ensures that a
    /// [`ConstructionCombineRequest`] is well-formatted.
    pub(crate) fn construction_combine_request(
        &self,
        request: &ConstructionCombineRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        if request.unsigned_transaction.is_empty() {
            Err(ServerError::ConstructionCombineRequestUnsignedTxEmpty)?
        } else {
            signatures(&request.signatures)
        }
    }

    /// [`construction_hash_request`] ensures that a [`ConstructionHashRequest`]
    /// is well-formatted.
    pub(crate) fn construction_hash_request(
        &self,
        request: &ConstructionHashRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        if request.signed_transaction.is_empty() {
            Err(ServerError::ConstructionHashRequestSignedTxEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`construction_parse_request`] ensures that a
    /// [`ConstructionParseRequest`] is well-formatted.
    pub(crate) fn construction_parse_request(
        &self,
        request: &ConstructionParseRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        if request.transaction.is_empty() {
            Err(ServerError::ConstructionParseRequestEmpty)?
        } else {
            Ok(())
        }
    }

    /// [`valid_call_method`] returns an error if a [`CallRequest`] method
    /// is not valid.
    pub(crate) fn valid_call_method(&self, method: &str) -> AssertResult<()> {
        // TODO if self == nil
        if method.is_empty() {
            Err(ServerError::CallMethodEmpty)?
        }

        self.call_methods
            .get(method)
            .ok_or_else(|| format!("{}: {method}", ServerError::CallMethodUnsupported))?;
        Ok(())
    }

    /// [`call_request`] ensures that a [`CallRequest`]
    /// is well-formatted.
    pub(crate) fn call_request(&self, request: &CallRequest) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        self.valid_call_method(&request.method)
    }

    /// [`account_coins_request`] ensures that a [`AccountCoinsRequest`]
    /// is well-formatted.
    pub(crate) fn account_coins_request(&self, request: &AccountCoinsRequest) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
        account_identifier(Some(&request.account_identifier))?;
        if request.include_mempool && !self.mempool_coins {
            Err(ServerError::MempoolCoinsNotSupported)?
        } else if let Some(c) =
            contains_duplicate_currency(request.currencies.as_ref().unwrap_or(&Vec::new()))
        {
            Err(format!("{}: {c:?}", ServerError::DuplicateCurrency))?
        } else {
            Ok(())
        }
    }

    /// [`events_block_request`] ensures that a [`EventsBlocksRequest`]
    /// is well-formatted.
    pub(crate) fn events_block_request(&self, request: &EventsBlocksRequest) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil
        self.valid_supported_network(&request.network_identifier)?;
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
    pub(crate) fn search_transactions_request(
        &self,
        request: &SearchTransactionsRequest,
    ) -> AssertResult<()> {
        // TODO if self == nil
        // TODO if request == nil

        self.valid_supported_network(&request.network_identifier)?;
        todo!("impossible cases");
        // if let Some(op) = request.operator {
        //     match op {
        //         crate::models::Operator::Or => Ok(()),
        //         crate::models::Operator::And => Ok(()),
        //     }
        // }
        // if matches!(request.max_block, Some(i) if i < 0) {
        //     Err(ServerError::MaxBlockInvalid)?
        // } else if matches!(request.offset, Some(i) if i < 0) {
        //     Err(ServerError::OffsetIsNegative)?
        // } else if matches!(request.limit, Some(i) if i < 0) {
        //     Err(ServerError::LimitIsNegative)?
        // }

        if let Some(id) = &request.transaction_identifier {
            transaction_identifier(id)?;
        }

        if let Some(id) = &request.account_identifier {
            account_identifier(Some(id))?;
        }

        if let Some(id) = &request.coin_identifier {
            coin_identifier(id)?;
        }

        if let Some(c) = &request.currency {
            currency(c)?;
        }

        if let Some(s) = &request.status {
            // self.operation_status(s, false)?;
        }

        if let Some(t) = &request.type_ {
            // self.operation_type(t, false)?;
        }

        if matches!(request.address, Some(a) if a.is_empty()) {
            Err(BlockError::AccountAddrMissing)?
        } else {
            Ok(())
        }
    }
}
