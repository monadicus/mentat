//! Validates that block data is correct.

use std::str::FromStr;

use num_bigint_dig::{BigInt, Sign};

use super::*;

/// `currency` ensures a [`Currency`] is valid.
pub fn currency(currency: Option<&UncheckedCurrency>) -> AssertResult<()> {
    let currency = currency.ok_or(BlockError::AmountCurrencyIsNil)?;
    if currency.symbol.is_empty() {
        Err(BlockError::AmountCurrencySymbolEmpty)?
    } else if currency.decimals < 0 {
        Err(BlockError::AmountCurrencyHasNegDecimals)?
    } else {
        Ok(())
    }
}

/// `amount` ensures a [`Amount`] has an
/// integer value, specified precision, and symbol.
pub fn amount(amount: Option<&UncheckedAmount>) -> AssertResult<()> {
    let amount = amount.ok_or(BlockError::AmountValueMissing)?;

    if amount.value.is_empty() {
        Err(BlockError::AmountValueMissing)?
    } else if BigInt::from_str(&amount.value).is_err() {
        Err(BlockError::AmountIsNotInt)?
    } else {
        currency(amount.currency.as_ref())
    }
}

/// `operation_identifier` returns an error if index of the
/// [`UncheckedOperationIdentifier`] is out-of-order or if the NetworkIndex is
/// invalid.
pub fn operation_identifier(
    ident: Option<&UncheckedOperationIdentifier>,
    index: isize,
) -> AssertResult<()> {
    let ident = ident.ok_or(BlockError::OperationIdentifierIndexIsNil)?;

    if ident.index != index {
        Err(format!(
            "expected identifier index {index} but got {}: {}",
            ident.index,
            BlockError::OperationIdentifierIndexOutOfOrder,
        ))?
    } else if matches!(ident.network_index, Some(i) if i < 0) {
        Err(BlockError::OperationIdentifierNetworkIndexInvalid)?
    } else {
        Ok(())
    }
}

/// `account_identifier` returns an error if a [`AccountIdentifier`]
/// is missing an address or a provided SubAccount is missing an identifier.
pub fn account_identifier(account: Option<&AccountIdentifier>) -> AssertResult<()> {
    let account = account.ok_or(BlockError::AccountIsNil)?;

    if account.address.is_empty() {
        Err(BlockError::AccountAddrMissing)?
    } else if account.sub_account.is_none() {
        Ok(())
    } else if matches!(&account.sub_account, Some(acct) if acct.address.is_empty()) {
        Err(BlockError::AccountSubAccountAddrMissing)?
    } else {
        Ok(())
    }
}

impl Asserter {
    /// `operation_status` returns an error if an operation.Status
    /// is not valid.
    pub fn operation_status(
        &self,
        status: Option<&String>,
        construction: bool,
    ) -> AssertResult<()> {
        if self.response.is_none() && self.request.is_none() {
            Err(AsserterError::NotInitialized)?;
        }

        if status.is_none() || status.unwrap().is_empty() {
            return if construction {
                Ok(())
            } else {
                Err(BlockError::OperationStatusMissing)?
            };
        }

        let status = status.unwrap();

        if construction {
            Err(BlockError::OperationStatusNotEmptyForConstruction)?
        }

        if self
            .response
            .as_ref()
            .and_then(|r| r.operation_status_map.get(status))
            .is_none()
        {
            Err(format!(
                "operation status {status} is invalid: {}",
                BlockError::OperationStatusInvalid,
            ))?
        } else {
            Ok(())
        }
    }

    /// `operation_type` returns an error if an operation.Type
    /// is not valid.
    pub fn operation_type(&self, t: String) -> AssertResult<()> {
        if self.response.is_none() && self.request.is_none() {
            Err(AsserterError::NotInitialized)?;
        }

        if t.is_empty() || !self.operation_types.contains(&t) {
            Err(format!(
                "operation type {t} is invalid: {}",
                BlockError::OperationTypeInvalid
            ))?
        } else {
            Ok(())
        }
    }

