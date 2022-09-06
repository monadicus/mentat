//! TODO

use std::fmt::Write;

use indexmap::IndexSet;
use mentat_types::{hash, AccountIdentifier, Operation, SigningPayload};

use crate::{IntentError, Parser, ParserResult};

/// `expected_operation` returns an error if an observed operation
/// differs from the intended operation. An operation is considered
/// to be different from the intent if the [`AccountIdentifier`],
/// [`Amount`], or [`Type`] has changed.
pub fn expected_operation(intent: &Operation, observed: &Operation) -> ParserResult<()> {
    if hash(intent.account.as_ref()) != hash(observed.account.as_ref()) {
        Err(format!(
            "expected operation account identifier {} but got {}: {}",
            serde_json::to_string_pretty(&intent.account).unwrap(),
            serde_json::to_string_pretty(&observed.account).unwrap(),
            IntentError::ExpectedOperationAccountMismatch,
        ))?
    } else if hash(intent.amount.as_ref()) != hash(observed.amount.as_ref()) {
        Err(format!(
            "expected operation amount {} but got {}: {}",
            serde_json::to_string_pretty(&intent.amount).unwrap(),
            serde_json::to_string_pretty(&observed.amount).unwrap(),
            IntentError::ExpectedOperationAmountMismatch,
        ))?
    } else if intent.type_ != observed.type_ {
        Err(format!(
            "expected operation type {} but got {}: {}",
            intent.type_,
            observed.type_,
            IntentError::ExpectedOperationTypeMismatch,
        ))?
    } else {
        Ok(())
    }
}

impl Parser {
    /// `expected_operations` returns an error if a slice of intended
    /// operations differ from observed operations. Optionally,
    /// it is possible to error if any extra observed operations
    /// are found or if operations matched are not considered
    /// successful.
    pub fn expected_operations(
        &self,
        intent: &[Operation],
        observed: &[Operation],
        err_extra: bool,
        confirm_success: bool,
    ) -> ParserResult<()> {
        let mut matches = vec![false; intent.len()];
        let mut failed_matches = Vec::new();
        for obs in observed {
            let mut found_match = false;
            for (i, intent) in intent.iter().enumerate() {
                if matches[i] {
                    continue;
                }

                // Any error returned here only indicated that intent
                // does not match observed. For ExpectedOperations,
                // we don't care about the content of the error, we
                // just care if it errors so we can evaluate the next
                // operation for a match.
                if expected_operation(intent, obs).is_err() {
                    continue;
                }

                if confirm_success {
                    // TODO coinbase never checks if self is nil
                    // TODO coinbase never checks if asserter is nil
                    let obs_success = self
                        .asserter
                        .as_ref()
                        .unwrap()
                        .operation_successful(obs)
                        .map_err(|e| {
                            format!("failed to check the status of operation {obs:?}: {e}")
                        })?;

                    if !obs_success {
                        failed_matches.push(obs);
                        continue;
                    }
                }

                matches[i] = true;
                found_match = true;
                break;
            }

            if !found_match && err_extra {
                Err(format!(
                    "{}: {}",
                    IntentError::ExpectedOperationsExtraOperation,
                    serde_json::to_string_pretty(obs).unwrap()
                ))?;
            }
        }

        let missing_intent = matches
            .into_iter()
            .enumerate()
            .filter_map(|(i, m)| if !m { Some(i) } else { None })
            .collect::<Vec<_>>();

        if !missing_intent.is_empty() {
            let mut err_string = format!("could not intent match {missing_intent:?}");

            if !failed_matches.is_empty() {
                write!(
                    err_string,
                    ": found matching ops with unsuccessful status {}",
                    serde_json::to_string_pretty(&failed_matches).unwrap()
                )
                .unwrap();
            }

            Err(err_string)?
        } else {
            Ok(())
        }
    }
}

/// ExpectedSigners returns an error if a slice of SigningPayload
/// has different signers than what was observed (typically populated
/// using the signers returned from parsing a transaction).
pub fn expected_signers(
    intent: &[SigningPayload],
    observed: &[AccountIdentifier],
) -> ParserResult<()> {
    // De-duplicate required signers (ex: multiple UTXOs from same address)
    let intended_signers = intent
        .iter()
        .map(|payload| hash(payload.account_identifier.as_ref()))
        .collect::<IndexSet<String>>();

    // Could exit here if len(intent) != len(observed) but
    // more useful to print out a detailed error message.
    let mut seen_signers = IndexSet::new();
    let mut unmatched = Vec::new();
    for signer in observed {
        let signer_hash = hash(Some(signer));
        if intended_signers.contains(&signer_hash) {
            seen_signers.insert(signer_hash);
        } else {
            unmatched.push(signer);
        }
    }

    // Check to see if there are any expected
    // signers that we could not find.
    for payload in intent {
        let hash = hash(payload.account_identifier.as_ref());
        if !seen_signers.contains(&hash) {
            Err(format!(
                "payload account identifier {:?} is invalid: {}",
                payload.account_identifier,
                IntentError::ExpectedSignerMissing,
            ))?;
        }
    }

    // Return an error if any observed signatures
    // were not expected.
    if !unmatched.is_empty() {
        Err(format!(
            "unmatched signers are {unmatched:?}: {}",
            IntentError::ExpectedSignerUnexpectedSigner,
        ))?
    } else {
        Ok(())
    }
}
