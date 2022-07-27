//! Validates that construction data is correct.

use super::{
    account_array,
    account_identifier,
    assert_unique_amounts,
    asserter_tools::Asserter,
    block::transaction_identifier,
    bytes_array_zero,
    errors::{AsserterError, BlockError},
    AssertResult,
    ConstructionDeriveResponse,
    ConstructionError,
    ConstructionMetadataResponse,
    ConstructionParseResponse,
    ConstructionPayloadsResponse,
    ConstructionPreprocessResponse,
    CurveType,
    PublicKey,
    ResponseAsserter,
    Signature,
    SignatureType,
    SigningPayload,
};
use crate::types::{ConstructionCombineResponse, TransactionIdentifierResponse};

/// the request public keys are not valid AccountIdentifiers.
pub(crate) fn construction_preprocess_response(
    resp: Option<&ConstructionPreprocessResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionPreprocessResponseIsNil)?;

    resp.required_public_keys
        .iter()
        .try_for_each(|pub_key| account_identifier(pub_key.as_ref()))?;

    Ok(())
}

/// `construction_metadata_response` returns an error if
/// the metadata is not a JSON object.
pub(crate) fn construction_metadata_response(
    resp: Option<&ConstructionMetadataResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionMetadataResponseIsNil)?;

    if resp.metadata.is_none() {
        Err(ConstructionError::ConstructionMetadataResponseMetadataMissing)?;
    }

    assert_unique_amounts(&resp.suggested_fee)
        .map_err(|err| format!("{err}: duplicate suggested fee currency found"))?;

    Ok(())
}

/// `TransactionIdentifierResponse` returns an error if
/// the [`TransactionIdentifier`] in the response is not
/// valid.
pub(crate) fn transaction_identifier_response(
    response: Option<&TransactionIdentifierResponse>,
) -> AssertResult<()> {
    let response = response.ok_or(ConstructionError::TxIdentifierResponseIsNil)?;
    transaction_identifier(response.transaction_identifier.as_ref())
}

/// `ConstructionCombineResponse` returns an error if
/// a [`ConstructionCombineResponse`] does
/// not have a populated [`SignedTransaction`].
pub(crate) fn construction_combine_response(
    response: Option<&ConstructionCombineResponse>,
) -> AssertResult<()> {
    let response = response.ok_or(ConstructionError::ConstructionCombineResponseIsNil)?;
    if response.signed_transaction.is_empty() {
        Err(ConstructionError::SignedTxEmpty)?
    } else {
        Ok(())
    }
}

/// `construction_derive_response` returns an error if
/// a [`ConstructionDeriveResponse`] does
/// not have a populated Address.
pub(crate) fn construction_derive_response(
    resp: Option<&ConstructionDeriveResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionDeriveResponseIsNil)?;

    account_identifier(resp.account_identifier.as_ref()).map_err(|err| {
        format!(
            "{}: {err}",
            ConstructionError::ConstructionDeriveResponseAddrEmpty
        )
    })?;

    Ok(())
}

impl Asserter {
    /// ConstructionParseResponse returns an error if
    /// a *types.ConstructionParseResponse does
    /// not have a valid set of operations or
    /// if the signers is empty.
    pub(crate) fn construction_parse_response(
        &self,
        resp: Option<&ConstructionParseResponse>,
        signed: bool,
    ) -> AssertResult<()> {
        self.response
            .as_ref()
            .ok_or(AsserterError::NotInitialized)?;

        let resp = resp.ok_or(ConstructionError::ConstructionParseResponseIsNil)?;

        if resp.operations.is_empty() {
            Err(ConstructionError::ConstructionParseResponseOperationsEmpty)?;
        }

        self.operations(&resp.operations, true)
            .map_err(|err| format!("{err} unable to parse operations"))?;

        if signed && resp.account_identifier_signers.is_empty() {
            Err(ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx)?;
        }

        if !signed && !resp.account_identifier_signers.is_empty() {
            Err(ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx)?;
        }

        resp.account_identifier_signers
            .iter()
            .enumerate()
            .try_for_each(|(index, ident)| {
                account_identifier(ident.as_ref()).map_err(|_| {
                    format!(
                        "{} at index {index}",
                        ConstructionError::ConstructionParseResponseSignerEmpty
                    )
                })
            })?;

        if !resp.account_identifier_signers.is_empty() {
            account_array("signers", &resp.account_identifier_signers).map_err(|err| {
                format!(
                    "{}: {err}",
                    ConstructionError::ConstructionParseResponseDuplicateSigner
                )
            })?;
        }

        Ok(())
    }
}

