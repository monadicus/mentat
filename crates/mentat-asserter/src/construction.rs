//! Validates that construction data is correct.

use super::*;

/// the request public keys are not valid AccountIdentifiers.
pub fn construction_preprocess_response(
    resp: Option<&UncheckedConstructionPreprocessResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionPreprocessResponseIsNil)?;

    resp.required_public_keys.iter().try_for_each(|pub_key| {
        account_identifier(pub_key.as_ref())
            .map_err(|e| format!("account identifier {pub_key:?} is invalid: {e}"))
    })?;

    Ok(())
}

/// `construction_metadata_response` returns an error if
/// the metadata is not a JSON object.
pub fn construction_metadata_response(
    resp: Option<&UncheckedConstructionMetadataResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionMetadataResponseIsNil)?;

    if resp.metadata.is_none() {
        Err(ConstructionError::ConstructionMetadataResponseMetadataMissing)?;
    }

    assert_unique_amounts(&resp.suggested_fee)
        .map_err(|err| format!("suggested fee {:?} is invalid: {err}", resp.suggested_fee))?;

    Ok(())
}

/// `TransactionIdentifierResponse` returns an error if
/// the [`TransactionIdentifier`] in the response is not
/// valid.
pub fn transaction_identifier_response(
    response: Option<&UncheckedTransactionIdentifierResponse>,
) -> AssertResult<()> {
    let response = response.ok_or(ConstructionError::TxIdentifierResponseIsNil)?;
    transaction_identifier(response.transaction_identifier.as_ref()).map_err(|e| {
        format!(
            "transaction identifier {:?} is invalid: {e}",
            response.transaction_identifier
        )
        .into()
    })
}

/// `ConstructionCombineResponse` returns an error if
/// a [`ConstructionCombineResponse`] does
/// not have a populated [`SignedTransaction`].
pub fn construction_combine_response(
    response: Option<&UncheckedConstructionCombineResponse>,
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
pub fn construction_derive_response(
    resp: Option<&UncheckedConstructionDeriveResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionDeriveResponseIsNil)?;

    account_identifier(resp.account_identifier.as_ref()).map_err(|e| {
        format!(
            "account identifier {:?} is invalid: {e}",
            resp.account_identifier
        )
    })?;

    Ok(())
}

impl Asserter {
    /// ConstructionParseResponse returns an error if
    /// a *types.ConstructionParseResponse does
    /// not have a valid set of operations or
    /// if the signers is empty.
    pub fn construction_parse_response(
        &self,
        resp: Option<&UncheckedConstructionParseResponse>,
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
            .map_err(|e| format!("operations {:?} are invalid: {e}", resp.operations))?;

        if signed && resp.account_identifier_signers.is_empty() {
            Err(ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx)?;
        }

        if !signed && !resp.account_identifier_signers.is_empty() {
            Err(ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx)?;
        }

        resp.account_identifier_signers
            .iter()
            .try_for_each(|ident| {
                account_identifier(ident.as_ref())
                    .map_err(|e| format!("account identifier of signer {ident:?} is invalid: {e}",))
            })?;

        if !resp.account_identifier_signers.is_empty() {
            account_array("signers", &resp.account_identifier_signers).map_err(|e| {
                format!(
                    "account identifiers of signers {:?} are invalid: {e}",
                    resp.account_identifier_signers
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
pub fn construction_payloads_response(
    resp: Option<&UncheckedConstructionPayloadsResponse>,
) -> AssertResult<()> {
    let resp = resp.ok_or(ConstructionError::ConstructionPayloadsResponseIsNil)?;

    if resp.unsigned_transaction.is_empty() {
        Err(ConstructionError::ConstructionPayloadsResponseUnsignedTxEmpty)?;
    }

    if resp.payloads.is_empty() {
        Err(ConstructionError::ConstructionPayloadsResponsePayloadsEmpty)?;
    }

    resp.payloads.iter().try_for_each(|payload| {
        signing_payload(payload.as_ref())
            .map_err(|e| format!("signing payload {payload:?} is invalid: {e}"))
    })?;

    Ok(())
}

/// `public_key` returns an error if
/// the [PublicKey] is nil, is not
/// valid hex, or has an undefined CurveType.
pub fn public_key(key: Option<&UncheckedPublicKey>) -> AssertResult<()> {
    let key = key.ok_or(ConstructionError::PublicKeyIsNil)?;

    if key.bytes.is_empty() {
        Err(ConstructionError::PublicKeyBytesEmpty)?;
    }

    if bytes_array_zero(&key.bytes) {
        Err(ConstructionError::PublicKeyBytesZero)?;
    }

    curve_type(&key.curve_type)
        .map_err(|e| format!("public key curve type {} is invalid: {e}", key.curve_type))?;

    Ok(())
}

/// `curve_type` returns an error if
/// the curve is not a valid [CurveType].
pub fn curve_type(curve: &UncheckedCurveType) -> AssertResult<()> {
    if !curve.valid() {
        Err(ConstructionError::CurveTypeNotSupported)?
    } else {
        Ok(())
    }
}

/// `signing_payload` returns an error
/// if a [SigningPayload] is nil,
/// has an empty address, has invalid hex,
/// or has an invalid [SignatureType] (if populated).
pub fn signing_payload(payload: Option<&UncheckedSigningPayload>) -> AssertResult<()> {
    let payload = payload.ok_or(ConstructionError::SigningPayloadIsNil)?;

    account_identifier(payload.account_identifier.as_ref()).map_err(|e| {
        format!(
            "account identifier {:?} is invalid: {e}",
            payload.account_identifier
        )
    })?;

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

    signature_type(&payload.signature_type).map_err(|e| {
        format!(
            "signature type {:?} is invalid: {e}",
            payload.signature_type
        )
    })?;

    Ok(())
}

/// `signatures` returns an error if any
/// [Signature] is invalid.
pub fn signatures(signatures: &[Option<&UncheckedSignature>]) -> AssertResult<()> {
    if signatures.is_empty() {
        Err(ConstructionError::SignaturesEmpty)?;
    }

    for sig in signatures {
        // TODO coinbase doesn't check for nil here
        let sig = sig.unwrap();
        signing_payload(sig.signing_payload.as_ref())
            .map_err(|e| format!("signing payload {:?} is invalid: {e}", sig.signing_payload))?;

        public_key(sig.public_key.as_ref())
            .map_err(|e| format!("public key {:?} is invalid: {e}", sig.public_key))?;

        signature_type(&sig.signature_type)
            .map_err(|e| format!("signature type {:?} is invalid: {e}", sig.signature_type))?;

        // Return an error if the requested signature type does not match the
        // signature type in the returned signature.
        let sig_type = &sig.signing_payload.as_ref().unwrap().signature_type;
        if !sig_type.is_empty() && *sig_type != sig.signature_type {
            Err(ConstructionError::SignaturesReturnedSigMismatch)?;
        } else if sig.bytes.is_empty() {
            Err(ConstructionError::SignatureBytesEmpty)?;
        } else if bytes_array_zero(&sig.bytes) {
            Err(ConstructionError::SignatureBytesZero)?;
        }
    }

    Ok(())
}

/// signature_type returns an error if
/// signature is not a valid [`SignatureType`].
pub fn signature_type(st: &UncheckedSignatureType) -> AssertResult<()> {
    if !st.valid() {
        Err(ConstructionError::SignatureTypeNotSupported)?
    } else {
        Ok(())
    }
}
