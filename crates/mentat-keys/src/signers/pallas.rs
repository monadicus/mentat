use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};
use serde::Deserialize;

use super::SignerInterface;
use crate::{
    errors::{KeysError, KeysResult},
    types::{KeyPair, UncheckedKeyPair},
};

pub struct SignerPallas {
    pub key_pair: UncheckedKeyPair,
}

impl SignerInterface for SignerPallas {
    fn public_key(&self) -> PublicKey {
        let kp: KeyPair = self.key_pair.clone().into();
        kp.public_key
    }

    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature> {
        let valid_key_pair = self
            .key_pair
            .clone()
            .is_valid()
            .map_err(|err| format!("key pair is invalid: {err}"))?;

        if !(matches!(
            payload.signature_type,
            SignatureType::SchnorrPoseidon | SignatureType::EmptyString
        )) {
            Err(format!(
                "expected signing payload signature type {} but got {}: {}",
                SignatureType::SchnorrPoseidon,
                payload.signature_type,
                KeysError::ErrSignUnsupportedPayloadSignatureType
            ))?;
        }

        if !matches!(sig_type, SignatureType::SchnorrPoseidon) {
            Err(format!(
                "expected signature type {} but got {}: {}",
                SignatureType::SchnorrPoseidon,
                sig_type,
                KeysError::ErrSignUnsupportedSignatureType
            ))?;
        }

        Ok(Signature {
            signature_type: payload.signature_type,
            signing_payload: payload,
            public_key: valid_key_pair.public_key,
            bytes: todo!(),
        })
    }

    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()> {
        if signature.signature_type != UncheckedSignatureType::SCHNORR_POSEIDON.into() {
            Err(format!(
                "expected signing payload signature type {} but got {}: {}",
                SignatureType::SchnorrPoseidon,
                signature.signature_type,
                KeysError::ErrVerifyUnsupportedPayloadSignatureType
            ))?;
        }

        mentat_asserter::signatures(&[Some(&signature)])
            .map_err(|err| format!("signature is invalid: {err}"))?;
        let signature: Signature = signature.into();

        todo!();

        Ok(())
    }
}

// https://github.com/coinbase/kryptology/blob/master/pkg/signatures/schnorr/mina/txn.go
struct Transaction {
    fee: u64,
    fee_token: u64,
    fee_payer_pk: (),
    nonce: u32,
    valid_until: u32,
    memo: String,
    tag: [bool; 3],
    source_pk: (),
    receiver_pk: (),
    token_id: u64,
    amount: u64,
    locked: bool,
    network_id: (),
}

#[derive(Debug, Deserialize)]
struct PayloadFields {
    to: String,
    from: String,
    fee: String,
    #[serde(skip_serializing_if = "Option::is_empty")]
    amount: Option<String>,
    nonce: String,
    #[serde(skip_serializing_if = "Option::is_empty")]
    valid_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_empty")]
    memo: Option<String>,
}

impl TryFrom<PayloadFields> for Transaction {
    type Error = KeysError;

    fn try_from(value: PayloadFields) -> Result<Self, Self::Error> {
      // let public_key = mina_signer::PubKey;
        todo!()
    }
}

#[derive(Debug, Deserialize)]
struct PallasSigningPayload {
    payment: Option<PayloadFields>,
}

impl TryFrom<SigningPayload> for Transaction {
    type Error = KeysError;

    fn try_from(raw_payload: SigningPayload) -> Result<Self, Self::Error> {
        let signing_payload: PallasSigningPayload = serde_json::from_slice(&raw_payload.bytes)
            .map_err(|err| format!("failed to unmarshal payload: {err}"))?;

        let payload_fields = signing_payload
            .payment
            .ok_or(KeysError::ErrPaymentNotFound)?;

        let tx = payload_fields
            .try_into()
            .map_err(|err| format!("failed to construct transaction: {err}"))?;

        Ok(tx)
    }
}
