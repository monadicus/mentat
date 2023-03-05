use crate::{types::KeyPair, Signer};

fn test_signer() -> Signer {
    let kp = KeyPair::import_private_key(
        "A80F3DE13EE5AE01119E7D98A8F2317070BFB6D2A1EA712EE1B55EE7B938AD1D".to_string(),
        mentat_types::CurveType::Pallas,
    )
    .unwrap();
    dbg!("owo");
    kp.signer().unwrap()
}

static UNSIGNED_TX_BYTES: &[u8] = "{\"randomOracleInput\":\"000000033769356015133A338518173BE9C263D6E463538ACDF11D523\
    DDEB8C82467093E3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E167031AAE689272378D\
    05042083C66C593EF025060E4C8CA1CBD022E47C72D220000025701154880000000008000000000000000400000007FFFFFFF\
    C0000000000000000000000000000000000000000000000000000000000000000000060000000000000001BC6CD9C400000000\",\
    \"signerInput\":{\"prefix\":[\"3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E\",\
    \"3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E\",\
    \"167031AAE689272378D05042083C66C593EF025060E4C8CA1CBD022E47C72D22\"],\
    \"suffix\":[\"0000000000000007FFFFFFFC0000000400000000000000020000000002255100\",\
    \"0000000003000000000000000000000000000000000000000000000000000000\",\
    \"000000000000000000000000000000000000000000000000047366C7B0000000\"]},\
    \"payment\":{\"to\":\"B62qoLLD2LK2pL2dq2oDHh6ohdaYusgTEYRUZ43Y41Kk9Rgen4v643x\",\
    \"from\":\"B62qooQQ952uaoUSTQP3sZCviGmsWeusBwhg3qVF1Ww662sgzimA25Q\",\"fee\":\"18000000\",\
    \"token\":\"1\",\"nonce\":\"1\",\"memo\":null,\"amount\":\"2389498102\",\"valid_until\":\"4294967295\"},\
    \"stakeDelegation\":null,\"createToken\":null,\"createTokenAccount\":null,\"mintTokens\":null}".as_bytes();

#[test]
fn test_sign_pallas() {
    let signer = test_signer();
}

//     let tests = vec![
//         TestCase {
//             name: "correct payload signature type",
//             payload: mock_payload(vec![0; 32], SignatureType::Ed25519),
//             criteria: None,
//         },
//         TestCase {
//             name: "implicit payload signature type",
//             payload: mock_payload(vec![0; 32], SignatureType::EmptyString),
//             criteria: None,
//         },
//         TestCase {
//             name: "incorrect payload signature type 1",
//             payload: mock_payload(vec![0; 33], SignatureType::Ecdsa),
//             criteria:
// Some(KeysError::ErrSignUnsupportedPayloadSignatureType),         },
//         TestCase {
//             name: "incorrect payload signature type 2",
//             payload: mock_payload(vec![0; 34], SignatureType::EcdsaRecovery),
//             criteria:
// Some(KeysError::ErrSignUnsupportedPayloadSignatureType),         },
//     ];

//     TestCase::run_err_match(tests, |p| {
//         let sig = signer.sign(p, SignatureType::Ed25519)?;
//         assert_eq!(sig.bytes.len(), 64);
//         assert_eq!(signer.public_key(), sig.public_key);

//         Ok::<_, KeysError>(sig)
//     })
// }
