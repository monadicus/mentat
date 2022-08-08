//! TODO doc

use num_bigint_dig::{BigInt, Sign};
use num_traits::{sign::Signed, Zero};
use serde_json::Value;

use super::*;

/// AmountSign is used to represent possible signedness
/// of an amount.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AmountSign(pub usize);

impl AmountSign {
    /// `ANY` is a positive or negative amount.
    pub(crate) const ANY: AmountSign = AmountSign(0);
    /// `NEGATIVE` is a negative amount.
    pub(crate) const NEGATIVE: AmountSign = AmountSign(1);
    /// `NEGATIVE_OR_ZERO` is a positive or zero amount.
    pub(crate) const NEGATIVE_OR_ZERO: AmountSign = AmountSign(4);
    /// OPPOSITES_LENGTH is the only allowed number of
    /// operations to compare as opposites.
    pub(crate) const OPPOSITES_LENGTH: usize = 2;
    /// `POSITIVE` is a positive amount.
    pub(crate) const POSITIVE: AmountSign = AmountSign(2);
    /// `POSITIVE_OR_ZERO` is a positive or zero amount.
    pub(crate) const POSITIVE_OR_ZERO: AmountSign = AmountSign(3);

    /// match_ returns a boolean indicating if an [`Amount`]
    /// has an [`AmountSign`].
    pub fn match_(self, amount: Option<&Amount>) -> bool {
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
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct MetadataDescription {
    pub key: String,
    pub value_kind: Value,
}

/// AccountDescription is used to describe a [`AccountIdentifier`].
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct AccountDescription {
    pub exists: bool,
    pub sub_account_exists: bool,
    pub sub_account_address: String,
    pub sub_account_metadata_keys: Vec<Option<MetadataDescription>>,
}

/// AmountDescription is used to describe a [`Amount`].
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct AmountDescription {
    pub exists: bool,
    pub sign: AmountSign,
    pub currency: Option<Currency>,
}

/// OperationDescription is used to describe a [`Operation`].
#[derive(Debug, Default, PartialEq, Eq)]
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
    /// [`CoinChange`] and that it should have the
    /// [`CoinAction`]. If this is not populated,
    /// [`CoinChange`] is not checked.
    pub coin_action: CoinAction,
}

/// Descriptions contains a slice of [`OperationDescription`]s and
/// high-level requirements enforced across multiple [`Operation`]s.
#[derive(Debug, Default, PartialEq, Eq)]
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
        .map_err(|e| ParserError::String(format!("{e}: account metadata keys mismatch")))
}

