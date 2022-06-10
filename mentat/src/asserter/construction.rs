//! Validates that construction data is correct.

use super::{
    account::assert_unique_amounts,
    asserter_tools::ResponseAsserter,
    block::account_identifier,
    errors::{AssertResult, ConstructionError},
    util::{account_array, bytes_array_zero},
};
use crate::{
    models::{CurveType, PublicKey, Signature, SignatureType, SigningPayload},
    responses::{
        ConstructionDeriveResponse,
        ConstructionMetadataResponse,
        ConstructionParseResponse,
        ConstructionPayloadsResponse,
        ConstructionPreprocessResponse,
    },
};

/// the request public keys are not valid AccountIdentifiers.
pub(crate) fn construction_preprocess_response(
    resp: &ConstructionPreprocessResponse,
) -> AssertResult<()> {
    // TODO if resp nil

    resp.required_public_keys
        .iter()
        .flatten()
        .try_for_each(|pub_key| account_identifier(Some(pub_key)))?;

    Ok(())
}

/// `construction_metadata_response` returns an error if
/// the metadata is not a JSON object.
pub(crate) fn construction_metadata_response(
    resp: &ConstructionMetadataResponse,
) -> AssertResult<()> {
    // TODO if resp nil

    if resp.metadata.is_empty() {
        Err(ConstructionError::ConstructionMetadataResponseMetadataMissing)?;
    }

    resp.suggested_fee
        .as_ref()
        .map(|fee| assert_unique_amounts(fee))
        .transpose()
        .map_err(|err| format!("{err}: duplicate suggested fee currency found"))?;

    Ok(())
}

/// `construction_derive_response` returns an error if
/// a [`ConstructionDeriveResponse`] does
/// not have a populated Address.
pub(crate) fn construction_derive_response(resp: &ConstructionDeriveResponse) -> AssertResult<()> {
    // TODO if resp nil

    account_identifier(resp.account_identifier.as_ref()).map_err(|err| {
        format!(
            "{}: {err}",
            ConstructionError::ConstructionDeriveResponseAddrEmpty
        )
    })?;

    Ok(())
}

impl ResponseAsserter {
    /// ConstructionParseResponse returns an error if
    /// a *types.ConstructionParseResponse does
    /// not have a valid set of operations or
    /// if the signers is empty.
    pub(crate) fn ConstructionParseResponse(
        &self,
        resp: &ConstructionParseResponse,
        signed: bool,
    ) -> AssertResult<()> {
        // if self nil
        // if resp nil

        if resp.operations.is_empty() {
            Err(ConstructionError::ConstructionParseResponseOperationsEmpty)?;
        }

        self.operations(&resp.operations, true)
            .map_err(|err| format!("{err} unable to parse operations"))?;

        if signed
            && resp
                .account_identifier_signers
                .as_ref()
                .map_or_else(|| true, |v| v.is_empty())
        {
            Err(ConstructionError::ConstructionParseResponseIsNil)?;
        }

        if !signed
            && resp
                .account_identifier_signers
                .as_ref()
                .map_or_else(|| true, |v| v.is_empty())
        {
            Err(ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx)?;
        }

        resp.account_identifier_signers
            .as_ref()
            .unwrap()
            .iter()
            .enumerate()
            .try_for_each(|(index, ident)| {
                account_identifier(Some(ident)).map_err(|err| format!("{err} at index {index}"))
            })?;

        if resp
            .account_identifier_signers
            .as_ref()
            .map_or_else(|| false, |v| !v.is_empty())
        {
            account_array("signers", resp.account_identifier_signers.as_ref().unwrap())?;
        }

        Ok(())
    }
}

/// `construction_payload_response` returns an error if
/// a [`ConstructionPayloadsResponse`] does
/// not have an UnsignedTransaction or has no
/// valid [`SigningPayload`].
pub(crate) fn construction_payload_response(
    resp: &ConstructionPayloadsResponse,
) -> AssertResult<()> {
    // TODO if resp nil

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
            signing_payload(payload)
                .map_err(|err| format!("{err}: signing payload {index} is invalid"))
        })?;

    Ok(())
}

/// `public_key` returns an error if
/// the [PublicKey] is nil, is not
/// valid hex, or has an undefined CurveType.
pub(crate) fn public_key(key: &PublicKey) -> AssertResult<()> {
    // TODO if key nil
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
pub(crate) fn curve_type(_: &CurveType) -> AssertResult<()> {
    // todo impossible
    Ok(())
}

/// `signing_payload` returns an error
/// if a [SigningPayload] is nil,
/// has an empty address, has invalid hex,
/// or has an invalid [SignatureType] (if populated).
pub(crate) fn signing_payload(payload: &SigningPayload) -> AssertResult<()> {
    // TODO if payload ni;

    account_identifier(payload.account_identifier.as_ref())
        .map_err(|err| format!("{}: {err}", ConstructionError::SigningPayloadAddrEmpty))?;

    if payload.bytes.is_empty() {
        Err(ConstructionError::SigningPayloadBytesEmpty)?;
    }

    if bytes_array_zero(&payload.bytes) {
        Err(ConstructionError::SigningPayloadBytesZero)?;
    }

    // SignatureType can be optionally populated
    if payload.signature_type.is_none() {
        return Ok(());
    }

    signature_type(payload.signature_type.as_ref().unwrap())
        .map_err(|err| format!("{err} signature payload signature type is not valid"))?;

    Ok(())
}

/// `signatures` returns an error if any
/// [Signature] is invalid.
pub(crate) fn signatures(signatures: &[Signature]) -> AssertResult<()> {
    if signatures.is_empty() {
        Err(ConstructionError::SignaturesEmpty)?;
    }

    for (index, sig) in signatures.iter().enumerate() {
        signing_payload(&sig.signing_payload)
            .map_err(|err| format!("{err}: signature {index} has invalid signing payload"))?;

        public_key(&sig.public_key)
            .map_err(|err| format!("{err}: signature {index} has invalid public key"))?;

        signature_type(&sig.signature_type)
            .map_err(|err| format!("{err}: signature {index} has invalid signature type"))?;

        // Return an error if the requested signature type does not match the
        // signature type in the returned signature.
        // TODO they check if sigtype is an empty string but ours is an enum
        if sig
            .signing_payload
            .signature_type
            .as_ref()
            .map_or_else(|| false, |s| s != &sig.signature_type)
        {
            Err(ConstructionError::SignaturesReturnedSigMismatch)?;
        }

        if sig.bytes.is_empty() {
            Err(format!(
                "{}: signature {index} has 0 bytes",
                ConstructionError::SignatureBytesEmpty
            ))?;
        }

        if bytes_array_zero(&sig.bytes) {
            Err(ConstructionError::SigningPayloadBytesZero)?;
        }
    }

    Ok(())
}

/// signature_type returns an error if
/// signature is not a valid [`SignatureType`].
pub(crate) fn signature_type(_: &SignatureType) -> AssertResult<()> {
    // TODO impossible since ours is enum
    Ok(())
}