    /// `operation` ensures a [`TypesOperation`] has a valid
    /// type, status, and amount.
    pub fn operation(
        &self,
        operation: Option<&UncheckedOperation>,
        index: isize,
        construction: bool,
    ) -> AssertResult<()> {
        if self.response.is_none() && self.request.is_none() {
            Err(AsserterError::NotInitialized)?;
        }

        let operation = operation.ok_or(BlockError::OperationIsNil)?;

        operation_identifier(operation.operation_identifier.as_ref(), index).map_err(|e| {
            format!(
                "operation identifier {:?} is invalid in operation {index}: {e}",
                operation.operation_identifier
            )
        })?;

        self.operation_type(operation.type_.clone()).map_err(|e| {
            format!(
                "operation type {:?} is invalid in operation {index}: {e}",
                operation.type_
            )
        })?;

        self.operation_status(operation.status.as_ref(), construction)
            .map_err(|e| {
                format!(
                    "operation status {:?} is invalid in operation {index}: {e}",
                    operation.status
                )
            })?;

        if operation.amount.is_none() {
            return Ok(());
        }

        account_identifier(operation.account.as_ref()).map_err(|e| {
            format!(
                "operation account identifier {:?} is invalid in operation {index}: {e}",
                operation.account
            )
        })?;

        amount(operation.amount.as_ref()).map_err(|e| {
            format!(
                "operation amount {:?} is invalid in operation {index}: {e}",
                operation.amount
            )
        })?;

        if operation.coin_change.is_none() {
            return Ok(());
        }

        coin_change(operation.coin_change.as_ref()).map_err(|e| {
            format!(
                "operation coin change {:?} is invalid in operation {index}: {e}",
                operation.coin_change
            )
        })?;

        Ok(())
    }

    /// `operations` returns an error if any [`TypesOperation`]
    /// in a [`TypesOperation`] is invalid.
    pub fn operations(
        &self,
        operations: &[Option<UncheckedOperation>],
        construction: bool,
    ) -> AssertResult<()> {
        if operations.is_empty() && construction {
            Err(BlockError::NoOperationsForConstruction)?;
        }

        let mut payment_total = BigInt::from(0u8);
        let mut fee_total = BigInt::from(0u8);
        let mut payment_count = 0;
        let mut fee_count = 0;
        let mut related_ops_exist = false;

        for (index, op) in operations.iter().enumerate() {
            self.operation(op.as_ref(), index as isize, construction)
                .map_err(|e| format!("operation {op:?} is invalid: {e}"))?;
            let op = op.as_ref().unwrap();
            if self.validations.enabled {
                if op.type_ == self.validations.payment.name {
                    let val = BigInt::from_str(&op.amount.as_ref().unwrap().value).unwrap();
                    payment_total += val;
                    payment_count += 1;
                }

                if op.type_ == self.validations.fee.name {
                    if !op.related_operations.is_empty() {
                        Err(format!(
                            "operation {op:?} is invalid with operation index {index}: {}",
                            BlockError::RelatedOperationInFeeNotAllowed
                        ))?;
                    }

                    let val = BigInt::from_str(&op.amount.as_ref().unwrap().value)
                        .map_err(|err| err.to_string())?;

                    if !matches!(val.sign(), Sign::Minus) {
                        Err(format!(
                            "operation {op:?} is invalid with operation index {index}: {}",
                            BlockError::FeeAmountNotNegative
                        ))?;
                    }

                    fee_total += val;
                    fee_count += 1;
                }
            }

            // Ensure an operation's related_operations are only
            // operations with an index less than the operation
            // and that there are no duplicates.
            let operation_identifier_index = op.operation_identifier.as_ref().unwrap().index;
            let mut related_indexes = IndexSet::new();

            for related_op in op.related_operations.iter().flat_map(|i| i.as_ref()) {
                related_ops_exist = true;

                if related_op.index >= operation_identifier_index {
                    Err(format!(
                        "related operation index {} >= operation index {}: {}",
                        related_op.index,
                        operation_identifier_index,
                        BlockError::RelatedOperationIndexOutOfOrder,
                    ))?;
                }

                if related_indexes.contains(&related_op.index) {
                    Err(format!(
                        "related operation index {} found for operation index {}: {}",
                        related_op.index,
                        operation_identifier_index,
                        BlockError::RelatedOperationIndexDuplicate,
                    ))?;
                }

                related_indexes.insert(related_op.index);
            }
        }

        // throw an error if relatedOps is not implemented and relatedOps is supported
        if !related_ops_exist && self.validations.enabled && self.validations.related_ops_exists {
            Err(BlockError::RelatedOperationMissing)?;
        }

        if self.validations.enabled && self.validations.chain_type == super::asserter_tools::ACCOUNT
        {
            // only account based validation
            self.validate_payment_and_fee(payment_total, payment_count, fee_total, fee_count)?;
        }

        Ok(())
    }

