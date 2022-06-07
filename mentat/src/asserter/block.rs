use std::str::FromStr;

use indexmap::IndexSet;
use num_bigint_dig::{BigInt, Sign};

use super::{
    asserter::ResponseAsserter,
    coin::coin_change,
    errors::{AssertResult, BlockError},
    network::network_identifier,
    util::hash,
};
use crate::{
    identifiers::{
        AccountIdentifier,
        BlockIdentifier,
        OperationIdentifier,
        PartialBlockIdentifier,
        TransactionIdentifier,
    },
    models::{Amount, Block, Currency, Direction, Operation, RelatedTransaction, Transaction},
};

/// `currency` ensures a [`Currency`] is valid.
pub(crate) fn currency(currency: &Currency) -> AssertResult<()> {
    //TODO if currency nil
    if currency.symbol.is_empty() {
        Err(BlockError::AmountCurrencySymbolEmpty)?;
    }

    // we use a usize this error doesn't apply?
    if currency.decimals < 0 {
        Err(BlockError::AmountCurrencyHasNegDecimals)?;
    }

    Ok(())
}

/// `amount` ensures a [`Amount`] has an
/// integer value, specified precision, and symbol.
pub(crate) fn amount(amount: Option<&Amount>) -> AssertResult<()> {
    // or if currency nil
    let amount = amount.ok_or(BlockError::AmountValueMissing)?;

    if amount.value.is_empty() {
        Err(BlockError::AmountValueMissing)?;
    }

    if BigInt::from_str(&amount.value).is_err() {
        Err(format!("{}: {}", BlockError::AmountIsNotInt, amount.value))?;
    }

    currency(&amount.currency)
}

/// `operation_identifier` returns an error if index of the
/// [`OperationIdentifier`] is out-of-order or if the NetworkIndex is
/// invalid.
pub(crate) fn operation_identifier(ident: &OperationIdentifier, index: i64) -> AssertResult<()> {
    // TODO if ident nil
    if ident.index as i64 != index {
        return Err(format!(
            "{}: expected {index} but got {}",
            BlockError::OperationIdentifierIndexOutOfOrder,
            ident.index
        )
        .into());
    }

    if ident.network_index.is_some() && ident.network_index.unwrap() < 0 {
        Err(BlockError::OperationIdentifierNetworkIndexInvalid)?;
    }

    Ok(())
}

/// `account_identifier` returns an error if a [`AccountIdentifier`]
/// is missing an address or a provided SubAccount is missing an identifier.
pub(crate) fn account_identifier(account: Option<&AccountIdentifier>) -> AssertResult<()> {
    // TODO if account nil
    let account = account.ok_or(BlockError::AccountIsNil)?;

    if account.address.is_empty() {
        Err(BlockError::AccountIsNil)?;
    }

    if account.sub_account.is_none() {
        return Ok(());
    }

    if account.sub_account.as_ref().unwrap().address.is_empty() {
        Err(BlockError::AccountSubAccountAddrMissing)?;
    }
    Ok(())
}

/// `contains` checks if a value is contained in a slice
/// of strings.
pub(crate) fn contains<T: Eq>(valid: &[T], value: &T) -> bool {
    valid.iter().any(|v| v == value)
}

impl ResponseAsserter {
    /// `operation_status` returns an error if an operation.Status
    /// is not valid.
    pub(crate) fn operation_status(
        &self,
        status: Option<&String>,
        construction: bool,
    ) -> AssertResult<()> {
        // TODO if self nil bruh
        if status.is_none() || status.unwrap().is_empty() {
            if construction {
                return Ok(());
            }

            Err(BlockError::OperationStatusMissing)?;
        }

        if construction {
            Err(BlockError::OperationStatusNotEmptyForConstruction)?;
        }

        if !self.operation_status_map[status.unwrap()] {
            return Err(format!(
                "{}: {}",
                BlockError::OperationStatusInvalid,
                status.unwrap()
            )
            .into());
        }

        Ok(())
    }

    /// `operation_type` returns an error if an operation.Type
    /// is not valid.
    pub(crate) fn operation_type(&self, t: String) -> AssertResult<()> {
        // TODO if self nil bruh
        if t.is_empty() || contains(&self.operation_types, &t) {
            Err(format!("{}: {t}", BlockError::OperationTypeInvalid))?;
        }

        Ok(())
    }

