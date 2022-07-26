use super::{
    server_test::{valid_account, valid_amount, valid_public_key},
    test_utils::AsserterTest,
};
use crate::{
    asserter::{
        asserter_tools::{Asserter, RequestAsserter, ResponseAsserter},
        construction::{
            construction_combine_response, construction_derive_response,
            construction_metadata_response, construction_payloads_response,
            construction_preprocess_response, public_key, signatures, signing_payload,
            transaction_identifier_response,
        },
        errors::{BlockError, ConstructionError, ServerError},
    },
    types::{
        AccountIdentifier, CurveType, NullableConstructionCombineResponse,
        NullableConstructionDeriveResponse, NullableConstructionMetadataResponse,
        NullableConstructionParseResponse, NullableConstructionPayloadsResponse,
        NullableConstructionPreprocessResponse, NullableOperation, NullablePublicKey,
        NullableSignature, NullableSigningPayload, NullableTransactionIdentifierResponse,
        OperationIdentifier, SignatureType, TransactionIdentifier,
    },
};

#[test]
fn test_construction_preprocess_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableConstructionPreprocessResponse {
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid response with accounts",
            payload: Some(NullableConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "invalid response with accounts",
            payload: Some(NullableConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            err: Some(BlockError::AccountAddrMissing.into()),
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionPreprocessResponseIsNil.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, construction_preprocess_response)
}

#[test]
fn test_construction_metadata_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "with suggested fee",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![Some(valid_amount())],
            }),
            err: None,
        },
        AsserterTest {
            name: "with duplicate suggested fee",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![Some(valid_amount()), Some(valid_amount())],
            }),
            err: Some(format!("currency {:?} used multiple times", valid_amount().currency).into()),
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionMetadataResponseIsNil.into()),
        },
        AsserterTest {
            name: "invalid metadata",
            payload: Some(Default::default()),
            err: Some(ConstructionError::ConstructionMetadataResponseMetadataMissing.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, construction_metadata_response)
}

#[test]
fn test_transaction_identifier_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableTransactionIdentifierResponse {
                transaction_identifier: Some(TransactionIdentifier { hash: "tx1".into() }),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::TxIdentifierResponseIsNil.into()),
        },
        AsserterTest {
            name: "invalid transaction identifier",
            payload: Some(Default::default()),
            err: Some(BlockError::TxIdentifierIsNil.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, transaction_identifier_response)
}

#[test]
fn test_construction_combine_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableConstructionCombineResponse {
                signed_transaction: "signed tx".into(),
            }),
            err: None,
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionCombineResponseIsNil.into()),
        },
        AsserterTest {
            name: "empty signed transaction",
            payload: Some(Default::default()),
            err: Some(ConstructionError::SignedTxEmpty.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, construction_combine_response)
}