    /// `validate_payment_and_fee`validates payments and fees.
    pub fn validate_payment_and_fee(
        &self,
        payment_total: BigInt,
        payment_count: isize,
        fee_total: BigInt,
        fee_count: isize,
    ) -> AssertResult<()> {
        let zero = BigInt::from(0u8);
        if self.validations.payment.operation.count != -1
            && self.validations.payment.operation.count != payment_count
        {
            Err(BlockError::PaymentCountMismatch)?
        }

        if self.validations.payment.operation.should_balance && payment_total != zero {
            Err(BlockError::PaymentAmountNotBalancing)?
        }

        if self.validations.fee.operation.count != -1
            && self.validations.fee.operation.count != fee_count
        {
            Err(BlockError::FeeCountMismatch)?
        }

        if self.validations.fee.operation.should_balance && fee_total != zero {
            Err(BlockError::FeeAmountNotBalancing)?
        } else {
            Ok(())
        }
    }

    /// `transaction` returns an error if the [`TransactionIdentifier`]
    /// is invalid, if any [`TypesOperation`] within the [`Transaction`]
    /// is invalid, or if any operation index is reused within a transaction.
    pub fn transaction(&self, transaction: Option<&UncheckedTransaction>) -> AssertResult<()> {
        self.response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        let transaction = transaction.ok_or(BlockError::TxIsNil)?;

        transaction_identifier(transaction.transaction_identifier.as_ref()).map_err(|e| {
            format!(
                "transaction identifier {:?} is invalid: {e}",
                transaction.transaction_identifier
            )
        })?;
        let transaction_identifier = transaction.transaction_identifier.as_ref().unwrap();

        self.operations(&transaction.operations, false)
            .map_err(|e| {
                format!(
                    "invalid operation in transaction operations {}: {e}",
                    transaction_identifier.hash
                )
            })?;

        self.related_transactions(&transaction.related_transactions)
            .map_err(|e| {
                format!(
                    "invalid related transaction in related transactions {:?}: {e}",
                    transaction.related_transactions
                )
            })?;

        Ok(())
    }

    /// `related_transactions` returns an error if the array of
    /// [`RelatedTransaction`] is non-null and non-empty and any of the
    /// related transactions contain invalid types, invalid network
    /// identifiers, invalid transaction identifiers, or a direction not
    /// defined by the enum.
    pub fn related_transactions(
        &self,
        related_transactions: &[Option<UncheckedRelatedTransaction>],
    ) -> AssertResult<()> {
        if let Some(dup) = duplicate_related_transaction(related_transactions) {
            Err(format!(
                "{}: {dup:?}",
                BlockError::DuplicateRelatedTransaction
            ))?;
        }

        for (i, related) in related_transactions
            .iter()
            .filter_map(|i| i.as_ref())
            .enumerate()
        {
            network_identifier(related.network_identifier.as_ref()).map_err(|e| {
                format!(
                    "network identifier {:?} is invalid in related transaction at index {i}: {e}",
                    related.network_identifier
                )
            })?;

            transaction_identifier(related.transaction_identifier.as_ref()).map_err(|e| {
                format!(
                    "invalid transaction identifier {:?} in related transaction at index {i}: {e}",
                    related.transaction_identifier
                )
            })?;

            self.direction(&related.direction).map_err(|e| {
                format!(
                    "invalid direction {:?} in related transaction at index {i}: {e}",
                    related.direction
                )
            })?;
        }
        Ok(())
    }

    /// `direction` returns an error if the value passed is not
    /// [Direction::Forward] or [Direction::Backward]
    pub fn direction(&self, direction: &UncheckedDirection) -> AssertResult<()> {
        if !direction.valid() {
            Err(BlockError::InvalidDirection)?
        } else {
            Ok(())
        }
    }

