use super::*;

#[test]
fn test_construction_preprocess_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(NullableConstructionPreprocessResponse {
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
            name: "valid response with accounts",
            payload: Some(NullableConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
            name: "invalid response with accounts",
            payload: Some(NullableConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            result: Some(BlockError::AccountAddrMissing.into()),
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::ConstructionPreprocessResponseIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_preprocess_response(t.as_ref()))
}

#[test]
fn test_construction_metadata_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
            name: "with suggested fee",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![valid_amount()],
            }),
            result: None,
        },
        TestCase {
            name: "with duplicate suggested fee",
            payload: Some(NullableConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![valid_amount(), valid_amount()],
            }),
            result: Some(
                format!(
                    "currency {:?} used multiple times",
                    valid_amount().unwrap().currency
                )
                .into(),
            ),
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::ConstructionMetadataResponseIsNil.into()),
        },
        TestCase {
            name: "invalid metadata",
            payload: Some(Default::default()),
            result: Some(ConstructionError::ConstructionMetadataResponseMetadataMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_metadata_response(t.as_ref()))
}

#[test]
fn test_transaction_identifier_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(NullableTransactionIdentifierResponse {
                transaction_identifier: Some(TransactionIdentifier { hash: "tx1".into() }),
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::TxIdentifierResponseIsNil.into()),
        },
        TestCase {
            name: "invalid transaction identifier",
            payload: Some(Default::default()),
            result: Some(BlockError::TxIdentifierIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| transaction_identifier_response(t.as_ref()))
}

#[test]
fn test_construction_combine_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(NullableConstructionCombineResponse {
                signed_transaction: "signed tx".into(),
            }),
            result: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::ConstructionCombineResponseIsNil.into()),
        },
        TestCase {
            name: "empty signed transaction",
            payload: Some(Default::default()),
            result: Some(ConstructionError::SignedTxEmpty.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_combine_response(t.as_ref()))
}