    /// `operation` ensures a [`Operation`] has a valid
    /// type, status, and amount.
    pub(crate) fn operation(
        &self,
        operation: Option<&Operation>,
        index: i64,
        construction: bool,
    ) -> AssertResult<()> {
        // TODO if self nil bruh
        let operation = operation.ok_or(BlockError::OperationIsNil)?;

        operation_identifier(&operation.operation_identifier, index).map_err(|err| {
            format!("{err}: Operation identifier is invalid in operation {index}")
        })?;

        self.operation_type(operation.type_.clone())
            .map_err(|err| format!("{err}: operation type is invalid in operation {index}"))?;

        self.operation_status(operation.status.as_ref(), construction)
            .map_err(|err| format!("{err}: operation type is invalid in operation {index}"))?;

        if operation.amount.is_none() {
            return Ok(());
        }

        account_identifier(operation.account.as_ref())
            .map_err(|err| format!("{err}: account identifier is invalid in operation {index}"))?;

        amount(operation.amount.as_ref())
            .map_err(|err| format!("{err}: amount is invalid in operation {index}"))?;

        if operation.coin_change.is_none() {
            return Ok(());
        }

        coin_change(operation.coin_change.as_ref())
            .map_err(|err| format!("{err}: coin change is invalid in operation {index}"))?;

        Ok(())
    }

