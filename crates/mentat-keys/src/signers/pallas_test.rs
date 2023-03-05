// use mentat_test_utils::TestCase;
// use mentat_types::SignatureType;

// use super::{mock_payload, mock_signature};
// use crate::{
//     errors::KeysError,
//     signers::{
//         // pallas::{PallasSigningPayload, PayloadFields},
//         // signer::SignerInterface,
//     },
//     types::KeyPair,
//     Signer,
// };

// fn _test_signer() -> Signer {
//     let kp = KeyPair::import_private_key(
//         "A80F3DE13EE5AE01119E7D98A8F2317070BFB6D2A1EA712EE1B55EE7B938AD1D".
// to_string(),         mentat_types::CurveType::Pallas,
//     )
//     .unwrap();
//     kp.signer().unwrap()
// }

// static _UNSIGNED_TX_BYTES: &[u8] =
// "{\"randomOracleInput\":\"
// 000000033769356015133A338518173BE9C263D6E463538ACDF11D523\
//     DDEB8C82467093E3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E167031AAE689272378D\
//     05042083C66C593EF025060E4C8CA1CBD022E47C72D220000025701154880000000008000000000000000400000007FFFFFFF\
//     C0000000000000000000000000000000000000000000000000000000000000000000060000000000000001BC6CD9C400000000\",\
//     \"signerInput\":{\"prefix\":[\"
// 3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E\",\
//     \"3769356015133A338518173BE9C263D6E463538ACDF11D523DDEB8C82467093E\",\
//     \"167031AAE689272378D05042083C66C593EF025060E4C8CA1CBD022E47C72D22\"],\
//     \"suffix\":[\"
// 0000000000000007FFFFFFFC0000000400000000000000020000000002255100\",\
//     \"0000000003000000000000000000000000000000000000000000000000000000\",\
//     \"000000000000000000000000000000000000000000000000047366C7B0000000\"]},\
//     \"payment\":{\"to\":\"
// B62qoLLD2LK2pL2dq2oDHh6ohdaYusgTEYRUZ43Y41Kk9Rgen4v643x\",\     \"from\":\"
// B62qooQQ952uaoUSTQP3sZCviGmsWeusBwhg3qVF1Ww662sgzimA25Q\",\"fee\":\"18000000\
// ",\     \"token\":\"1\",\"nonce\":\"1\",\"memo\":null,\"amount\":\"
// 2389498102\",\"valid_until\":\"4294967295\"},\     \"stakeDelegation\":null,\
// "createToken\":null,\"createTokenAccount\":null,\"mintTokens\":null}".
// as_bytes();

// // TODO
// // #[test]
// // fn test_parse_signing_payload() {
// //     let from_address =
// // "B62qkuFDYD82nxNNgGm1aJcSAErLZgAb19A9skSPtAxbBHsUZxsMbhU";
// //     let from_pub_key = ();
// //     let to_address =
// // "B62qo7Ddbw8SXo55bTH6yJAASgQ6owtMYSw5tkuPJJ6GLJ36zvUnEpG";     let
// to_pub_key // = ();

// //     let amount = Some("34".to_string());
// //     let valid_until = Some("78".to_string());
// //     let memo = Some("memo".to_string());
// //     let signing_payload_with_payment = PallasSigningPayload {
// //         payment: Some(PayloadFields {
// //             to: to_address.to_string(),
// //             from: from_address.to_string(),
// //             fee: 12.to_string(),
// //             amount: amount.clone(),
// //             nonce: 56.to_string(),
// //             valid_until: valid_until.clone(),
// //             memo: memo.clone(),
// //         }),
// //     };
// //     let signing_payload_with_no_payment = PallasSigningPayload::default();
// //     let signing_payload_with_payment_and_null_valid_until_and_null_memo =
// // PallasSigningPayload {         payment: Some(PayloadFields {
// //             to: to_address.to_string(),
// //             from: from_address.to_string(),
// //             fee: 12.to_string(),
// //             amount: amount.clone(),
// //             nonce: 56.to_string(),
// //             ..Default::default()
// //         }),
// //     };
// //     let signing_payload_with_payment_and_invalid_from_public_key =
// // PallasSigningPayload {         payment: Some(PayloadFields {
// //             to: to_address.to_string(),
// //             from: "InvalidFrom".to_string(),
// //             fee: 12.to_string(),
// //             amount: amount.clone(),
// //             nonce: 56.to_string(),
// //             valid_until: valid_until.clone(),
// //             memo: memo.clone(),
// //         }),
// //     };
// //     let signing_payload_with_payment_and_invalid_to_public_key =
// // PallasSigningPayload {         payment: Some(PayloadFields {
// //             to: "InvalidTo".to_string(),
// //             from: from_address.to_string(),
// //             fee: 12.to_string(),
// //             amount: amount.clone(),
// //             nonce: 56.to_string(),
// //             valid_until: valid_until.clone(),
// //             memo: memo.clone(),
// //         }),
// //     };