/// `construction_payloads_response` returns an error if
/// a [`ConstructionPayloadsResponse`] does
/// not have an UnsignedTransaction or has no
/// valid [`SigningPayload`].
pub(crate) fn construction_payloads_response(
    resp: Option<&ConstructionPayloadsResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionPayloadsResponseIsNil)?;

    if resp.unsigned_transaction.is_empty() {
        Err(ConstructionError::ConstructionPayloadsResponseUnsignedTxEmpty)?;
    }

    if resp.payloads.is_empty() {
        Err(ConstructionError::ConstructionPayloadsResponsePayloadsEmpty)?;
    }

    resp.payloads
        .iter()
        .enumerate()
        .try_for_each(|(index, payload)| {
            signing_payload(payload.as_ref())
                .map_err(|err| format!("{err}: signing payload {index} is invalid"))
        })?;

    Ok(())
}

/// `public_key` returns an error if
/// the [PublicKey] is nil, is not
/// valid hex, or has an undefined CurveType.
pub(crate) fn public_key(key: Option<&PublicKey>) -> AssertResult<()> {
    let key = key.ok_or(ConstructionError::PublicKeyIsNil)?;

    if key.bytes.is_empty() {
        Err(ConstructionError::PublicKeyBytesEmpty)?;
    }

    if bytes_array_zero(&key.bytes) {
        Err(ConstructionError::PublicKeyBytesZero)?;
    }

    curve_type(&key.curve_type)
        .map_err(|err| format!("{err} public key curve type is not supported"))?;

    Ok(())
}

/// `curve_type` returns an error if
/// the curve is not a valid [CurveType].
pub(crate) fn curve_type(curve: &CurveType) -> AssertResult<()> {
    if !curve.valid() {
        Err(format!(
            "{}: {}",
            ConstructionError::CurveTypeNotSupported,
            curve
        ))?
    } else {
        Ok(())
    }
}

/// `signing_payload` returns an error
/// if a [SigningPayload] is nil,
/// has an empty address, has invalid hex,
/// or has an invalid [SignatureType] (if populated).
pub(crate) fn signing_payload(payload: Option<&SigningPayload>) -> AssertResult<()> {
    let payload = payload.ok_or(ConstructionError::SigningPayloadIsNil)?;

    account_identifier(payload.account_identifier.as_ref())
        .map_err(|err| format!("{}: {err}", ConstructionError::SigningPayloadAddrEmpty))?;

    if payload.bytes.is_empty() {
        Err(ConstructionError::SigningPayloadBytesEmpty)?;
    }

    if bytes_array_zero(&payload.bytes) {
        Err(ConstructionError::SigningPayloadBytesZero)?;
    }

    // SignatureType can be optionally populated
    if payload.signature_type.is_empty() {
        return Ok(());
    }

    signature_type(&payload.signature_type)
        .map_err(|err| format!("{err} signature payload signature type is not valid"))?;

    Ok(())
}

/// `signatures` returns an error if any
/// [Signature] is invalid.
pub(crate) fn signatures(signatures: &[Option<&Signature>]) -> AssertResult<()> {
    if signatures.is_empty() {
        Err(ConstructionError::SignaturesEmpty)?;
    }

    for (index, sig) in signatures.iter().enumerate() {
        // TODO coinbase doesn't check for nil here
        let sig = sig.unwrap();
        signing_payload(sig.signing_payload.as_ref())
            .map_err(|err| format!("{err}: signature {index} has invalid signing payload"))?;

        public_key(sig.public_key.as_ref())
            .map_err(|err| format!("{err}: signature {index} has invalid public key"))?;

        signature_type(&sig.signature_type)
            .map_err(|err| format!("{err}: signature {index} has invalid signature type"))?;

        // Return an error if the requested signature type does not match the
        // signature type in the returned signature.
        let sig_type = &sig.signing_payload.as_ref().unwrap().signature_type;
        if !sig_type.is_empty() && *sig_type != sig.signature_type {
            Err(ConstructionError::SignaturesReturnedSigMismatch)?;
        } else if sig.bytes.is_empty() {
            Err(format!(
                "{}: signature {index} has 0 bytes",
                ConstructionError::SignatureBytesEmpty
            ))?;
        } else if bytes_array_zero(&sig.bytes) {
            Err(ConstructionError::SignatureBytesZero)?;
        }
    }

    Ok(())
}

/// signature_type returns an error if
/// signature is not a valid [`SignatureType`].
pub(crate) fn signature_type(st: &SignatureType) -> AssertResult<()> {
    if !st.valid() {
        Err(AsserterError::from(format!(
            "{}: {}",
            ConstructionError::SignatureTypeNotSupported,
            st
        )))
    } else {
        Ok(())
    }
}
