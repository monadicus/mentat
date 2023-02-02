use mentat_types::{
    AccountIdentifier,
    CurveType,
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
};

use crate::{types::KeyPair, Signer};

pub fn mock_signer(curve: CurveType) -> Signer {
    let key_pair = KeyPair::generate(curve).unwrap();

    key_pair.signer().unwrap()
}

pub fn mock_payload(msg: Vec<u8>, signature_type: SignatureType) -> SigningPayload {
    SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".to_string(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type,
        ..Default::default()
    }
}

pub fn mock_signature(
    signature_type: SignatureType,
    public_key: PublicKey,
    msg: Vec<u8>,
    sig: Vec<u8>,
) -> Signature {
    let signing_payload = SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "test".into(),
            ..Default::default()
        }),
        bytes: msg,
        signature_type,
        ..Default::default()
    };

    Signature {
        signing_payload,
        public_key,
        signature_type,
        bytes: sig,
    }
}