#[test]
fn test_construction_derive_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableConstructionDeriveResponse {
                account_identifier: Some(AccountIdentifier {
                    address: "addr".into(),
                    metadata: [("name".into(), "hello".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionDeriveResponseIsNil.into()),
        },
        AsserterTest {
            name: "empty address",
            payload: Some(NullableConstructionDeriveResponse {
                metadata: [("name".into(), "hello".into())].into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::ConstructionDeriveResponseAddrEmpty.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, construction_derive_response)
}

#[derive(Default)]
struct ConstructionParseResponseTest {
    payload: Option<NullableConstructionParseResponse>,
    signed: bool,
}

#[test]
fn test_construction_parse_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![Some(valid_account())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            err: None,
        },
        AsserterTest {
            name: "duplicate signer",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(AccountIdentifier {
                                address: "addr 2".into(),
                                ..Default::default()
                            }),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![Some(valid_account()), Some(valid_account())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            err: Some(ConstructionError::ConstructionParseResponseDuplicateSigner.into()),
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionParseResponseIsNil.into()),
        },
        AsserterTest {
            name: "no operations",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    account_identifier_signers: vec![Some(valid_account())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: Some(ConstructionError::ConstructionParseResponseOperationsEmpty.into()),
        },
        AsserterTest {
            name: "invalid operation ordering",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![Some(NullableOperation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        }),
                        type_: "PAYMENT".into(),
                        account: Some(valid_account()),
                        amount: Some(valid_amount()),
                        ..Default::default()
                    })],
                    account_identifier_signers: vec![Some(valid_account())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        AsserterTest {
            name: "no signers",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            err: Some(ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx.into()),
        },
        AsserterTest {
            name: "empty account identifier signer",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![Some(Default::default())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            err: Some(ConstructionError::ConstructionParseResponseSignerEmpty.into()),
        },
        AsserterTest {
            name: "invalid signer unsigned",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![Some(valid_account())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: Some(
                ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx.into(),
            ),
        },
        AsserterTest {
            name: "valid response unsigned",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                        Some(NullableOperation {
                            operation_identifier: Some(OperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(OperationIdentifier {
                                index: 0,
                                ..Default::default()
                            })],
                            type_: "PAYMENT".into(),
                            account: Some(valid_account()),
                            amount: Some(valid_amount()),
                            ..Default::default()
                        }),
                    ],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: None,
        },
    ];

    AsserterTest::default_request_asserter_tests(tests, |asserter, test| {
        asserter.construction_parse_response(
            test.unwrap().payload.as_ref(),
            test.as_ref().unwrap().signed,
        )
    });
}

#[test]
fn test_construction_payloads_response() {
    let tests = &[
        AsserterTest {
            name: "valid response",
            payload: Some(NullableConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                payloads: vec![Some(NullableSigningPayload {
                    account_identifier: Some(AccountIdentifier {
                        address: "hello".into(),
                        ..Default::default()
                    }),
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
            }),
            err: None,
        },
        AsserterTest {
            name: "nil response",
            payload: None,
            err: Some(ConstructionError::ConstructionPayloadsResponseIsNil.into()),
        },
        AsserterTest {
            name: "empty unsigned transaction",
            payload: Some(NullableConstructionPayloadsResponse {
                payloads: vec![Some(NullableSigningPayload {
                    account_identifier: Some(AccountIdentifier {
                        address: "hello".into(),
                        ..Default::default()
                    }),
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            err: Some(ConstructionError::ConstructionPayloadsResponseUnsignedTxEmpty.into()),
        },
        AsserterTest {
            name: "empty signing payloads",
            payload: Some(NullableConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::ConstructionPayloadsResponsePayloadsEmpty.into()),
        },
        AsserterTest {
            name: "invalid signing payload",
            payload: Some(NullableConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                payloads: vec![Some(NullableSigningPayload {
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
            }),
            err: Some(ConstructionError::SigningPayloadAddrEmpty.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, construction_payloads_response)
}

#[test]
fn test_public_key() {
    let tests = &[
        AsserterTest {
            name: "valid public key",
            payload: Some(NullablePublicKey {
                bytes: "blah".into(),
                curve_type: CurveType::SECP256K1.into(),
            }),
            err: None,
        },
        AsserterTest {
            name: "zero public key",
            payload: Some(NullablePublicKey {
                bytes: vec![0; 4],
                curve_type: CurveType::SECP256K1.into(),
            }),
            err: Some(ConstructionError::PublicKeyBytesZero.into()),
        },
        AsserterTest {
            name: "nil public key",
            payload: None,
            err: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        AsserterTest {
            name: "invalid bytes",
            payload: Some(NullablePublicKey {
                curve_type: CurveType::SECP256K1.into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
        AsserterTest {
            name: "invalid curve",
            payload: Some(NullablePublicKey {
                bytes: "hello".into(),
                curve_type: "test".into(),
            }),
            err: Some(ConstructionError::CurveTypeNotSupported.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, public_key)
}

#[test]
fn test_signing_payload() {
    let tests = &[
        AsserterTest {
            name: "valid signing payload",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "valid signing payload with signature type",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                signature_type: SignatureType::ED25519.into(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterTest {
            name: "nil signing payload",
            payload: None,
            err: Some(ConstructionError::SigningPayloadIsNil.into()),
        },
        AsserterTest {
            name: "empty address",
            payload: Some(NullableSigningPayload {
                bytes: "blah".into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::SigningPayloadAddrEmpty.into()),
        },
        AsserterTest {
            name: "zero signing payload",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: vec![0; 4],
                ..Default::default()
            }),
            err: Some(ConstructionError::SigningPayloadBytesZero.into()),
        },
        AsserterTest {
            name: "empty bytes",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: Some(ConstructionError::SigningPayloadBytesEmpty.into()),
        },
        AsserterTest {
            name: "invalid signature",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                signature_type: "blah".into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::SignatureTypeNotSupported.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, signing_payload)
}

#[test]
fn test_signatures() {
    let tests = &[
        AsserterTest {
            name: "valid signatures",
            payload: Some(vec![
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    bytes: "hello".into(),
                }),
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
            ]),
            err: None,
        },
        AsserterTest {
            name: "signature type match",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: Some(valid_account()),
                    bytes: "blah".into(),
                    signature_type: SignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: SignatureType::ED25519.into(),
                bytes: "hello".into(),
            })]),
            err: None,
        },
        AsserterTest {
            name: "nil signatures",
            payload: Some(Vec::new()),
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterTest {
            name: "empty signature",
            payload: Some(vec![
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        signature_type: SignatureType::ED25519.into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    ..Default::default()
                }),
            ]),
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        AsserterTest {
            name: "signature zero bytes",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: Some(valid_account()),
                    bytes: "blah".into(),
                    signature_type: SignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: SignatureType::ED25519.into(),
                bytes: vec![0],
            })]),
            err: Some(ConstructionError::SignatureBytesZero.into()),
        },
        AsserterTest {
            name: "signature type mismatch",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: Some(valid_account()),
                    bytes: "blah".into(),
                    signature_type: SignatureType::ECDSA_RECOVERY.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: SignatureType::ED25519.into(),
                bytes: "hello".into(),
            })]),
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
    ];

    AsserterTest::non_asserter_tests(tests, |test| {
        signatures(&test.unwrap().iter().map(|s| s.as_ref()).collect::<Vec<_>>())
    })
}
