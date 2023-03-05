use mentat_types::{
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    UncheckedSignature,
    UncheckedSignatureType,
};
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
        let _valid_key_pair = self
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

        unimplemented!("no good library exists for this in rust")
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

        unimplemented!("no good library exists for this in rust")

        // Ok(())
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum NetworkType {
    TestNet,
    // MainNet,
    // NullNet,
}

type PallasPublicKey = ();

// https://github.com/coinbase/kryptology/blob/master/pkg/signatures/schnorr/mina/txn.go
#[derive(Debug, Clone)]
struct Transaction {
    _fee: u64,
    _fee_token: u64,
    _fee_payer_pk: PallasPublicKey,
    _nonce: u32,
    _valid_until: u32,
    _memo: String,
    _tag: [bool; 3],
    _source_pk: PallasPublicKey,
    _receiver_pk: PallasPublicKey,
    _token_id: u64,
    _amount: u64,
    _locked: bool,
    _network_id: NetworkType,
}

// impl Hashable for Transaction {
//     type D = NetworkId;

//     fn to_roinput(&self) -> mina_hasher::ROInput {
//         let mut roi = ROInput::new();

//         roi = roi.append_field(self.fee_payer_pk.x);
//         roi = roi.append_field(self.source_pk.x);
//         roi = roi.append_field(self.receiver_pk.x);

//         roi = roi.append_u64(self.fee);
//         roi = roi.append_u64(self.fee_token);
//         roi = roi.append_u32(self.nonce);
//         roi = roi.append_u32(self.valid_until);
//         roi = roi.append_bytes(self.memo.as_bytes());

//         for b in self.tag {
//             roi = roi.append_bool(b);
//         }

//         roi = roi.append_u64(self.token_id);
//         roi = roi.append_u64(self.amount);
//         roi = roi.append_bool(self.locked);

//         roi
//     }

//     fn domain_string(domain_param: Self::D) -> Option<String> {
//         match domain_param {
//             NetworkId::MAINNET => "MinaSignatureMainnet",
//             NetworkId::TESTNET => "CodaSignature",
//         }
//         .to_string()
//         .into()
//     }
// }

#[derive(Debug, Deserialize, Default)]
pub struct PayloadFields {
    pub to: String,
    pub from: String,
    pub fee: String,
    #[serde(skip_serializing_if = "Option::is_empty")]
    pub amount: Option<String>,
    pub nonce: String,
    #[serde(skip_serializing_if = "Option::is_empty")]
    pub valid_until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_empty")]
    pub memo: Option<String>,
}

impl TryFrom<PayloadFields> for Transaction {
    type Error = KeysError;

    fn try_from(value: PayloadFields) -> Result<Self, Self::Error> {
        // let from_public_key = PubKey::from_address(&value.from)
        // .map_err(|err| format!("failed to parse \"from\" address: {err}"))?;
        //
        // let to_public_key = PubKey::from_address(&value.to)
        // .map_err(|err| format!("failed to parse \"to\" address: {err}"))?;

        let _fee = value
            .fee
            .parse()
            .map_err(|err| format!("failed to parse uint for fee: {err}"))?;

        let _amount = if let Some(amt) = value.amount {
            amt.parse()
                .map_err(|err| format!("failed to parse uint for fee: {err}"))?
        } else {
            0
        };

        let _nonce = value
            .nonce
            .parse()
            .map_err(|err| format!("failed to parse uint for nonce: {err}"))?;

        let _valid_until = if let Some(valid_until) = value.valid_until {
            valid_until
                .parse()
                .map_err(|err| format!("failed to parse uint for valid until memo: {err}"))?
        } else {
            0
        };

        let _memo = if let Some(memo) = value.memo {
            memo
        } else {
            String::new()
        };

        Ok(Transaction {
            _fee,
            _fee_token: 1,
            _fee_payer_pk: (),
            _nonce,
            _valid_until,
            _memo,
            _tag: [false, false, false],
            _source_pk: (),
            _receiver_pk: (),
            _token_id: 1,
            _amount,
            _locked: false,
            _network_id: NetworkType::TestNet,
        })
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PallasSigningPayload {
    pub payment: Option<PayloadFields>,
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
