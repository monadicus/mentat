//! TODO doc

use num_bigint_dig::{BigInt, Sign};
use serde_json::Value;

use super::*;
use crate::{MatchOperationsError, ParserError, ParserResult};

/// AmountSign is used to represent possible signedness
/// of an amount.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmountSign(pub usize);

impl AmountSign {
    /// `ANY` is a positive or negative amount.
    const ANY: AmountSign = AmountSign(0);
    /// `NEGATIVE` is a negative amount.
    const NEGATIVE: AmountSign = AmountSign(1);
    /// `NEGATIVE_OR_ZERO` is a positive or zero amount.
    const NEGATIVE_OR_ZERO: AmountSign = AmountSign(4);
    /// OPPOSITES_LENGTH is the only allowed number of
    /// operations to compare as opposites.
    const OPPOSITES_LENGTH: AmountSign = AmountSign(2);
    /// `POSITIVE` is a positive amount.
    const POSITIVE: AmountSign = AmountSign(2);
    /// `POSITIVE_OR_ZERO` is a positive or zero amount.
    const POSITIVE_OR_ZERO: AmountSign = AmountSign(3);

    /// match_ returns a boolean indicating if an [`Amount`]
    /// has an [`AmountSign`].
    pub fn match_(self, amount: Option<&NullableAmount>) -> bool {
        if self == Self::ANY {
            return true;
        }

        let numeric = if let Ok(v) = amount_value(amount) {
            v
        } else {
            return false;
        };

        (self == Self::NEGATIVE && numeric.sign() == Sign::Minus)
            || (self == Self::POSITIVE && numeric.sign() == Sign::Plus)
            || (self == Self::POSITIVE_OR_ZERO
                && (numeric.sign() == Sign::Plus || numeric.bits() == 0))
            || (self == Self::NEGATIVE_OR_ZERO
                && (numeric.sign() == Sign::Minus || numeric.bits() == 0))
    }

    /// string returns a description of an [`AmountSign`].
    pub fn string(self) -> &'static str {
        match self {
            Self::ANY => "any",
            Self::NEGATIVE => "negative",
            Self::POSITIVE => "positive",
            Self::POSITIVE_OR_ZERO => "positive or zero",
            Self::NEGATIVE_OR_ZERO => "negative or zero",
            _ => "invalid",
        }
    }
}

/// MetadataDescription is used to check if a `IndexMap<String, Value>`
/// has certain keys and values of a certain kind.
#[allow(clippy::missing_docs_in_private_items)]
pub struct MetadataDescription {
    pub key: String,
    pub value_kind: Value,
}

/// AccountDescription is used to describe a [`AccountIdentifier`].
#[allow(clippy::missing_docs_in_private_items)]
pub struct AccountDescription {
    pub exists: bool,
    pub sub_account_exists: bool,
    pub sub_account_address: String,
    pub sub_account_metadata_keys: Vec<Option<MetadataDescription>>,
}

/// AmountDescription is used to describe a [`Amount`].
#[allow(clippy::missing_docs_in_private_items)]
pub struct AmountDescription {
    pub exists: bool,
    pub sign: AmountSign,
    pub currency: Option<NullableCurrency>,
}

/// OperationDescription is used to describe a [`NullableOperation`].
#[allow(clippy::missing_docs_in_private_items)]
pub struct OperationDescription {
    pub account: Option<AccountDescription>,
    pub amount: Option<AmountDescription>,
    pub metadata: Vec<Option<MetadataDescription>>,
    /// `type_` is the [`Type`] that must match. If this is left empty,
    /// any type is considered a match.
    pub type_: String,
    /// `allow_repeats` indicates that multiple operations can be matched
    /// to a particular description.
    pub allow_repeats: bool,
    /// `Optional` indicates that not finding any operations that meet
    /// the description should not trigger an error.
    pub optional: bool,
    /// `coin_action` indicates that an operation should have a
    /// [`NullableCoinChange`] and that it should have the
    /// [`NullableCoinAction`]. If this is not populated,
    /// [`NullableCoinChange`] is not checked.
    pub coin_action: NullableCoinAction,
}

