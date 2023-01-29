use mentat_types::{AccountIdentifier, CurveType, SignatureType, SigningPayload};

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