    /// `block` runs a basic set of assertions for each returned [`Block`].
    pub fn block(&self, block: Option<&UncheckedBlock>) -> AssertResult<()> {
        let asserter = self
            .response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;
        let block = block.ok_or(BlockError::BlockIsNil)?;

        block_identifier(block.block_identifier.as_ref()).map_err(|e| {
            format!(
                "block identifier {:?} is invalid: {e}",
                block.block_identifier,
            )
        })?;
        block_identifier(block.parent_block_identifier.as_ref()).map_err(|e| {
            format!(
                "parent block identifier {:?} is invalid: {e}",
                block.parent_block_identifier,
            )
        })?;
        let block_identifier = block.block_identifier.as_ref().unwrap();
        let parent_block_identifier = block.parent_block_identifier.as_ref().unwrap();

        // Only apply duplicate hash and index checks if the block index is not the
        // genesis index.
        if asserter.genesis_block.index != block_identifier.index {
            if block_identifier.hash == parent_block_identifier.hash {
                Err(BlockError::BlockHashEqualsParentBlockHash)?;
            } else if block_identifier.index <= parent_block_identifier.index {
                Err(BlockError::BlockIndexPrecedesParentBlockIndex)?;
            }
        }

        // Only check for timestamp validity if timestamp start index is <=
        // the current block index.
        if asserter.timestamp_start_index as isize <= block_identifier.index {
            timestamp(block.timestamp)
                .map_err(|e| format!("timestamp {} is invalid: {e}", block.timestamp))?;
        }

        block.transactions.iter().try_for_each(|transaction| {
            self.transaction(transaction.as_ref())
                .map_err(|e| format!("transaction {transaction:?} is invalid: {e}").into())
        })
    }
}

/// `block_identifier` ensures a [`UncheckedBlockIdentifier`]
/// is well-formatted.
pub fn block_identifier(block: Option<&UncheckedBlockIdentifier>) -> AssertResult<()> {
    let block = block.ok_or(BlockError::BlockIdentifierIsNil)?;
    if block.hash.is_empty() {
        Err(BlockError::BlockIdentifierHashMissing)?
    } else if block.index < 0 {
        Err(BlockError::BlockIdentifierIndexIsNeg)?
    } else {
        Ok(())
    }
}

/// `partial_block_identifier` ensures a [`UncheckedPartialBlockIdentifier`]
/// is well-formatted.
pub fn partial_block_identifier(
    block_identifier: Option<&UncheckedPartialBlockIdentifier>,
) -> AssertResult<()> {
    let block_identifier = block_identifier.ok_or(BlockError::PartialBlockIdentifierIsNil)?;
    if matches!(&block_identifier.hash, Some(hash) if !hash.is_empty())
        || matches!(block_identifier.index, Some(index) if index >= 0)
    {
        Ok(())
    } else {
        Err(BlockError::PartialBlockIdentifierFieldsNotSet.into())
    }
}

/// `duplicate_related_transaction` returns nil if no duplicates are found in
/// the array and returns the first duplicated item found otherwise.
pub fn duplicate_related_transaction(
    items: &[Option<UncheckedRelatedTransaction>],
) -> Option<&UncheckedRelatedTransaction> {
    let mut seen = IndexSet::new();

    for item in items {
        let key = hash(item.as_ref());

        if seen.contains(&key) {
            return item.as_ref();
        }

        seen.insert(key);
    }

    None
}

/// `transaction_identifier` returns an error if a
/// [`TransactionIdentifier`] has an invalid hash.
pub fn transaction_identifier(ident: Option<&TransactionIdentifier>) -> AssertResult<()> {
    let ident = ident.ok_or(BlockError::TxIdentifierIsNil)?;
    if ident.hash.is_empty() {
        Err(BlockError::TxIdentifierHashMissing.into())
    } else {
        Ok(())
    }
}

/// The min unix epoch
pub static MIN_UNIX_EPOCH: isize = 946713600000;
/// The max unix epoch
pub static MAX_UNIX_EPOCH: isize = 2209017600000;

/// `timestamp` returns an error if the timestamp
/// on a block is less than or equal to 0.
pub fn timestamp(timestamp: isize) -> Result<(), BlockError> {
    if timestamp < MIN_UNIX_EPOCH {
        Err(BlockError::TimestampBeforeMin)
    } else if timestamp > MAX_UNIX_EPOCH {
        Err(BlockError::TimestampAfterMax)
    } else {
        Ok(())
    }
}
