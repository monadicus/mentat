use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};
use mina_signer::PubKey;
use serde::Deserialize;

use super::*;

/// `SignerPallas` is initialized from a `UncheckedKeyPair`.
pub struct SignerPallas {
    pub key_pair: UncheckedKeyPair,
}

impl SignerInterface for SignerPallas {
    fn public_key(&self) -> PublicKey {
        let kp: KeyPair = self.key_pair.clone().into();
        kp.public_key
    }

    fn sign(&self, payload: SigningPayload, sig_type: SignatureType) -> KeysResult<Signature> {
        // TODO some of this seems repetitive so far?
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
                "expected signature type {} but got {sig_type}: {}",
                SignatureType::SchnorrPoseidon,
                KeysError::ErrSignUnsupportedSignatureType
            ))?;
        }

        todo!("Not possible with this library")
        // Ok(Signature {
        //     signature_type: payload.signature_type,
        //     signing_payload: payload,
        //     public_key: valid_key_pair.public_key,
        //     bytes: todo!("Not possible with this library"),
        // })
    }

    fn verify(&self, signature: UncheckedSignature) -> KeysResult<()> {
        // TODO some of this seems repetitive so far?
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
        let _signature: Signature = signature.into();

        todo!("not possible with this library");

        // Ok(())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
#[repr(u8)]
pub enum NetworkType {
    TestNet,
    MainNet,
    NullNet,
}

// https://github.com/coinbase/kryptology/blob/master/pkg/signatures/schnorr/mina/txn.go
#[derive(Debug)]
struct Transaction {
    fee: u64,
    fee_token: u64,
    fee_payer_pk: PubKey,
    nonce: u32,
    valid_until: u32,
    memo: String,
    tag: [bool; 3],
    source_pk: PubKey,
    receiver_pk: PubKey,
    token_id: u64,
    amount: u64,
    locked: bool,
    network_id: NetworkType,
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
        let from_public_key = PubKey::from_address(&value.from)
            .map_err(|err| format!("failed to parse \"from\" address: {err}"))?;

        let to_public_key = PubKey::from_address(&value.to)
            .map_err(|err| format!("failed to parse \"to\" address: {err}"))?;

        let fee = value
            .fee
            .parse()
            .map_err(|err| format!("failed to parse uint for fee: {err}"))?;

        let amount = if let Some(amt) = value.amount {
            amt.parse()
                .map_err(|err| format!("failed to parse uint for fee: {err}"))?
        } else {
            0
        };

        let nonce = value
            .nonce
            .parse()
            .map_err(|err| format!("failed to parse uint for nonce: {err}"))?;

        let valid_until = if let Some(valid_until) = value.valid_until {
            valid_until
                .parse()
                .map_err(|err| format!("failed to parse uint for valid until memo: {err}"))?
        } else {
            0
        };

        let memo = if let Some(memo) = value.memo {
            memo
        } else {
            String::new()
        };

        Ok(Transaction {
            fee,
            fee_token: 1,
            fee_payer_pk: from_public_key.clone(),
            nonce,
            valid_until,
            memo,
            tag: [false, false, false],
            source_pk: from_public_key,
            receiver_pk: to_public_key,
            token_id: 1,
            amount,
            locked: false,
            network_id: NetworkType::TestNet,
        })
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
