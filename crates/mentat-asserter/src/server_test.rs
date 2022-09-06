use super::*;

pub(crate) fn valid_network_identifier() -> Option<NetworkIdentifier> {
    Some(NetworkIdentifier {
        blockchain: "Bitcoin".into(),
        network: "Mainnet".into(),
        sub_network_identifier: None,
    })
}

pub(crate) fn wrong_network_identifier() -> Option<NetworkIdentifier> {
    Some(NetworkIdentifier {
        blockchain: "Bitcoin".into(),
        ..Default::default()
    })
}

pub(crate) fn missing_network_identifier() -> Option<NetworkIdentifier> {
    Some(NetworkIdentifier {
        blockchain: "Bitcoin".into(),
        network: "Testnet".into(),
        sub_network_identifier: None,
    })
}

pub(crate) fn valid_account_identifier() -> Option<AccountIdentifier> {
    Some(AccountIdentifier {
        address: "acct1".into(),
        ..Default::default()
    })
}

pub(crate) const fn genesis_block_index() -> isize {
    0
}

pub(crate) const fn valid_block_index() -> isize {
    1000
}

pub(crate) fn valid_partial_block_identifier() -> UncheckedPartialBlockIdentifier {
    UncheckedPartialBlockIdentifier {
        index: Some(valid_block_index()),
        ..Default::default()
    }
}

pub(crate) fn valid_block_identifier() -> Option<UncheckedBlockIdentifier> {
    Some(UncheckedBlockIdentifier {
        index: valid_block_index(),
        hash: "block 1".into(),
    })
}

pub(crate) fn empty_block_identifier() -> Option<UncheckedBlockIdentifier> {
    Some(UncheckedBlockIdentifier {
        ..Default::default()
    })
}

pub(crate) fn valid_transaction_identifier() -> TransactionIdentifier {
    TransactionIdentifier { hash: "tx1".into() }
}

pub(crate) fn valid_public_key() -> UncheckedPublicKey {
    UncheckedPublicKey {
        bytes: "hello".into(),
        curve_type: UncheckedCurveType::SECP256K1.into(),
    }
}

pub(crate) fn missing_bytes_public_key() -> UncheckedPublicKey {
    UncheckedPublicKey {
        curve_type: UncheckedCurveType::SECP256K1.into(),
        ..Default::default()
    }
}