/// [`amount_match`] returns an error if an [`Amount`] does not meet an
/// [`AmountDescription`].
pub fn amount_match(req: Option<&AmountDescription>, amount: Option<&Amount>) -> ParserResult<()> {
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
    // TODO redundant check
    // if amount.currency.is_none() ||
    if hash(Some(&amount.currency)) != hash(req.currency.as_ref()) {
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
    required_action: CoinAction,
    coin_change: Option<&CoinChange>,
) -> ParserResult<()> {
    // TODO redundant check
    // if required_action.is_empty() {
    //     return Ok(());
    // }

    let coin_change = coin_change.ok_or(format!(
        "{}: expected {required_action}",
        MatchOperationsError::CoinActionMatchCoinChangeIsNil
    ))?;

    if coin_change.coin_action != required_action {
        Err(ParserError::String(format!(
            "{}: expected {required_action} but got {}",
            MatchOperationsError::CoinActionMatchUnexpectedCoinAction,
            coin_change.coin_action
        )))
    } else {
        Ok(())
    }
}

/// [`operation_match`] returns an error if a [`Operation`] does not
/// match a [`OperationDescription`].
pub fn operation_match(
    operation: Option<Operation>,
    descriptions: &[Option<OperationDescription>],
    matches: &mut [Option<Match>],
) -> bool {
    // TODO coinbase never checks for null here
    let operation = operation.unwrap();
    for (i, des) in descriptions.iter().enumerate() {
        // TODO: coinbase never checks for null here
        let des = des.as_ref().unwrap();
        if (matches[i].is_some() && !des.allow_repeats)
            || (!des.type_.is_empty() && des.type_ != operation.type_)
            || account_match(des.account.as_ref(), operation.account.as_ref()).is_err()
            || amount_match(des.amount.as_ref(), operation.amount.as_ref()).is_err()
            || metadata_match(&des.metadata, &operation.metadata).is_err()
            || coin_action_match(des.coin_action, operation.coin_change.as_ref()).is_err()
        {
            continue;
        }

        if matches[i].is_none() {
            matches[i] = Some(Match::default());
        }

        if operation.amount.is_some() {
            let val = if let Ok(v) = amount_value(operation.amount.as_ref()) {
                v
            } else {
                continue;
            };
            matches[i].as_mut().unwrap().amounts.push(Some(val));
        } else {
            matches[i].as_mut().unwrap().amounts.push(None);
        }

        // Wait to add operation to matches in case that we "continue" when
        // parsing `operation.amount`.
        matches[i]
            .as_mut()
            .unwrap()
            .operations
            .push(Some(operation));
        return true;
    }
    false
}

/// equalAmounts returns an error if a slice of operations do not have
/// equal amounts.
pub fn equal_amounts(ops: &[Option<&Operation>]) -> ParserResult<()> {
    if ops.is_empty() {
        Err(MatchOperationsError::EqualAmountsNoOperations)?;
    }

    // TODO coinbase never checks nil
    let val = amount_value(ops[0].unwrap().amount.as_ref())?;

    for op in ops {
        // TODO coinbase never checks nil
        let other_val = amount_value(op.unwrap().amount.as_ref())?;

        if val != other_val {
            Err(format!(
                "{}: {val} is not equal to {other_val}",
                MatchOperationsError::EqualAmountsNotEqual,
            ))?;
        }
    }

    Ok(())
}

/// oppositeAmounts returns an error if two operations do not have opposite
/// amounts.
pub fn opposite_amounts(a: Option<&Operation>, b: Option<&Operation>) -> ParserResult<()> {
    // TODO coinbase never checks nil
    let a_val = amount_value(a.as_ref().unwrap().amount.as_ref())?;
    // TODO coinbase never checks nil
    let b_val = amount_value(b.as_ref().unwrap().amount.as_ref())?;

    if a_val.sign() == b_val.sign() {
        Err(format!(
            "{}: {a_val} and {b_val}",
            MatchOperationsError::OppositeAmountsSameSign,
        ))?
    } else if a_val.abs() != b_val.abs() {
        Err(format!(
            "{a_val}: {} and {b_val}",
            MatchOperationsError::OppositeAmountsAbsValMismatch,
        ))?
    } else {
        Ok(())
    }
}

/// oppositeOrZeroAmounts returns an error if two operations do not have
/// opposite amounts and both amounts are not zero.
pub fn opposite_or_zero_amounts(a: Option<&Operation>, b: Option<&Operation>) -> ParserResult<()> {
    // TODO coinbase never checks nil
    let a_val = amount_value(a.as_ref().unwrap().amount.as_ref())?;
    // TODO coinbase never checks nil
    let b_val = amount_value(b.as_ref().unwrap().amount.as_ref())?;

    if a_val.is_zero() && b_val.is_zero() {
        Ok(())
    } else if a_val.sign() == b_val.sign() {
        Err(format!(
            "{}: {a_val} and {b_val}",
            MatchOperationsError::OppositeAmountsSameSign,
        ))?
    } else if a_val.abs() != b_val.abs() {
        Err(format!(
            "{}: {a_val} and {b_val}",
            MatchOperationsError::OppositeAmountsAbsValMismatch,
        ))?
    } else {
        Ok(())
    }
}

/// equalAddresses returns an error if a slice of operations do not have
/// equal addresses.
pub fn equal_addresses(ops: &[Option<&Operation>]) -> ParserResult<()> {
    if ops.len() <= 1 {
        Err(format!(
            "{}: got {} operations",
            MatchOperationsError::EqualAddressesTooFewOperations,
            ops.len()
        ))?;
    }

    let mut base = "";

    for op in ops {
        // TODO coinbase never checks nil
        let op = op.unwrap();
        let account = op
            .account
            .as_ref()
            .ok_or(MatchOperationsError::EqualAddressesAccountIsNil)?;

        if base.is_empty() {
            base = &account.address;
            continue;
        } else if base != account.address {
            Err(format!(
                "{}: {base} is not equal to {}",
                MatchOperationsError::EqualAddressesAddrMismatch,
                account.address
            ))?;
        }
    }

    Ok(())
}

#[allow(clippy::missing_docs_in_private_items)]
pub fn match_index_valid(matches: &[Option<Match>], index: usize) -> ParserResult<()> {
    match matches.get(index) {
        None => Err(format!(
            "{}: at index {index}",
            MatchOperationsError::MatchIndexValidIndexOutOfRange,
        ))?,
        Some(None) => Err(format!(
            "{}: at index {index}",
            MatchOperationsError::MatchIndexValidIndexIsNil,
        ))?,
        Some(Some(_)) => Ok(()),
    }
}

#[allow(clippy::missing_docs_in_private_items)]
pub fn check_ops(
    requests: &[Vec<usize>],
    matches: &[Option<Match>],
    valid: fn(&[Option<&Operation>]) -> ParserResult<()>,
) -> ParserResult<()> {
    for batch in requests {
        let mut ops = Vec::new();
        for req_index in batch {
            match_index_valid(matches, *req_index)
                .map_err(|e| format!("{e}: index {req_index} not valid"))?;

            ops.extend(
                matches[*req_index]
                    .as_ref()
                    .unwrap()
                    .operations
                    .iter()
                    .map(|o| o.as_ref()),
            )
        }
        valid(&ops).map_err(|e| format!("{e} operations not valid"))?;
    }

    Ok(())
}

/// [`compare_opposite_matches`] ensures collections of [`Operation`]
/// that may have opposite amounts contain valid matching amounts
pub fn compare_opposite_matches(
    amount_pairs: &[Vec<usize>],
    matches: &[Option<Match>],
    amount_checker: fn(Option<&Operation>, Option<&Operation>) -> ParserResult<()>,
) -> ParserResult<()> {
    for amount_match in amount_pairs {
        if amount_match.len() != AmountSign::OPPOSITES_LENGTH {
            // cannot have opposites without exactly 2
            Err(format!(
                "cannot check opposites of {} operations",
                amount_match.len()
            ))?;
        }

        // compare all possible pairs
        match_index_valid(matches, amount_match[0])
            .map_err(|e| format!("{e}: amount comparison error"))?;
        match_index_valid(matches, amount_match[1])
            .map_err(|e| format!("{e}: amount comparison error"))?;

        let match_0_ops = &matches[amount_match[0]].as_ref().unwrap().operations;
        let match_1_ops = &matches[amount_match[1]].as_ref().unwrap().operations;
        let eq_amounts = |ops: &[Option<Operation>], amount_match| {
            let ops = ops.iter().map(|v| v.as_ref()).collect::<Vec<_>>();
            equal_amounts(&ops).map_err(|e| {
                format!("{e}: amounts comparison error for match index {amount_match}",)
            })
        };
        eq_amounts(match_0_ops, amount_match[0])?;
        eq_amounts(match_1_ops, amount_match[1])?;

        // only need to check amount for the very first operation from each
        // matched operations group since we made sure all amounts within the same
        // matched operation group are the same
        amount_checker(match_0_ops[0].as_ref(), match_1_ops[0].as_ref())
            .map_err(|e| format!("{e}: amounts do not match the amount_checker function"))?;
    }

    Ok(())
}

/// [`comparison_matches`] ensures collections of [`Operation`]
/// have either equal or opposite amounts.
pub fn comparison_match(
    descriptions: Option<&Descriptions>,
    matches: &[Option<Match>],
) -> ParserResult<()> {
    let descriptions = descriptions.unwrap();
    check_ops(&descriptions.equal_amounts, matches, equal_amounts)
        .map_err(|e| format!("{e}: operation amounts not equal"))?;
    check_ops(&descriptions.equal_addresses, matches, equal_addresses)
        .map_err(|e| format!("{e}: operation addresses not equal"))?;
    compare_opposite_matches(&descriptions.opposite_amounts, matches, opposite_amounts)
        .map_err(|e| format!("{e}: operation amounts not opposite"))?;
    compare_opposite_matches(
        &descriptions.opposite_or_zero_amounts,
        matches,
        opposite_or_zero_amounts,
    )
    .map_err(|e| format!("{e}: both operation amounts not opposite and not zero"))?;
    Ok(())
}

/// `Match` contains all [`Operation`] matching a given
/// [`OperationDescription`] and their parsed [`BigInt`] amounts (if populated).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct Match {
    pub operations: Vec<Option<Operation>>,
    /// `amounts` has the same length as [`Operations`]. If an operation has
    /// a populate `amount`, its corresponding [`BigInt`] will be non-nil.
    pub amounts: Vec<Option<BigInt>>,
}