// //     unimplemented!()
// // }

// // TODO
// // #[test]
// // fn test_sign_pallas() {
// //     let signer = test_signer();

// //     let tests = vec![
// //         TestCase {
// //             name: "correct payload signature type",
// //             payload: mock_payload(UNSIGNED_TX_BYTES.to_vec(),
// SignatureType::SchnorrPoseidon), //             criteria: None,
// //         },
// //         TestCase {
// //             name: "implicit payload signature type",
// //             payload: mock_payload(UNSIGNED_TX_BYTES.to_vec(),
// SignatureType::EmptyString), //             criteria: None,
// //         },
// //         TestCase {
// //             name: "incorrect payload signature type 1",
// //             payload: mock_payload(UNSIGNED_TX_BYTES.to_vec(),
// SignatureType::Ecdsa), //             criteria:
// Some(KeysError::ErrSignUnsupportedPayloadSignatureType), //         },
// //         TestCase {
// //             name: "incorrect payload signature type 2",
// //             payload: mock_payload(UNSIGNED_TX_BYTES.to_vec(),
// SignatureType::EcdsaRecovery), //             criteria:
// Some(KeysError::ErrSignUnsupportedPayloadSignatureType), //         },
// //     ];

// //     TestCase::run_err_match(tests, |p| {
// //         let sig = signer.sign(p, SignatureType::Ed25519)?;
// //         assert_eq!(sig.bytes.len(), 64);
// //         assert_eq!(signer.public_key(), sig.public_key);

// //         Ok::<_, KeysError>(sig)
// //     })
// // }

// // TODO
// // #[test]
// // fn test_verify_pallas() {
// //     let signer = test_signer();

// //     let payload = mock_payload(UNSIGNED_TX_BYTES.to_vec(),
// SignatureType::SchnorrPoseidon); //     let test_sig = signer
// //         .sign(payload, SignatureType::SchnorrPoseidon)
// //         .unwrap();

// //     let mut simple_bytes = vec![0; 32];
// //     let hello = "hello".as_bytes();
// //     simple_bytes[..hello.len()].copy_from_slice(hello);

// //     let tests = vec![
// //         TestCase {
// //             name: "incorrect payload signature type 1",
// //             payload: mock_signature(
// //                 SignatureType::Ecdsa,
// //                 signer.public_key(),
// //                 UNSIGNED_TX_BYTES.to_vec(),
// //                 simple_bytes.clone(),
// //             ),
// //             criteria:
// Some(KeysError::ErrVerifyUnsupportedPayloadSignatureType), //         },
// //         TestCase {
// //             name: "incorrect payload signature type 2",
// //             payload: mock_signature(
// //                 SignatureType::EcdsaRecovery,
// //                 signer.public_key(),
// //                 UNSIGNED_TX_BYTES.to_vec(),
// //                 simple_bytes.clone(),
// //             ),
// //             criteria:
// Some(KeysError::ErrVerifyUnsupportedPayloadSignatureType), //         },
// //         TestCase {
// //             name: "invalid singing payload",
// //             payload: mock_signature(
// //                 SignatureType::SchnorrPoseidon,
// //                 signer.public_key(),
// //                 simple_bytes,
// //                 test_sig.bytes.clone(),
// //             ),
// //             criteria: Some(KeysError::String(
// //                 "failed to parse signing payload".to_string(),
// //             )),
// //         },
// //         TestCase {
// //             name: "correct payload signature",
// //             payload: mock_signature(
// //                 SignatureType::SchnorrPoseidon,
// //                 signer.public_key(),
// //                 UNSIGNED_TX_BYTES.to_vec(),
// //                 test_sig.bytes,
// //             ),
// //             criteria: None,
// //         },
// //     ];

// //     TestCase::run_err_match(tests, |p| signer.verify(p.into()))
// // }