pub(crate) fn valid_amount() -> Option<UncheckedAmount> {
    Some(UncheckedAmount {
        value: "1000".into(),
        currency: Some(UncheckedCurrency {
            symbol: "BTC".into(),
            decimals: 8,
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub(crate) fn valid_account() -> Option<AccountIdentifier> {
    Some(AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    })
}

pub(crate) fn valid_ops() -> Vec<Option<UncheckedOperation>> {
    vec![
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
            type_: "PAYMENT".into(),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
    ]
}

pub(crate) fn unsupported_type_ops() -> Vec<Option<UncheckedOperation>> {
    vec![
        Some(UncheckedOperation {
            operation_identifier: Some(UncheckedOperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "STAKE".into(),
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
    ]
}

pub(crate) fn invalid_ops() -> Vec<Option<UncheckedOperation>> {
    vec![
        Some(UncheckedOperation {
            operation_identifier: Some(UncheckedOperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
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
            status: Some("SUCCESS".into()),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
    ]
}

pub(crate) fn valid_signatures() -> Vec<Option<UncheckedSignature>> {
    vec![Some(UncheckedSignature {
        signing_payload: Some(UncheckedSigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: UncheckedSignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn signature_type_mismatch() -> Vec<Option<UncheckedSignature>> {
    vec![Some(UncheckedSignature {
        signing_payload: Some(UncheckedSigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: UncheckedSignatureType::ECDSA_RECOVERY.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: UncheckedSignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn signature_type_match() -> Vec<Option<UncheckedSignature>> {
    vec![Some(UncheckedSignature {
        signing_payload: Some(UncheckedSigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: UncheckedSignatureType::ED25519.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: UncheckedSignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn empty_signature() -> Vec<Option<UncheckedSignature>> {
    vec![Some(UncheckedSignature {
        signing_payload: Some(UncheckedSigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: UncheckedSignatureType::ED25519.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: UncheckedSignatureType::ED25519.into(),
        ..Default::default()
    })]
}

pub(crate) fn request_asserter() -> Asserter {
    Asserter::new_server(
        vec!["PAYMENT".into()],
        true,
        vec![valid_network_identifier().unwrap()],
        vec!["eth_call".into()],
        false,
        None,
    )
    .unwrap()
}

fn network_identifier_not_supported() -> Option<AsserterError> {
    Some(
        format!(
            "network identifier {:?} is not supported: {}",
            wrong_network_identifier(),
            ServerError::RequestedNetworkNotSupported,
        )
        .into(),
    )
}

fn null_network_identifier_is_invalid() -> Option<AsserterError> {
    Some(
        format!(
            "network identifier {:?} is invalid: {}",
            Option::<()>::None,
            NetworkError::NetworkIdentifierIsNil
        )
        .into(),
    )
}

struct NewWithOptionsTest {
    supported_operation_types: Vec<String>,
    supported_networks: Vec<Option<NetworkIdentifier>>,
    call_methods: Vec<String>,
}

impl NewWithOptionsTest {
    fn run(self) -> AssertResult<Asserter> {
        Asserter::new_server(
            self.supported_operation_types,
            true,
            self.supported_networks
                .into_iter()
                .map(|ni| ni.ok_or(NetworkError::NetworkIdentifierIsNil))
                .collect::<Result<_, _>>()?,
            self.call_methods,
            false,
            None,
        )
    }
}

impl Default for NewWithOptionsTest {
    fn default() -> Self {
        Self {
            supported_operation_types: vec!["PAYMENT".into()],
            supported_networks: vec![valid_network_identifier()],
            call_methods: vec!["eth_call".into()],
        }
    }
}

#[test]
fn test_new_with_options() {
    let tests = vec![
        TestCase {
            name: "basic",
            payload: Default::default(),
            criteria: None,
        },
        TestCase {
            name: "no call methods",
            payload: NewWithOptionsTest {
                call_methods: vec![],
                ..Default::default()
            },
            criteria: None,
        },
        TestCase {
            name: "duplicate operation types",
            payload: NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "PAYMENT".into()],
                ..Default::default()
            },
            criteria: Some("Allow.OperationTypes contains a duplicate PAYMENT".into()),
        },
        TestCase {
            name: "empty operation type",
            payload: NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "".into()],
                ..Default::default()
            },
            criteria: Some("Allow.OperationTypes has an empty string".into()),
        },
        TestCase {
            name: "duplicate network identifier",
            payload: NewWithOptionsTest {
                supported_networks: vec![valid_network_identifier(), valid_network_identifier()],
                ..Default::default()
            },
            criteria: Some(ServerError::SupportedNetworksDuplicate.into()),
        },
        TestCase {
            name: "nil network identifier",
            payload: NewWithOptionsTest {
                supported_networks: vec![valid_network_identifier(), None],
                ..Default::default()
            },
            criteria: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        TestCase {
            name: "no supported networks",
            payload: NewWithOptionsTest {
                supported_networks: vec![],
                ..Default::default()
            },
            criteria: Some(ServerError::NoSupportedNetworks.into()),
        },
    ];

    TestCase::run_err_match(tests, NewWithOptionsTest::run);
}

#[test]
fn test_supported_networks() {
    let tests = vec![
        TestCase {
            name: "valid networks",
            payload: vec![valid_network_identifier(), wrong_network_identifier()],
            criteria: None,
        },
        TestCase {
            name: "no valid networks",
            payload: Default::default(),
            criteria: Some(ServerError::NoSupportedNetworks.into()),
        },
        TestCase {
            name: "invalid network",
            payload: vec![Some(NetworkIdentifier {
                blockchain: "blah".into(),
                network: "".into(),
                sub_network_identifier: None,
            })],
            criteria: Some(
                format!(
                    "network identifier {:?} is invalid: {}",
                    missing_network_identifier(),
                    NetworkError::NetworkIdentifierNetworkMissing
                )
                .into(),
            ),
        },
        TestCase {
            name: "duplicate networks",
            payload: vec![valid_network_identifier(), valid_network_identifier()],
            criteria: Some(
                format!(
                    "network identifier {:?} is invalid: {}",
                    valid_network_identifier(),
                    ServerError::SupportedNetworksDuplicate,
                )
                .into(),
            ),
        },
    ];

    TestCase::run_err_match(tests, |test| supported_networks(&test));
}

#[test]
fn test_account_balance_request() {
    let asserter = |allow_historical: bool| {
        Asserter::new_server(
            vec!["PAYMENT".into()],
            allow_historical,
            vec![valid_network_identifier().unwrap()],
            vec![],
            false,
            None,
        )
        .unwrap()
    };

    let tests = vec![
        TestCase {
            name: "valid request",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid request with currencies",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    currencies: vec![
                        Some(UncheckedCurrency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            ..Default::default()
                        }),
                        Some(UncheckedCurrency {
                            symbol: "ETH".into(),
                            decimals: 18,
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid request with duplicate currencies",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    currencies: vec![
                        Some(UncheckedCurrency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            ..Default::default()
                        }),
                        Some(UncheckedCurrency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            },
            criteria: Some(ServerError::DuplicateCurrency.into()),
        },
        TestCase {
            name: "invalid request wrong network",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: wrong_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        TestCase {
            name: "nil request",
            payload: MethodPayload {
                caller: asserter(false),
                payload: None,
            },
            criteria: Some(ServerError::AccountBalanceRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        TestCase {
            name: "missing account",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::AccountIsNil.into()),
        },
        TestCase {
            name: "valid historical request",
            payload: MethodPayload {
                caller: asserter(true),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    block_identifier: Some(valid_partial_block_identifier()),
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "invalid historical request",
            payload: MethodPayload {
                caller: asserter(true),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    block_identifier: Some(UncheckedPartialBlockIdentifier::default()),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
        TestCase {
            name: "valid historical request when not enabled",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountBalanceRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    block_identifier: Some(valid_partial_block_identifier()),
                    ..Default::default()
                }),
            },
            criteria: Some(
                ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported.into(),
            ),
        },
    ];

    TestCase::run_err_match(tests, |t| {
        t.caller.account_balance_request(t.payload.as_ref())
    });
}

#[test]
fn test_block_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedBlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request for block 0",
            payload: Some(UncheckedBlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(UncheckedPartialBlockIdentifier {
                    index: Some(genesis_block_index()),
                    ..Default::default()
                }),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedBlockRequest {
                network_identifier: wrong_network_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::BlockRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: Some(UncheckedBlockRequest {
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            criteria: null_network_identifier_is_invalid(),
        },
        TestCase {
            name: "missing block identifier",
            payload: Some(UncheckedBlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        TestCase {
            name: "invalid PartialBlockIdentifier request",
            payload: Some(UncheckedBlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(UncheckedPartialBlockIdentifier::default()),
            }),
            criteria: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.block_request(t.as_ref()));
}

#[test]
fn test_block_transaction_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedBlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedBlockTransactionRequest {
                network_identifier: wrong_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::BlockTransactionRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: Some(UncheckedBlockTransactionRequest {
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            criteria: null_network_identifier_is_invalid(),
        },
        TestCase {
            name: "missing block identifier",
            payload: Some(UncheckedBlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            criteria: Some(
                format!(
                    "block identifier {:?} is invalid: {}",
                    Option::<()>::None,
                    BlockError::BlockIdentifierIsNil
                )
                .into(),
            ),
        },
        TestCase {
            name: "invalid BlockIdentifier request",
            payload: Some(UncheckedBlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(UncheckedBlockIdentifier::default()),
                ..Default::default()
            }),
            criteria: Some(
                format!(
                    "block identifier {:?} is invalid: {}",
                    empty_block_identifier(),
                    BlockError::BlockIdentifierHashMissing
                )
                .into(),
            ),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.block_transaction_request(t.as_ref()));
}

#[test]
fn test_construction_metadata_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with public keys",
            payload: Some(UncheckedConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(UncheckedPublicKey {
                    bytes: "hello".into(),
                    curve_type: UncheckedCurveType::SECP256K1.into(),
                })],
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionMetadataRequest {
                network_identifier: wrong_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionMetadataRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: Some(UncheckedConstructionMetadataRequest {
                options: Some(Default::default()),
                ..Default::default()
            }),
            criteria: null_network_identifier_is_invalid(),
        },
        TestCase {
            name: "missing options",
            payload: Some(UncheckedConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: None,
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request with public keys",
            payload: Some(UncheckedConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(UncheckedPublicKey {
                    curve_type: UncheckedCurveType::SECP256K1.into(),
                    ..Default::default()
                })],
            }),
            criteria: Some(
                format!(
                    "public key {:?} is invalid: {}",
                    missing_bytes_public_key(),
                    ConstructionError::PublicKeyBytesEmpty
                )
                .into(),
            ),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| {
        asserter.construction_metadata_request(t.as_ref())
    });
}

#[test]
fn test_construction_submit_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionSubmitRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "tx".into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionSubmitRequest {
                network_identifier: wrong_network_identifier(),
                signed_transaction: "tx".into(),
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionSubmitRequestIsNil.into()),
        },
        TestCase {
            name: "empty tx",
            payload: Some(Default::default()),
            criteria: null_network_identifier_is_invalid(),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_submit_request(t.as_ref()));
}

#[test]
fn test_mempool_transaction_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedMempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedMempoolTransactionRequest {
                network_identifier: wrong_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::MempoolTransactionRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: Some(UncheckedMempoolTransactionRequest {
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            criteria: null_network_identifier_is_invalid(),
        },
        TestCase {
            name: "invalid TransactionIdentifier request",
            payload: Some(UncheckedMempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(Default::default()),
            }),
            criteria: Some(BlockError::TxIdentifierHashMissing.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.mempool_transaction_request(t.as_ref()));
}

#[test]
fn test_metadata_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(Default::default()),
            criteria: None,
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::MetadataRequestIsNil.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.metadata_request(t.as_ref()));
}

#[test]
fn test_network_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedNetworkRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedNetworkRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::NetworkRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: Some(Default::default()),
            criteria: null_network_identifier_is_invalid(),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.network_request(t.as_ref()));
}

#[test]
fn test_construction_derive_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(valid_public_key()),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionDeriveRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionDeriveRequestIsNil.into()),
        },
        TestCase {
            name: "nil public key",
            payload: Some(UncheckedConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        TestCase {
            name: "empty public key bytes",
            payload: Some(UncheckedConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(UncheckedPublicKey {
                    curve_type: UncheckedCurveType::SECP256K1.into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_derive_request(t.as_ref()));
}

#[test]
fn test_construction_preprocess_request() {
    let positive_fee_multiplier = Some(1.1f64);
    let negative_fee_multiplier = Some(-1.1f64);

    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with suggested fee multiplier",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with max fee",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with suggested fee multiplier and max fee",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: vec![valid_amount()],
                suggested_fee_multiplier: positive_fee_multiplier,
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionPreprocessRequestIsNil.into()),
        },
        TestCase {
            name: "nil operations",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(BlockError::NoOperationsForConstruction.into()),
        },
        TestCase {
            name: "empty operations",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: Vec::new(),
                ..Default::default()
            }),
            criteria: Some(BlockError::NoOperationsForConstruction.into()),
        },
        TestCase {
            name: "unsupported operation type",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            criteria: Some(BlockError::OperationTypeInvalid.into()),
        },
        TestCase {
            name: "invalid operations",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            criteria: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        TestCase {
            name: "negative suggested fee multiplier",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                suggested_fee_multiplier: negative_fee_multiplier,
                ..Default::default()
            }),
            criteria: Some(
                ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg.into(),
            ),
        },
        TestCase {
            name: "max fee with duplicate currency",
            payload: Some(UncheckedConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: vec![valid_amount(), valid_amount()],
                ..Default::default()
            }),
            criteria: Some(AccountBalanceError::CurrencyUsedMultipleTimes.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| {
        asserter.construction_preprocess_request(t.as_ref())
    });
}

#[test]
fn test_construction_payloads_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with public keys",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(UncheckedPublicKey {
                    bytes: "hello".into(),
                    curve_type: UncheckedCurveType::SECP256K1.into(),
                })],
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionPayloadsRequestIsNil.into()),
        },
        TestCase {
            name: "nil operations",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(BlockError::NoOperationsForConstruction.into()),
        },
        TestCase {
            name: "empty operations",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: vec![],
                ..Default::default()
            }),
            criteria: Some(BlockError::NoOperationsForConstruction.into()),
        },
        TestCase {
            name: "unsupported operation type",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            criteria: Some(BlockError::OperationTypeInvalid.into()),
        },
        TestCase {
            name: "invalid operations",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            criteria: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        TestCase {
            name: "invalid request with public keys",
            payload: Some(UncheckedConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(UncheckedPublicKey {
                    curve_type: UncheckedCurveType::SECP256K1.into(),
                    ..Default::default()
                })],
            }),
            criteria: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_payload_request(t.as_ref()));
}

#[test]
fn test_construction_combine_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: valid_signatures(),
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request 2",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    bytes: "blah".into(),
                })],
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request 3",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(UncheckedSignature {
                    signing_payload: Some(UncheckedSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: UncheckedSignatureType::ED25519.into(),
                    bytes: "hello".into(),
                })],
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        },
        TestCase {
            name: "empty unsigned transaction",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                signatures: valid_signatures(),
                ..Default::default()
            }),
            criteria: Some(ServerError::ConstructionCombineRequestUnsignedTxEmpty.into()),
        },
        TestCase {
            name: "nil signatures",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                ..Default::default()
            }),
            criteria: Some(ConstructionError::SignaturesEmpty.into()),
        },
        TestCase {
            name: "empty signatures",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![],
            }),
            criteria: Some(ConstructionError::SignaturesEmpty.into()),
        },
        TestCase {
            name: "signature type mismatch",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_mismatch(),
            }),
            criteria: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
        TestCase {
            name: "empty signature",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: empty_signature(),
            }),
            criteria: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        TestCase {
            name: "signature type match",
            payload: Some(UncheckedConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_match(),
            }),
            criteria: None,
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_combine_request(t.as_ref()));
}