/// Descriptions contains a slice of [`OperationDescription`]s and
/// high-level requirements enforced across multiple [`Operations`].
#[allow(clippy::missing_docs_in_private_items)]
pub struct Descriptions {
    pub operation_descriptions: Vec<Option<OperationDescription>>,
    /// `equal_amounts` are specified using the operation indices of
    /// [`OperationDescription`]s to handle out of order matches.
    /// [`match_operations`] will error if all groups of operations aren't
    /// equal.
    pub equal_amounts: Vec<Vec<usize>>,
    /// `opposite_amounts` are specified using the operation indices of
    /// [`OperationDescription`]s to handle out of order matches.
    /// [`match_operations`] will error if all groups of operations aren't
    /// opposites.
    pub opposite_amounts: Vec<Vec<usize>>,
    /// `opposite_amounts` are specified using the operation indices of
    /// [`OperationDescription`]s to handle out of order matches.
    /// [`match_operations`] will error if all groups of operations aren't 0
    /// or opposites.
    pub opposite_or_zero_amounts: Vec<Vec<usize>>,
    /// `equal_addresses` are specified using the operation indices of
    /// [`OperationDescription`]s to handle out of order matches.
    /// [`match_operations`] will error if all groups of operations
    /// addresses aren't equal.
    pub equal_addresses: Vec<Vec<usize>>,
    /// `err_unmatched` indicates that an error should be returned
    /// if all operations cannot be matched to a description.
    pub err_unmatched: bool,
}

/// metadata_match returns an error if a `IndexMap<String, Value>` does not meet
/// a slice of [`MetadataDescription`].
pub fn metadata_match(
    reqs: &[Option<MetadataDescription>],
    metadata: &IndexMap<String, Value>,
) -> ParserResult<()> {
    if reqs.is_empty() {
        return Ok(());
    }

    for req in reqs {
        // TODO: coinbase never checks null here
        let req = req.as_ref().unwrap();
        let val = metadata.get(&req.key).ok_or(format!(
            "{}: {}",
            MatchOperationsError::MetadataMatchKeyNotFound,
            req.key
        ))?;

        match (val, &req.value_kind) {
            (Value::Null, Value::Null)
            | (Value::Bool(_), Value::Bool(_))
            | (Value::Number(_), Value::Number(_))
            | (Value::String(_), Value::String(_))
            | (Value::Array(_), Value::Array(_))
            | (Value::Object(_), Value::Object(_)) => {}
            _ => Err(format!(
                "{}: value of {} is not of type {}",
                MatchOperationsError::MetadataMatchKeyValueMismatch,
                req.key,
                req.value_kind
            ))?,
        }
    }

    Ok(())
}

/// account_match returns an error if a [`AccountIdentifier`] does not meet
/// an [`AccountDescription`].
pub fn account_match(
    req: Option<&AccountDescription>,
    account: Option<&AccountIdentifier>,
) -> ParserResult<()> {
    let req = if let Some(r) = req { r } else { return Ok(()) };
    let account = if let Some(account) = account {
        account
    } else if req.exists {
        Err(MatchOperationsError::AccountMatchAccountMissing)?
    } else {
        return Ok(());
    };

    let sub_account = if let Some(sub) = &account.sub_account {
        sub
    } else if req.sub_account_exists {
        Err(MatchOperationsError::AccountMatchSubAccountMissing)?
    } else {
        return Ok(());
    };

    if !req.sub_account_exists {
        Err(MatchOperationsError::AccountMatchSubAccountPopulated)?
    };

    if !req.sub_account_address.is_empty() && sub_account.address != req.sub_account_address {
        Err(format!(
            "{}: expected {} but got {}",
            MatchOperationsError::AccountMatchUnexpectedSubAccountAddr,
            req.sub_account_address,
            sub_account.address
        ))?
    }

    metadata_match(&req.sub_account_metadata_keys, &sub_account.metadata)
        .map_err(|e| ParserError::String(format!("{}: account metadata keys mismatch", e)))
}

/// [`amount_match`] returns an error if an [`Amount`] does not meet an
/// [`AmountDescription`].
pub fn amount_match(
    req: Option<&AmountDescription>,
    amount: Option<&NullableAmount>,
) -> ParserResult<()> {
    let req = if let Some(req) = req {
        req
    } else {
        return Ok(());
    };

    if amount.is_none() {
        if req.exists {
            Err(MatchOperationsError::AmountMatchAmountMissing)?
        } else {
            return Ok(());
        }
    };

    if !req.exists {
        Err(MatchOperationsError::AmountMatchAmountPopulated)?
    }

    if !req.sign.match_(amount) {
        Err(format!(
            "{}: expected {}",
            MatchOperationsError::AmountMatchUnexpectedSign,
            req.sign.string()
        ))?
    }

    // If no currency is provided, anything is ok.
    if req.currency.is_none() {
        return Ok(());
    };

    let amount = amount.unwrap();
    if amount.currency.is_none() || hash(amount.currency.as_ref()) != hash(req.currency.as_ref()) {
        Err(ParserError::String(format!(
            "{}: expected {:?} but got {:?}",
            MatchOperationsError::AmountMatchUnexpectedCurrency,
            req.currency,
            amount.currency
        )))
    } else {
        Ok(())
    }
}

