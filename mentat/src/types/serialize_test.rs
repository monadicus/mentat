use serde::{Deserialize, Serialize};

use crate::types::{
    decode_from_hex_string,
    AccountIdentifier,
    ConstructionDeriveResponse,
    ConstructionParseResponse,
    CurveType,
    PublicKey,
    Signature,
    SignatureType,
    SigningPayload,
    SubAccountIdentifier,
};

#[derive(Deserialize, Serialize)]
struct HexBytesTester {
    hex_bytes: String,
}

#[test]
fn test_custom_marshal_public_key() {
    let s = PublicKey {
        bytes: "hsdjkfhkasjfhkjasdhfkjasdnfkjabsdfkjhakjsfdhjksadhfjk23478923645yhsdfn"
            .as_bytes()
            .to_vec(),
        curve_type: CurveType::SECP256K1.into(),
    };
    let json = serde_json::to_string(&s).unwrap();

    // Simple hex check
    let simple_type: HexBytesTester = serde_json::from_str(&json).unwrap();
    let b = decode_from_hex_string(simple_type.hex_bytes).unwrap();
    assert_eq!(b, s.bytes);

    // Full Check
    let s2: PublicKey = serde_json::from_str(&json).unwrap();
    assert_eq!(s.bytes, s2.bytes);

    // Invalid Hex Check
    let s3: Result<PublicKey, _> = serde_json::from_str("{ \"hex_bytes\": \"hello\" }");
    assert!(s3.is_err());
}

#[test]
fn test_custom_marshal_signature() {
    let s = Signature {
        bytes: "hsdjkfhkasjfhkjasdhfkjasdnfkjabsdfkjhakjsfdhjksadhfjk23478923645yhsdfn"
            .as_bytes()
            .to_vec(),
        signature_type: SignatureType::ECDSA.into(),
        ..Default::default()
    };
    let json = serde_json::to_string(&s).unwrap();

    // Simple hex check
    let simple_type: HexBytesTester = serde_json::from_str(&json).unwrap();
    let b = decode_from_hex_string(simple_type.hex_bytes).unwrap();
    assert_eq!(b, s.bytes);

    // Full Check
    let s2: Signature = serde_json::from_str(&json).unwrap();
    assert_eq!(s.bytes, s2.bytes);

    // Invalid Hex Check
    let s3: Result<Signature, _> = serde_json::from_str("{ \"hex_bytes\": \"hello\" }");
    assert!(s3.is_err());
}

#[test]
fn test_custom_marshal_signing_payload() {
    let s = SigningPayload {
        account_identifier: Some(AccountIdentifier {
            address: "addr1".into(),
            sub_account: Some(SubAccountIdentifier {
                address: "sub".into(),
                metadata: Default::default(),
            }),
            metadata: Default::default(),
        }),
        bytes: "hsdjkfhkasjfhkjasdhfkjasdnfkjabsdfkjhakjsfdhjksadhfjk23478923645yhsdfn"
            .as_bytes()
            .to_vec(),
        ..Default::default()
    };
    let json = serde_json::to_string(&s).unwrap();

    // TODO Simple check
    let simple_type: HexBytesTester = serde_json::from_str(&json).unwrap();
    let b = decode_from_hex_string(simple_type.hex_bytes).unwrap();
    assert_eq!(b, s.bytes);

    // Full Check
    let s2: SigningPayload = serde_json::from_str(&json).unwrap();
    assert_eq!(s, s2);

    // Invalid Hex Check
    let s3: Result<SigningPayload, _> = serde_json::from_str("{ \"hex_bytes\": \"hello\" }");
    assert!(s3.is_err());

    // Deserialize Fields
    let s4: SigningPayload =
        serde_json::from_str("{ \"address\": \"hello\", \"hex_bytes\": \"74657374\" }").unwrap();
    assert_eq!(
        Some(AccountIdentifier {
            address: "hello".into(),
            ..Default::default()
        }),
        s4.account_identifier
    );
    assert_eq!("test".as_bytes(), &s4.bytes);

    // Deserialize Fields (empty address)
    let s5: SigningPayload = serde_json::from_str("{ \"hex_bytes\": \"74657374\" }").unwrap();
    assert!(s5.account_identifier.is_none());
    assert_eq!("test".as_bytes(), &s5.bytes);
}

#[test]
fn test_custom_construction_derive_response() {
    let s = ConstructionDeriveResponse {
        account_identifier: Some(AccountIdentifier {
            address: "addr1".into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let json = serde_json::to_string(&s).unwrap();

    // TODO Simple check

    // Full Check
    let s2: ConstructionDeriveResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(s, s2);

    // Deserialize Fields
    let s3: ConstructionDeriveResponse =
        serde_json::from_str("{ \"address\": \"hello\", \"hex_bytes\": \"74657374\" }").unwrap();
    assert_eq!(
        Some(AccountIdentifier {
            address: "hello".into(),
            ..Default::default()
        }),
        s3.account_identifier
    );

    // Deserialize Fields (empty address)
    let s4: ConstructionDeriveResponse =
        serde_json::from_str("{ \"hex_bytes\": \"74657374\" }").unwrap();
    assert!(s4.account_identifier.is_none());

    // Deserialize Fields (override)
    let s5: ConstructionDeriveResponse =
        serde_json::from_str("{ \"address\": \"hello\", \"account_identifier\": { \"address\": \"hello2\" }, \"hex_bytes\": \"74657374\" }").unwrap();
    assert_eq!(
        Some(AccountIdentifier {
            address: "hello2".into(),
            ..Default::default()
        }),
        s5.account_identifier
    );
}

#[test]
fn test_custom_construction_parse_response() {
    let s = ConstructionParseResponse {
        account_identifier_signers: vec![
            Some(AccountIdentifier {
                address: "addr1".into(),
                sub_account: Some(SubAccountIdentifier {
                    address: "sub".into(),
                    metadata: Default::default(),
                }),
                ..Default::default()
            }),
            Some(AccountIdentifier {
                address: "addr2".into(),

                ..Default::default()
            }),
        ],
        ..Default::default()
    };
    let json = serde_json::to_string(&s).unwrap();

    // TODO Simple check

    // Full Check
    let s2: ConstructionParseResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(s, s2);

    // Deserialize Fields
    let s3: ConstructionParseResponse =
        serde_json::from_str("{ \"signers\": [\"hello\"], \"hex_bytes\": \"74657374\" }").unwrap();
    assert_eq!(
        vec![Some(AccountIdentifier {
            address: "hello".into(),
            ..Default::default()
        })],
        s3.account_identifier_signers
    );

    // Deserialize Fields (empty address)
    let s4: ConstructionParseResponse =
        serde_json::from_str("{ \"hex_bytes\": \"74657374\" }").unwrap();
    assert!(s4.signers.is_empty());

    // Deserialize Fields (override)
    let s5: ConstructionParseResponse =
        serde_json::from_str("{ \"signers\": [\"hello\"], \"account_identifier_signers\": [{ \"address\": \"hello2\" }], \"hex_bytes\": \"74657374\" }").unwrap();
    assert_eq!(
        vec![Some(AccountIdentifier {
            address: "hello2".into(),
            ..Default::default()
        })],
        s5.account_identifier_signers
    );
}