#[test]
fn test_construction_hash_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "blah".into(),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionHashRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionHashRequestIsNil.into()),
        },
        TestCase {
            name: "empty signed transaction",
            payload: Some(UncheckedConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(ServerError::ConstructionHashRequestSignedTxEmpty.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_hash_request(t.as_ref()));
}

#[test]
fn test_construction_parse_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                transaction: "blah".into(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedConstructionParseRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::ConstructionParseRequestIsNil.into()),
        },
        TestCase {
            name: "empty signed transaction",
            payload: Some(UncheckedConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(ServerError::ConstructionParseRequestEmpty.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.construction_parse_request(t.as_ref()));
}

#[test]
fn test_call_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedCallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request with params",
            payload: Some(UncheckedCallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                parameters: indexmap!("hello".into() => "test".into()),
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedCallRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "unsupported method",
            payload: Some(UncheckedCallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_debug".into(),
                ..Default::default()
            }),
            criteria: Some(ServerError::CallMethodUnsupported.into()),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::CallRequestIsNil.into()),
        },
        TestCase {
            name: "empty method",
            payload: Some(UncheckedCallRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: Some(ServerError::CallMethodEmpty.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.call_request(t.as_ref()));
}

#[test]
fn test_account_coins_request() {
    let asserter = |allow_mempool: bool| {
        Asserter::new_server(
            vec!["PAYMENT".into()],
            true,
            vec![valid_network_identifier().unwrap()],
            vec![],
            allow_mempool,
            None,
        )
        .unwrap()
    };

    let tests = vec![
        TestCase {
            name: "valid request",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid request with currencies",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    currencies: vec![
                        Some(UncheckedCurrency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            ..Default::default()
                        }),
                        Some(UncheckedCurrency {
                            symbol: "ETH".into(),
                            decimals: 18,
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid request with duplicate currencies",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    currencies: vec![
                        Some(UncheckedCurrency {
                            symbol: "BTC".into(),
                            decimals: 8,
                            ..Default::default()
                        });
                        2
                    ],
                    ..Default::default()
                }),
            },
            criteria: Some(ServerError::DuplicateCurrency.into()),
        },
        TestCase {
            name: "invalid request wrong network",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: wrong_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        TestCase {
            name: "nil request",
            payload: MethodPayload {
                caller: asserter(false),
                payload: None,
            },
            criteria: Some(ServerError::AccountCoinsRequestIsNil.into()),
        },
        TestCase {
            name: "missing network",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        TestCase {
            name: "missing account",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    ..Default::default()
                }),
            },
            criteria: Some(BlockError::AccountIsNil.into()),
        },
        TestCase {
            name: "valid mempool lookup request",
            payload: MethodPayload {
                caller: asserter(true),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    ..Default::default()
                }),
            },
            criteria: None,
        },
        TestCase {
            name: "valid mempool lookup request when not enabled",
            payload: MethodPayload {
                caller: asserter(false),
                payload: Some(UncheckedAccountCoinsRequest {
                    network_identifier: valid_network_identifier(),
                    account_identifier: valid_account_identifier(),
                    include_mempool: true,
                    ..Default::default()
                }),
            },
            criteria: Some(ServerError::MempoolCoinsNotSupported.into()),
        },
    ];

    TestCase::run_err_match(tests, |t| {
        t.caller.account_coins_request(t.payload.as_ref())
    });
}

