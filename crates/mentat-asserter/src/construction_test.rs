use super::*;

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

    AsserterTest::run(tests, construction_preprocess_response)
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
                suggested_fee: vec![valid_amount()],
            }),
            err: None,
        },
        AsserterTest {
            name: "with duplicate suggested fee",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![valid_amount(), valid_amount()],
            }),
            err: Some(
                format!(
                    "currency {:?} used multiple times",
                    valid_amount().unwrap().currency
                )
                .into(),
            ),
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

    AsserterTest::run(tests, construction_metadata_response)
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

    AsserterTest::run(tests, transaction_identifier_response)
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

    AsserterTest::run(tests, construction_combine_response)
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

    AsserterTest::run(tests, construction_derive_response)
}

#[derive(Default)]
struct ConstructionParseResponseTest {
    payload: Option<NullableConstructionParseResponse>,
    signed: bool,
}

#[test]
fn test_construction_parse_response() {
    let tests = &[
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            extras: (),
            err: None,
        },
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![valid_account(), valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            extras: (),
            err: Some(ConstructionError::ConstructionParseResponseDuplicateSigner.into()),
        },
        CustomAsserterTest {
            name: "nil response",
            payload: Some(Default::default()),
            extras: (),
            err: Some(ConstructionError::ConstructionParseResponseIsNil.into()),
        },
        CustomAsserterTest {
            name: "no operations",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            extras: (),
            err: Some(ConstructionError::ConstructionParseResponseOperationsEmpty.into()),
        },
        CustomAsserterTest {
            name: "invalid operation ordering",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    operations: vec![Some(NullableOperation {
                        operation_identifier: Some(OperationIdentifier {
                            index: 1,
                            ..Default::default()
                        }),
                        type_: "PAYMENT".into(),
                        account: valid_account(),
                        amount: valid_amount(),
                        ..Default::default()
                    })],
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            extras: (),
            err: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            extras: (),
            err: Some(ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx.into()),
        },
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![Some(Default::default())],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                signed: true,
            }),
            extras: (),
            err: Some(ConstructionError::ConstructionParseResponseSignerEmpty.into()),
        },
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            extras: (),
            err: Some(
                ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx.into(),
            ),
        },
        CustomAsserterTest {
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
                            account: valid_account(),
                            amount: valid_amount(),
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
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                    ],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            extras: (),
            err: None,
        },
    ];

    let asserter = |_: &()| -> Asserter {
        Asserter::new_client_with_responses(
            Some(NetworkIdentifier {
                blockchain: "hello".into(),
                network: "world".into(),
                ..Default::default()
            }),
            Some(NullableNetworkStatusResponse {
                current_block_identifier: Some(BlockIdentifier {
                    index: 100,
                    hash: "block 100".into(),
                }),
                genesis_block_identifier: Some(BlockIdentifier {
                    index: 0,
                    hash: "block 0".into(),
                }),
                current_block_timestamp: MIN_UNIX_EPOCH + 1,
                peers: vec![Some(Peer {
                    peer_id: "peer 1".into(),
                    metadata: Default::default(),
                })],
                ..Default::default()
            }),
            Some(NullableNetworkOptionsResponse {
                version: Some(Version {
                    rosetta_version: "1.4.0".into(),
                    node_version: "1.0".into(),
                    ..Default::default()
                }),
                allow: Some(NullableAllow {
                    operation_statuses: vec![
                        Some(OperationStatus {
                            status: "SUCCESS".into(),
                            successful: true,
                        }),
                        Some(OperationStatus {
                            status: "FAILURE".into(),
                            successful: false,
                        }),
                    ],
                    operation_types: vec!["PAYMENT".into()],
                    ..Default::default()
                }),
            }),
            None,
        )
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(tests, asserter, |asserter, payload| {
        asserter.construction_parse_response(
            payload.unwrap().payload.as_ref(),
            payload.as_ref().unwrap().signed,
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

    AsserterTest::run(tests, construction_payloads_response)
}

#[test]
fn test_public_key() {
    let tests = &[
        AsserterTest {
            name: "valid public key",
            payload: Some(NullablePublicKey {
                bytes: "blah".into(),
                curve_type: NullableCurveType::SECP256K1.into(),
            }),
            err: None,
        },
        AsserterTest {
            name: "zero public key",
            payload: Some(NullablePublicKey {
                bytes: vec![0; 4],
                curve_type: NullableCurveType::SECP256K1.into(),
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
                curve_type: NullableCurveType::SECP256K1.into(),
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

    AsserterTest::run(tests, public_key)
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
                signature_type: NullableSignatureType::ED25519.into(),
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

    AsserterTest::run(tests, signing_payload)
}

#[test]
fn test_signatures() {
    let tests = &[
        AsserterTest {
            name: "valid signatures",
            payload: Some(vec![
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: NullableSignatureType::ED25519.into(),
                    bytes: "hello".into(),
                }),
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: NullableSignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
            ]),
            err: None,
        },
        AsserterTest {
            name: "signature type match",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
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
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: NullableSignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
                Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        signature_type: NullableSignatureType::ED25519.into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: NullableSignatureType::ED25519.into(),
                    ..Default::default()
                }),
            ]),
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        AsserterTest {
            name: "signature zero bytes",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
                bytes: vec![0],
            })]),
            err: Some(ConstructionError::SignatureBytesZero.into()),
        },
        AsserterTest {
            name: "signature type mismatch",
            payload: Some(vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ECDSA_RECOVERY.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
                bytes: "hello".into(),
            })]),
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
    ];

    AsserterTest::run(tests, |test| {
        signatures(&test.unwrap().iter().map(|s| s.as_ref()).collect::<Vec<_>>())
    })
}