#[allow(clippy::missing_docs_in_private_items)]
pub fn coin_action_match(
    required_action: NullableCoinAction,
    coin_change: Option<&NullableCoinChange>,
) -> ParserResult<()> {
    if required_action.is_empty() {
        return Ok(());
    }

    let coin_change = coin_change.ok_or(format!(
        "{}: expected {}",
        MatchOperationsError::CoinActionMatchCoinChangeIsNil,
        required_action
    ))?;

    if coin_change.coin_action != required_action {
        Err(ParserError::String(format!(
            "{}: expected {} but got {}",
            MatchOperationsError::CoinActionMatchUnexpectedCoinAction,
            required_action,
            coin_change.coin_action
        )))
    } else {
        Ok(())
    }
}

/// [`operation_match`] returns an error if a [`NullableOperation`] does not
/// match a [`OperationDescription`].
pub fn operation_match(
    operation: Option<&NullableOperation>,
    descriptions: &[Option<Descriptions>],
    matches: &[Option<Match>],
) -> bool {
    todo!()
}

/// equalAmounts returns an error if a slice of operations do not have
/// equal amounts.
pub fn equal_amounts(ops: &[Option<NullableOperation>]) -> ParserResult<()> {
    todo!()
}

/// oppositeAmounts returns an error if two operations do not have opposite
/// amounts.
pub fn opposite_amounts(
    a: Option<&NullableOperation>,
    b: Option<&NullableOperation>,
) -> ParserResult<()> {
    todo!()
}

/// oppositeOrZeroAmounts returns an error if two operations do not have
/// opposite amounts and both amounts are not zero.
pub fn opposite_or_zero_amounts(
    a: Option<&NullableOperation>,
    b: Option<&NullableOperation>,
) -> ParserResult<()> {
    todo!()
}

/// equalAddresses returns an error if a slice of operations do not have
/// equal addresses.
pub fn equal_addresses(ops: &[Option<NullableOperation>]) -> ParserResult<()> {
    todo!()
}

#[allow(clippy::missing_docs_in_private_items)]
pub fn match_index_valid(matches: &[Match], index: usize) -> ParserResult<()> {
    todo!()
}

#[allow(clippy::missing_docs_in_private_items)]
pub fn check_ops(
    requests: &[Vec<usize>],
    matches: &[Option<Match>],
    valid: fn(&[Option<NullableOperation>]) -> ParserResult<()>,
) -> ParserResult<()> {
    todo!()
}

/// [`compare_opposite_matches`] ensures collections of [`NullableOperation`]
/// that may have opposite amounts contain valid matching amounts
pub fn compare_opposite_matches(
    amount_pairs: &[Vec<usize>],
    matches: &[Option<Match>],
    amount_checker: fn(Option<&NullableOperation>, Option<&NullableOperation>) -> ParserResult<()>,
) -> ParserResult<()> {
    todo!()
}

/// [`comparison_matches`] ensures collections of [`NullableOperation`]
/// have either equal or opposite amounts.
pub fn comparison_match(
    descriptions: Option<&Descriptions>,
    matches: &[Match],
) -> ParserResult<()> {
    todo!()
}

/// `Match` contains all [`NullableOperation`] matching a given
/// [`OperationDescription`] and their parsed [`BigInt`] amounts (if populated).
#[allow(clippy::missing_docs_in_private_items)]
pub struct Match {
    pub operations: Vec<Option<NullableOperation>>,
    /// `amounts` has the same length as [`Operations`]. If an operation has
    /// a populate `amount`, its corresponding [`BigInt`] will be non-nil.
    pub amounts: Vec<Option<BigInt>>,
}

impl Match {
    /// `first` is a convenience method that returns the first matched operation
    /// and amount (if they exist). This is used when parsing matches when
    /// `allow_repeats` is set to false.
    pub fn first(m: Option<&Self>) -> (Option<NullableOperation>, Option<BigInt>) {
        todo!()
    }
}

/// `match_operations` attempts to match a slice of operations with a slice of
/// [`OperationDescription`]s (high-level descriptions of what operations are
/// desired). If matching succeeds, a slice of matching operations in the
/// mapped to the order of the descriptions is returned.
fn match_operations(
    descriptions: Option<&Descriptions>,
    operations: &[Option<NullableOperation>],
) -> ParserResult<Vec<Option<Match>>> {
    todo!()
}