impl Match {
    /// `first` is a convenience method that returns the first matched operation
    /// and amount (if they exist). This is used when parsing matches when
    /// `allow_repeats` is set to false.
    pub fn first(m: Option<&Self>) -> (Option<&Operation>, Option<&BigInt>) {
        match m {
            Some(m) if !m.operations.is_empty() => {
                (m.operations[0].as_ref(), m.amounts[0].as_ref())
            }
            _ => (None, None),
        }
    }
}

/// `match_operations` attempts to match a slice of operations with a slice of
/// [`OperationDescription`]s (high-level descriptions of what operations are
/// desired). If matching succeeds, a slice of matching operations in the
/// mapped to the order of the descriptions is returned.
pub fn match_operations(
    descriptions: Option<&Descriptions>,
    operations: Vec<Option<Operation>>,
) -> ParserResult<Vec<Option<Match>>> {
    // TODO coinbase never checks nil
    let descriptions = descriptions.unwrap();
    if operations.is_empty() {
        Err(MatchOperationsError::MatchOperationsNoOperations)?;
    } else if descriptions.operation_descriptions.is_empty() {
        Err(MatchOperationsError::MatchOperationsDescriptionsMissing)?;
    }

    let operation_descriptions = &descriptions.operation_descriptions;
    let mut matches = vec![None; operation_descriptions.len()];

    // Match an Operation to each OperationDescription
    for (i, op) in operations.into_iter().enumerate() {
        let match_found = operation_match(op, operation_descriptions, &mut matches);
        if !match_found && descriptions.err_unmatched {
            Err(format!(
                "{}: at index {i}",
                MatchOperationsError::MatchOperationsMatchNotFound,
            ))?;
        }
    }

    // Error if any OperationDescription is not matched
    for (i, m) in matches.iter().enumerate() {
        // TODO coinbase never checks nil
        if m.is_none()
            && !descriptions.operation_descriptions[i]
                .as_ref()
                .unwrap()
                .optional
        {
            Err(format!(
                "{}: {i}",
                MatchOperationsError::MatchOperationsDescriptionNotMatched,
            ))?;
        }
    }

    // Once matches are found, assert high-level descriptions between
    // Operations
    comparison_match(Some(descriptions), &matches)
        .map_err(|e| format!("{e}: group descriptions not met"))?;

    Ok(matches)
}
