//! TODO

use std::fmt::Write;

use indexmap::IndexSet;
use mentat_types::{hash, AccountIdentifier, Operation, SigningPayload};

use crate::{IntentError, Parser, ParserResult};

/// `expected_operation` returns an error if an observed operation
/// differs from the intended operation. An operation is considered
/// to be different from the intent if the [`AccountIdentifier`],
/// [`Amount`], or [`Type`] has changed.
pub fn expected_operation(
    intent: Option<&Operation>,
    observed: Option<&Operation>,
) -> ParserResult<()> {
    // TODO coinbase never checks nil here
    let intent = intent.unwrap();
    // TODO coinbase never checks nil here
    let observed = observed.unwrap();
    if hash(intent.account.as_ref()) != hash(observed.account.as_ref()) {
        Err(format!(
            "{}: expected {} but got {}",
            IntentError::ExpectedOperationAccountMismatch,
            serde_json::to_string_pretty(&intent.account).unwrap(),
            serde_json::to_string_pretty(&observed.account).unwrap()
        ))?
    } else if hash(intent.amount.as_ref()) != hash(observed.amount.as_ref()) {
        Err(format!(
            "{}: expected {} but got {}",
            IntentError::ExpectedOperationAmountMismatch,
            serde_json::to_string_pretty(&intent.amount).unwrap(),
            serde_json::to_string_pretty(&observed.amount).unwrap()
        ))?
    } else if intent.type_ != observed.type_ {
        Err(format!(
            "{}: expected {} but got {}",
            IntentError::ExpectedOperationTypeMismatch,
            intent.type_,
            observed.type_
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
        intent: &[Option<Operation>],
        observed: &[Option<Operation>],
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
                if expected_operation(intent.as_ref(), obs.as_ref()).is_err() {
                    continue;
                }
                let obs = obs.as_ref().unwrap();

                if confirm_success {
                    // TODO coinbase never checks if self is nil
                    // TODO coinbase never checks if asserter is nil
                    let obs_success = self
                        .asserter
                        .as_ref()
                        .unwrap()
                        .operation_successful(obs)
                        .map_err(|e| format!("{e}: unable to check operation success"))?;

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
    intent: &[Option<SigningPayload>],
    observed: &[Option<AccountIdentifier>],
) -> ParserResult<()> {
    // TODO coinbase never checks nil
    // De-duplicate required signers (ex: multiple UTXOs from same address)
    let intended_signers = intent
        .iter()
        .map(|payload| hash(payload.as_ref().unwrap().account_identifier.as_ref()))
        .collect::<IndexSet<String>>();

    // Could exit here if len(intent) != len(observed) but
    // more useful to print out a detailed error message.
    let mut seen_signers = IndexSet::new();
    let mut unmatched = Vec::new();
    for signer in observed {
        let signer_hash = hash(signer.as_ref());
        if intended_signers.contains(&signer_hash) {
            seen_signers.insert(signer_hash);
        } else {
            unmatched.push(signer);
        }
    }

    // Check to see if there are any expected
    // signers that we could not find.
    for payload in intent {
        // TODO coinbase never checks nil
        let payload = payload.as_ref().unwrap();
        let hash = hash(payload.account_identifier.as_ref());
        if seen_signers.contains(&hash) {
            Err(format!(
                "{}: {}",
                IntentError::ExpectedSignerMissing,
                serde_json::to_string(&payload.account_identifier).unwrap()
            ))?;
        }
    }

    // Return an error if any observed signatures
    // were not expected.
    if !unmatched.is_empty() {
        Err(format!(
            "{}: {}",
            IntentError::ExpectedSignerUnexpectedSigner,
            serde_json::to_string_pretty(&unmatched).unwrap()
        ))?
    } else {
        Ok(())
    }
}
