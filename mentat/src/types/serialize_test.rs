use serde::{Deserialize, Serialize};

use crate::types::{decode_from_hex_string, CurveType, PublicKey};

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
