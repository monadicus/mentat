use super::{
    asserter::ResponseAsserter,
    coins::coin_change,
    errors::{AssertResult, BlockError},
};
use crate::{
    identifiers::{
        AccountIdentifier,
        BlockIdentifier,
        OperationIdentifier,
        PartialBlockIdentifier,
        TransactionIdentifier,
    },
    models::{Amount, Currency, Operation},
};

/// `currency` ensures a [`Currency`] is valid.
pub(crate) fn currency(currency: &Currency) -> AssertResult<()> {
    //TODO if currency nil
    if currency.symbol.is_empty() {
        return Err(BlockError::AmountCurrencySymbolEmpty.into());
    }

    // we use a usize this error doesn't apply?
    if currency.decimals < 0 {
        return Err(BlockError::AmountCurrencyHasNegDecimals.into());
    }

    Ok(())
}

/// `amount` ensures a [`Amount`] has an
/// integer value, specified precision, and symbol.
pub(crate) fn amount(amount: Option<&Amount>) -> AssertResult<()> {
    // or if currency nil
    let amount = amount.ok_or_else(|| BlockError::AmountValueMissing)?;

    if amount.value.is_empty() {
        return Err(BlockError::AmountValueMissing.into());
    }

    if amount.value.parse::<i128>().is_err() {
        return Err(format!("{}: {}", BlockError::AmountIsNotInt, amount.value).into());
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
        return Err(BlockError::OperationIdentifierNetworkIndexInvalid.into());
    }

    Ok(())
}

/// `account_identifier` returns an error if a [`AccountIdentifier`]
/// is missing an address or a provided SubAccount is missing an identifier.
pub(crate) fn account_identifier(account: Option<&AccountIdentifier>) -> AssertResult<()> {
    // TODO if account nil
    let account = account.ok_or_else(|| BlockError::AccountIsNil)?;

    if account.address.is_empty() {
        return Err(BlockError::AccountIsNil.into());
    }

    if account.sub_account.is_none() {
        return Ok(());
    }

    if account.sub_account.unwrap().address.is_empty() {
        return Err(BlockError::AccountSubAccountAddrMissing.into());
    }
    Ok(())
}

/// `contains` checks if a value is contained in a slice
/// of strings.
pub(crate) fn contains<T: Eq>(valid: &[T], value: T) -> bool {
    valid.iter().any(|v| v == &value)
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

            return Err(BlockError::OperationStatusMissing.into());
        }

        if construction {
            return Err(BlockError::OperationStatusNotEmptyForConstruction.into());
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
        if t.is_empty() || contains(&self.operation_types, t) {
            return Err(format!("{}: {t}", BlockError::OperationTypeInvalid).into());
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
        let operation = operation.ok_or_else(|| BlockError::OperationIsNil)?;

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
            return Err(BlockError::NoOperationsForConstruction.into());
        }

        for (index, op) in operations.iter().enumerate() {
            self.operation(Some(op), index as i64, construction)?;
            todo!()
        }
        Ok(())
    }
}

/// `block_identifier` ensures a [`BlockIdentifier`]
/// is well-formatted.
pub(crate) fn block_identifier(block: &BlockIdentifier) -> AssertResult<()> {
    // TODO if block nil
    if block.hash.is_empty() {
        return Err(BlockError::BlockIdentifierHashMissing.into());
    }

    if block.index < 0 {
        return Err(BlockError::BlockIdentifierIndexIsNeg.into());
    }

    Ok(())
}

/// `partial_block_identifier` ensures a [`PartialBlockIdentifier`]
/// is well-formatted.
pub(crate) fn partial_block_identifier(
    block_identifier: &PartialBlockIdentifier,
) -> AssertResult<()> {
    // TODO if block_identifier nil
    if block_identifier.hash.is_some() && !block_identifier.hash.unwrap().is_empty() {
        return Ok(());
    }

    if block_identifier.index.is_some() && block_identifier.index.unwrap() >= 0 {
        return Ok(());
    }

    Err(BlockError::PartialBlockIdentifierFieldsNotSet.into())
}

/// `transaction_identifier` returns an error if a
/// [`TransactionIdentifier`] has an invalid hash.
pub(crate) fn transaction_identifier(ident: &TransactionIdentifier) -> AssertResult<()> {
    // TODO if ident nil
    if ident.hash.is_empty() {
        return Err(BlockError::TxIdentifierHashMissing.into());
    }

    Ok(())
}

// /// `operation_identifier` returns an error if index of the
// /// [`OperationIdentifier`] is out-of-order or if the NetworkIndex is
// /// invalid.
// pub(crate) fn operation_identifier(ident: &OperationIdentifier, index: i64)
// -> AssertResult<()> {     // TODO if ident nil
//     if blo
// }

static MIN_UNIX_EPOCH: i64 = 946713600000;
static MAX_UNIX_EPOCH: i64 = 2209017600000;

/// `timestamp` returns an error if the timestamp
/// on a block is less than or equal to 0.
pub(crate) fn timestamp(timestamp: i64) -> Result<(), String> {
    if timestamp < MIN_UNIX_EPOCH {
        Err(format!("{}: {timestamp}", todo!()))
    } else if timestamp > MAX_UNIX_EPOCH {
        Err(format!("{}: {timestamp}", todo!()))
    } else {
        Ok(())
    }
}