#[test]
fn test_event_blocks_request() {
    let tests = vec![
        TestCase {
            name: "valid request",
            payload: Some(UncheckedEventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedEventsBlocksRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::EventsBlocksRequestIsNil.into()),
        },
        TestCase {
            name: "negative offset",
            payload: Some(UncheckedEventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                offset: Some(-1),
                ..Default::default()
            }),
            criteria: Some(ServerError::OffsetIsNegative.into()),
        },
        TestCase {
            name: "negative limit",
            payload: Some(UncheckedEventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                limit: Some(-1),
                ..Default::default()
            }),
            criteria: Some(ServerError::LimitIsNegative.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.events_block_request(t.as_ref()));
}

#[test]
fn test_search_transactions_request() {
    let tests = vec![
        TestCase {
            name: "valid request no operator",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "valid request",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(UncheckedOperator::AND.into()),
                ..Default::default()
            }),
            criteria: None,
        },
        TestCase {
            name: "invalid request wrong network",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: wrong_network_identifier(),
                operator: Some(UncheckedOperator::OR.into()),
                ..Default::default()
            }),
            criteria: network_identifier_not_supported(),
        },
        TestCase {
            name: "nil request",
            payload: None,
            criteria: Some(ServerError::SearchTransactionsRequestIsNil.into()),
        },
        TestCase {
            name: "negative max block",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(UncheckedOperator::OR.into()),
                max_block: Some(-1),
                ..Default::default()
            }),
            criteria: Some(ServerError::MaxBlockInvalid.into()),
        },
        TestCase {
            name: "negative offset",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(UncheckedOperator::OR.into()),
                offset: Some(-1),
                ..Default::default()
            }),
            criteria: Some(ServerError::OffsetIsNegative.into()),
        },
        TestCase {
            name: "negative limit",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(UncheckedOperator::OR.into()),
                limit: Some(-1),
                ..Default::default()
            }),
            criteria: Some(ServerError::LimitIsNegative.into()),
        },
        TestCase {
            name: "invalid operator",
            payload: Some(UncheckedSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some("nor".into()),
                ..Default::default()
            }),
            criteria: Some(ServerError::OperatorInvalid.into()),
        },
    ];

    let asserter = request_asserter();

    TestCase::run_err_match(tests, |t| asserter.search_transactions_request(t.as_ref()));
}