    /// `operations` returns an error if any [`Operation`]
    /// in a [`Operation`] is invalid.
    pub(crate) fn operations(
        &self,
        operations: &[Operation],
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
            self.operation(Some(op), index as i64, construction)?;
            if self.validations.enabled {
                if op.type_ == self.validations.payment.name {
                    let val = BigInt::from_str(&op.amount.as_ref().unwrap().value).unwrap();
                    payment_total += val;
                    payment_count += 1;
                }

                if op.type_ == self.validations.fee.name {
                    if op.related_operations.is_some() {
                        Err(format!(
                            "{}: operation index {index}",
                            BlockError::RelatedOperationInFeeNotAllowed
                        ))?;
                    }

                    let val = BigInt::from_str(&op.amount.as_ref().unwrap().value)
                        .map_err(|err| err.to_string())?;

                    if !matches!(val.sign(), Sign::Minus) {
                        Err(format!(
                            "{}: operation index {index}",
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
            let mut related_indexes = IndexSet::new();

            for related_op in op.related_operations.iter().flatten() {
                related_ops_exist = true;

                if related_op.index >= op.operation_identifier.index {
                    Err(format!(
                        "{}: related operation index {} >= operation index {}",
                        BlockError::RelatedOperationIndexOutOfOrder,
                        related_op.index,
                        op.operation_identifier.index
                    ))?;
                }

                if related_indexes.contains(&related_op.index) {
                    Err(format!(
                        "{}: related operation index {} found for operation index {}",
                        BlockError::RelatedOperationIndexDuplicate,
                        related_op.index,
                        op.operation_identifier.index
                    ))?;
                }

                related_indexes.insert(related_op.index);
            }
        }

        // throw an error if relatedOps is not implemented and relatedOps is supported
        if !related_ops_exist && self.validations.enabled && self.validations.related_ops_exists {
            Err(BlockError::RelatedOperationMissing)?;
        }

        // only account based validation
        if self.validations.enabled && self.validations.chain_type == super::asserter::ACCOUNT {
            self.validate_payment_and_fee(payment_total, payment_count, fee_total, fee_count)?;
        }

        Ok(())
    }

    /// `validate_payment_and_fee`validates payments and fees.
    pub(crate) fn validate_payment_and_fee(
        &self,
        payment_total: BigInt,
        payment_count: i64,
        fee_total: BigInt,
        fee_count: i64,
    ) -> AssertResult<()> {
        let zero = BigInt::from(0u8);
        if self.validations.payment.operation.count != -1
            && self.validations.payment.operation.count != payment_count
        {
            Err(BlockError::PaymentCountMismatch)?;
        }

        if self.validations.payment.operation.should_balance && payment_total != zero {
            Err(BlockError::PaymentAmountNotBalancing)?;
        }

        if self.validations.fee.operation.count != -1
            && self.validations.payment.operation.count != fee_count
        {
            Err(BlockError::FeeCountMismatch)?;
        }

        if self.validations.fee.operation.should_balance && fee_total != zero {
            Err(BlockError::FeeAmountNotBalancing)?;
        }

        Ok(())
    }

    /// `transaction` returns an error if the [`TransactionIdentifier`]
    /// is invalid, if any [`Operation`] within the [`Transaction`]
    /// is invalid, or if any operation index is reused within a transaction.
    pub(crate) fn transaction(&self, transaction: &Transaction) -> AssertResult<()> {
        // TODO if self nil bruh
        // TODO if transaction nil
        transaction_identifier(&transaction.transaction_identifier)?;

        self.operations(&transaction.operations, false)
            .map_err(|err| {
                format!(
                    "{err} invalid operation in transaction {}",
                    transaction.transaction_identifier.hash
                )
            })?;

        self.related_transactions(transaction.related_transactions.as_ref())
            .map_err(|err| {
                format!(
                    "{err} invalid related transaction in transaction {}",
                    transaction.transaction_identifier.hash
                )
            })?;

        Ok(())
    }

    /// `related_transactions` returns an error if the array of
    /// [`RelatedTransaction`] is non-null and non-empty and any of the
    /// related transactions contain invalid types, invalid network
    /// identifiers, invalid transaction identifiers, or a direction not
    /// defined by the enum.
    pub(crate) fn related_transactions(
        &self,
        related_transactions: &[RelatedTransaction],
    ) -> AssertResult<()> {
        if let Some(dup) = duplicate_related_transaction(related_transactions) {
            Err(format!(
                "{}: {dup:?}",
                BlockError::DuplicateRelatedTransaction
            ))?;
        }

        for (index, related) in related_transactions.iter().enumerate() {
            if let Some(network_ident) = related.network_identifier.as_ref() {
                network_identifier(network_ident).map_err(|err| {
                    format!(
                        "{err} invalid network identifier in related transaction at index {index}"
                    )
                })?;
            }

            transaction_identifier(&related.transaction_identifier).map_err(|err| {
                format!(
                    "{err} invalid transaction identifier in related transaction at index {index}"
                )
            })?;

            self.direction(&related.direction).map_err(|err| {
                format!("{err} invalid direction in related transaction at index {index}")
            })?;
        }
        Ok(())
    }

    /// `direction` returns an error if the value passed is not
    /// [Direction::Forward] or [Direction::Backward]
    pub(crate) fn direction(&self, _: &Direction) -> AssertResult<()> {
        // TODO We only support those two values
        Ok(())
    }

    /// `block` runs a basic set of assertions for each returned [`Block`].
    pub(crate) fn block(&self, block: &Block) -> AssertResult<()> {
        // TODO if self nil bruh
        // TODO if block nil
        block_identifier(&block.block_identifier)?;
        block_identifier(&block.parent_block_identifier)?;

        // Only apply duplicate hash and index checks if the block index is not the
        // genesis index.
        if self.genesis_block.index != block.block_identifier.index {
            if block.block_identifier.hash == block.parent_block_identifier.hash {
                Err(BlockError::BlockHashEqualsParentBlockHash)?;
            }

            if block.block_identifier.index <= block.parent_block_identifier.index {
                Err(BlockError::BlockIndexPrecedesParentBlockIndex)?;
            }
        }

        // Only check for timestamp validity if timestamp start index is <=
        // the current block index.
        if self.timestamp_start_index <= block.block_identifier.index as i64 {
            timestamp(block.timestamp as i64)?;
        }

        block
            .transactions
            .iter()
            .try_for_each(|transaction| self.transaction(transaction))
    }
}

/// `block_identifier` ensures a [`BlockIdentifier`]
/// is well-formatted.
pub(crate) fn block_identifier(block: &BlockIdentifier) -> AssertResult<()> {
    // TODO if block nil
    if block.hash.is_empty() {
        Err(BlockError::BlockIdentifierHashMissing)?;
    }

    if block.index < 0 {
        Err(BlockError::BlockIdentifierIndexIsNeg)?;
    }

    Ok(())
}

// Transaction returns an error if the types.TransactionIdentifier
// is invalid, if any types.Operation within the types.Transaction
// is invalid, or if any operation index is reused within a transaction.

/// `partial_block_identifier` ensures a [`PartialBlockIdentifier`]
/// is well-formatted.
pub(crate) fn partial_block_identifier(
    block_identifier: &PartialBlockIdentifier,
) -> AssertResult<()> {
    // TODO if block_identifier nil
    if block_identifier.hash.is_some() && !block_identifier.hash.as_ref().unwrap().is_empty() {
        return Ok(());
    }

    if block_identifier.index.is_some() && block_identifier.index.unwrap() >= 0 {
        return Ok(());
    }

    Err(BlockError::PartialBlockIdentifierFieldsNotSet.into())
}

/// `duplicate_related_transaction` returns nil if no duplicates are found in
/// the array and returns the first duplicated item found otherwise.
pub(crate) fn duplicate_related_transaction(
    items: &[RelatedTransaction],
) -> Option<&RelatedTransaction> {
    let mut seen = IndexSet::new();

    for item in items {
        let key = hash(item);

        if seen.contains(&key) {
            return Some(item);
        }

        seen.insert(key);
    }

    None
}

/// `transaction_identifier` returns an error if a
/// [`TransactionIdentifier`] has an invalid hash.
pub(crate) fn transaction_identifier(ident: &TransactionIdentifier) -> AssertResult<()> {
    // TODO if ident nil
    if ident.hash.is_empty() {
        Err(BlockError::TxIdentifierHashMissing)?;
    }

    Ok(())
}

/// The min unix epoch
static MIN_UNIX_EPOCH: i64 = 946713600000;
/// The max unix epoch
static MAX_UNIX_EPOCH: i64 = 2209017600000;

/// `timestamp` returns an error if the timestamp
/// on a block is less than or equal to 0.
pub(crate) fn timestamp(timestamp: i64) -> Result<(), String> {
    if timestamp < MIN_UNIX_EPOCH {
        Err(format!("{}: {timestamp}", BlockError::TimestampBeforeMin))
    } else if timestamp > MAX_UNIX_EPOCH {
        Err(format!("{}: {timestamp}", BlockError::TimestampAfterMax))
    } else {
        Ok(())
    }
}
