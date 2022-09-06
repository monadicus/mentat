use super::*;

#[test]
fn test_construction_preprocess_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(UncheckedConstructionPreprocessResponse {
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid response with accounts",
            payload: Some(UncheckedConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid response with accounts",
            payload: Some(UncheckedConstructionPreprocessResponse {
                required_public_keys: vec![Some(AccountIdentifier {
                    address: "".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            criteria: Some(BlockError::AccountAddrMissing.into()),
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::ConstructionPreprocessResponseIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_preprocess_response(t.as_ref()))
}

#[test]
fn test_construction_metadata_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(UncheckedConstructionMetadataResponse {
                metadata: Some(Default::default()),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "with suggested fee",
            payload: Some(UncheckedConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![valid_amount()],
            }),
            criteria: None,
        },
        TestCase {
            name: "with duplicate suggested fee",
            payload: Some(UncheckedConstructionMetadataResponse {
                metadata: Some(Default::default()),
                suggested_fee: vec![valid_amount(), valid_amount()],
            }),
            criteria: Some(AccountBalanceError::CurrencyUsedMultipleTimes.into()),
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::ConstructionMetadataResponseIsNil.into()),
        },
        TestCase {
            name: "invalid metadata",
            payload: Some(Default::default()),
            criteria: Some(ConstructionError::ConstructionMetadataResponseMetadataMissing.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_metadata_response(t.as_ref()))
}

#[test]
fn test_transaction_identifier_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(UncheckedTransactionIdentifierResponse {
                transaction_identifier: Some(TransactionIdentifier { hash: "tx1".into() }),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::TxIdentifierResponseIsNil.into()),
        },
        TestCase {
            name: "invalid transaction identifier",
            payload: Some(Default::default()),
            criteria: Some(BlockError::TxIdentifierIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| transaction_identifier_response(t.as_ref()))
}

#[test]
fn test_construction_combine_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(UncheckedConstructionCombineResponse {
                signed_transaction: "signed tx".into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::ConstructionCombineResponseIsNil.into()),
        },
        TestCase {
            name: "empty signed transaction",
            payload: Some(Default::default()),
            criteria: Some(ConstructionError::SignedTxEmpty.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_combine_response(t.as_ref()))
}

#[test]
fn test_construction_derive_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(UncheckedConstructionDeriveResponse {
                account_identifier: Some(AccountIdentifier {
                    address: "addr".into(),
                    metadata: [("name".into(), "hello".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::ConstructionDeriveResponseIsNil.into()),
        },
        TestCase {
            name: "empty address",
            payload: Some(UncheckedConstructionDeriveResponse {
                metadata: [("name".into(), "hello".into())].into(),
                ..Default::default()
            }),
            criteria: Some(BlockError::AccountIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_derive_response(t.as_ref()))
}

#[derive(Default)]
struct ConstructionParseResponseTest {
    payload: Option<UncheckedConstructionParseResponse>,
    signed: bool,
}

#[test]
fn test_construction_parse_response() {
    let tests = vec![
        TestCase {
            name: "valid response",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: None,
        },
        TestCase {
            name: "duplicate signer",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: Some(UtilError::AccountArrayDuplicateAccount.into()),
        },
        TestCase {
            name: "nil response",
            payload: Some(Default::default()),
            criteria: Some(ConstructionError::ConstructionParseResponseIsNil.into()),
        },
        TestCase {
            name: "no operations",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    account_identifier_signers: vec![valid_account()],
                    metadata: [("extra".into(), "stuff".into())].into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::ConstructionParseResponseOperationsEmpty.into()),
        },
        TestCase {
            name: "invalid operation ordering",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![Some(UncheckedOperation {
                        operation_identifier: Some(UncheckedOperationIdentifier {
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
            criteria: Some(BlockError::OperationIdentifierIndexOutOfOrder.into()),
        },
        TestCase {
            name: "no signers",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: Some(
                ConstructionError::ConstructionParseResponseSignersEmptyOnSignedTx.into(),
            ),
        },
        TestCase {
            name: "empty account identifier signer",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: Some(BlockError::AccountAddrMissing.into()),
        },
        TestCase {
            name: "invalid signer unsigned",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: Some(
                ConstructionError::ConstructionParseResponseSignersNonEmptyOnUnsignedTx.into(),
            ),
        },
        TestCase {
            name: "valid response unsigned",
            payload: Some(ConstructionParseResponseTest {
                payload: Some(UncheckedConstructionParseResponse {
                    operations: vec![
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 0,
                                ..Default::default()
                            }),
                            type_: "PAYMENT".into(),
                            account: valid_account(),
                            amount: valid_amount(),
                            ..Default::default()
                        }),
                        Some(UncheckedOperation {
                            operation_identifier: Some(UncheckedOperationIdentifier {
                                index: 1,
                                ..Default::default()
                            }),
                            related_operations: vec![Some(UncheckedOperationIdentifier {
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
            criteria: None,
        },
    ];

    let asserter = Asserter::new_client_with_responses(
        Some(NetworkIdentifier {
            blockchain: "HELLO".into(),
            network: "WORLD".into(),
            ..Default::default()
        }),
        Some(UncheckedNetworkStatusResponse {
            current_block_identifier: Some(UncheckedBlockIdentifier {
                index: 100,
                hash: "block 100".into(),
            }),
            genesis_block_identifier: Some(UncheckedBlockIdentifier {
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
        Some(UncheckedNetworkOptionsResponse {
            version: Some(Version {
                rosetta_version: "1.4.0".into(),
                node_version: "1.0".into(),
                ..Default::default()
            }),
            allow: Some(UncheckedAllow {
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
            payload: Some(UncheckedConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                payloads: vec![Some(UncheckedSigningPayload {
                    account_identifier: Some(AccountIdentifier {
                        address: "hello".into(),
                        ..Default::default()
                    }),
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
            }),
            criteria: None,
        },
        TestCase {
            name: "nil response",
            payload: None,
            criteria: Some(ConstructionError::ConstructionPayloadsResponseIsNil.into()),
        },
        TestCase {
            name: "empty unsigned transaction",
            payload: Some(UncheckedConstructionPayloadsResponse {
                payloads: vec![Some(UncheckedSigningPayload {
                    account_identifier: Some(AccountIdentifier {
                        address: "hello".into(),
                        ..Default::default()
                    }),
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
                ..Default::default()
            }),
            criteria: Some(ConstructionError::ConstructionPayloadsResponseUnsignedTxEmpty.into()),
        },
        TestCase {
            name: "empty signing payloads",
            payload: Some(UncheckedConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::ConstructionPayloadsResponsePayloadsEmpty.into()),
        },
        TestCase {
            name: "invalid signing payload",
            payload: Some(UncheckedConstructionPayloadsResponse {
                unsigned_transaction: "tx blob".into(),
                payloads: vec![Some(UncheckedSigningPayload {
                    bytes: "48656c6c6f20476f7068657221".into(),
                    ..Default::default()
                })],
            }),
            criteria: Some(BlockError::AccountIsNil.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| construction_payloads_response(t.as_ref()))
}

#[test]
fn test_public_key() {
    let tests = vec![
        TestCase {
            name: "valid public key",
            payload: Some(UncheckedPublicKey {
                bytes: "blah".into(),
                curve_type: UncheckedCurveType::SECP256K1.into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "zero public key",
            payload: Some(UncheckedPublicKey {
                bytes: vec![0; 4],
                curve_type: UncheckedCurveType::SECP256K1.into(),
            }),
            criteria: Some(ConstructionError::PublicKeyBytesZero.into()),
        },
        TestCase {
            name: "nil public key",
            payload: None,
            criteria: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        TestCase {
            name: "invalid bytes",
            payload: Some(UncheckedPublicKey {
                curve_type: UncheckedCurveType::SECP256K1.into(),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
        TestCase {
            name: "invalid curve",
            payload: Some(UncheckedPublicKey {
                bytes: "hello".into(),
                curve_type: "test".into(),
            }),
            criteria: Some(ConstructionError::CurveTypeNotSupported.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| public_key(t.as_ref()))
}

#[test]
fn test_signing_payload() {
    let tests = vec![
        TestCase {
            name: "valid signing payload",
            payload: Some(UncheckedSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid signing payload with signature type",
            payload: Some(UncheckedSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                signature_type: UncheckedSignatureType::ED25519.into(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "nil signing payload",
            payload: None,
            criteria: Some(ConstructionError::SigningPayloadIsNil.into()),
        },
        TestCase {
            name: "empty address",
            payload: Some(UncheckedSigningPayload {
                bytes: "blah".into(),
                ..Default::default()
            }),
            criteria: Some(BlockError::AccountIsNil.into()),
        },
        TestCase {
            name: "zero signing payload",
            payload: Some(UncheckedSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: vec![0; 4],
                ..Default::default()
            }),
            criteria: Some(ConstructionError::SigningPayloadBytesZero.into()),
        },
        TestCase {
            name: "empty bytes",
            payload: Some(UncheckedSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::SigningPayloadBytesEmpty.into()),
        },
        TestCase {
            name: "invalid signature",
            payload: Some(UncheckedSigningPayload {
                account_identifier: Some(AccountIdentifier {
                    address: "hello".into(),
                    ..Default::default()
                }),
                bytes: "blah".into(),
                signature_type: "blah".into(),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::SignatureTypeNotSupported.into()),
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
                Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    bytes: "hello".into(),
                }),
                Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
            ],
            criteria: None,
        },
        TestCase {
            name: "signature type match",
            payload: vec![Some(UncheckedSignature {
                signing_payload: Some(UncheckedSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: UncheckedSignatureType::ED25519.into(),
                bytes: "hello".into(),
            })],
            criteria: None,
        },
        TestCase {
            name: "nil signatures",
            payload: Vec::new(),
            criteria: Some(ConstructionError::SignaturesEmpty.into()),
        },
        TestCase {
            name: "empty signature",
            payload: vec![
                Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ECDSA_RECOVERY.into(),
                    bytes: "hello".into(),
                }),
                Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        signature_type: UncheckedSignatureType::ED25519.into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    ..Default::default()
                }),
            ],
            criteria: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        TestCase {
            name: "signature zero bytes",
            payload: vec![Some(UncheckedSignature {
                signing_payload: Some(UncheckedSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: UncheckedSignatureType::ED25519.into(),
                bytes: vec![0],
            })],
            criteria: Some(ConstructionError::SignatureBytesZero.into()),
        },
        TestCase {
            name: "signature type mismatch",
            payload: vec![Some(UncheckedSignature {
                signing_payload: Some(UncheckedSigningPayload {
                    account_identifier: valid_account(),
                    bytes: "blah".into(),
                    signature_type: UncheckedSignatureType::ECDSA_RECOVERY.into(),
                    ..Default::default()
                }),
                public_key: Some(valid_public_key()),
                signature_type: UncheckedSignatureType::ED25519.into(),
                bytes: "hello".into(),
            })],
            criteria: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
    ];

    TestCase::run_err_match(tests, |test| {
        signatures(&test.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
    })
}