#[test]
fn test_construction_derive_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(NullableConstructionDeriveResponse {
                account_identifier: Some(AccountIdentifier {
                    address: "addr".into(),
                    metadata: [("name".into(), "hello".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::ConstructionDeriveResponseIsNil.into()),
        },
        TestCase {
            name: "empty address",
            payload: Some(NullableConstructionDeriveResponse {
                metadata: [("name".into(), "hello".into())].into(),
                ..Default::default()
            }),
            result: Some(ConstructionError::ConstructionDeriveResponseAddrEmpty.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_derive_response(t.as_ref()))
}

#[derive(Default)]
struct ConstructionParseResponseTest {
    payload: Option<NullableConstructionParseResponse>,
    signed: bool,
}

#[test]
fn test_construction_parse_response() {
    let tests = vec![
        TestCase {
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
            result: None,
        },
        TestCase {
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
            result: Some(ConstructionError::ConstructionParseResponseDuplicateSigner.into()),
        },
        TestCase {
            name: "nil response",
            payload: Some(Default::default()),
            result: Some(ConstructionError::ConstructionParseResponseIsNil.into()),
        },
        TestCase {
            name: "no operations",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(NullableConstructionParseResponse {
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            result: Some(ConstructionError::ConstructionParseResponseOperationsEmpty.into()),
        },
        TestCase {
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
            result: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        TestCase {
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
            result: Some(ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx.into()),
        },
        TestCase {
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
            result: Some(ConstructionError::ConstructionParseResponseSignerEmpty.into()),
        },
        TestCase {
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
            result: Some(
                ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx.into(),
            ),
        },
        TestCase {
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
            result: None,
        },
    ];

    let asserter = Asserter::new_client_with_responses(
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
    .unwrap();

    TestCase::run_err_match(tests, |t| {
        let t = t.as_ref().unwrap();
        asserter.construction_parse_response(t.payload.as_ref(), t.signed)
    });
}

#[test]
fn test_construction_payloads_response() {
    let tests = vec![
        TestCase {
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
            result: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            result: Some(ConstructionError::ConstructionPayloadsResponseIsNil.into()),
        },
        TestCase {
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
            result: Some(ConstructionError::ConstructionPayloadsResponseUnsignedTxEmpty.into()),
        },
        TestCase {
            name: "empty signing payloads",
            payload: Some(NullableConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                ..Default::default()
            }),
            result: Some(ConstructionError::ConstructionPayloadsResponsePayloadsEmpty.into()),
        },
        TestCase {
            name: "invalid signing payload",
            payload: Some(NullableConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                payloads: vec![Some(NullableSigningPayload {
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
            }),
            result: Some(ConstructionError::SigningPayloadAddrEmpty.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_payloads_response(t.as_ref()))
}

#[test]
fn test_public_key() {
    let tests = vec![
        TestCase {
            name: "valid public key",
            payload: Some(NullablePublicKey {
                bytes: "blah".into(),
                curve_type: NullableCurveType::SECP256K1.into(),
            }),
            result: None,
        },
        TestCase {
            name: "zero public key",
            payload: Some(NullablePublicKey {
                bytes: vec![0; 4],
                curve_type: NullableCurveType::SECP256K1.into(),
            }),
            result: Some(ConstructionError::PublicKeyBytesZero.into()),
        },
        TestCase {
            name: "nil public key",
            payload: None,
            result: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        TestCase {
            name: "invalid bytes",
            payload: Some(NullablePublicKey {
                curve_type: NullableCurveType::SECP256K1.into(),
                ..Default::default()
            }),
            result: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
        TestCase {
            name: "invalid curve",
            payload: Some(NullablePublicKey {
                bytes: "hello".into(),
                curve_type: "test".into(),
            }),
            result: Some(ConstructionError::CurveTypeNotSupported.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| public_key(t.as_ref()))
}

#[test]
fn test_signing_payload() {
    let tests = vec![
        TestCase {
            name: "valid signing payload",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                ..Default::default()
            }),
            result: None,
        },
        TestCase {
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
            result: None,
        },
        TestCase {
            name: "nil signing payload",
            payload: None,
            result: Some(ConstructionError::SigningPayloadIsNil.into()),
        },
        TestCase {
            name: "empty address",
            payload: Some(NullableSigningPayload {
                bytes: "blah".into(),
                ..Default::default()
            }),
            result: Some(ConstructionError::SigningPayloadAddrEmpty.into()),
        },
        TestCase {
            name: "zero signing payload",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: vec![0; 4],
                ..Default::default()
            }),
            result: Some(ConstructionError::SigningPayloadBytesZero.into()),
        },
        TestCase {
            name: "empty bytes",
            payload: Some(NullableSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            result: Some(ConstructionError::SigningPayloadBytesEmpty.into()),
        },
        TestCase {
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
            result: Some(ConstructionError::SignatureTypeNotSupported.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| signing_payload(t.as_ref()))
}

#[test]
fn test_signatures() {
    let tests = vec![
        TestCase {
            name: "valid signatures",
            payload: vec![
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
            ],
            result: None,
        },
        TestCase {
            name: "signature type match",
            payload: vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
                bytes: "hello".into(),
            })],
            result: None,
        },
        TestCase {
            name: "nil signatures",
            payload: Vec::new(),
            result: Some(ConstructionError::SignaturesEmpty.into()),
        },
        TestCase {
            name: "empty signature",
            payload: vec![
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
            ],
            result: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        TestCase {
            name: "signature zero bytes",
            payload: vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
                bytes: vec![0],
            })],
            result: Some(ConstructionError::SignatureBytesZero.into()),
        },
        TestCase {
            name: "signature type mismatch",
            payload: vec![Some(NullableSignature {
                signing_payload: Some(NullableSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: NullableSignatureType::ECDSA_RECOVERY.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: NullableSignatureType::ED25519.into(),
                bytes: "hello".into(),
            })],
            result: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
    ];

    TestCase::run_err_match(tests, |test| {
        signatures(&test.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
    })
}
