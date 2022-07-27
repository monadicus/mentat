use std::path::Path;

use indexmap::indexmap;

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

pub(crate) const fn genesis_block_index() -> i64 {
    0
}

pub(crate) const fn valid_block_index() -> i64 {
    1000
}

pub(crate) fn valid_partial_block_identifier() -> PartialBlockIdentifier {
    PartialBlockIdentifier {
        index: Some(valid_block_index()),
        ..Default::default()
    }
}

pub(crate) fn valid_block_identifier() -> Option<BlockIdentifier> {
    Some(BlockIdentifier {
        index: valid_block_index(),
        hash: "block 1".into(),
    })
}

pub(crate) fn valid_transaction_identifier() -> TransactionIdentifier {
    TransactionIdentifier { hash: "tx1".into() }
}

pub(crate) fn valid_public_key() -> PublicKey {
    PublicKey {
        bytes: "hello".into(),
        curve_type: CurveType::SECP256K1.into(),
    }
}

pub(crate) fn valid_amount() -> Option<Amount> {
    Some(Amount {
        value: "1000".into(),
        currency: Some(Currency {
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

pub(crate) fn valid_ops() -> Vec<Option<Operation>> {
    vec![
        Some(Operation {
            operation_identifier: Some(OperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "PAYMENT".into(),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
        Some(Operation {
            operation_identifier: Some(OperationIdentifier {
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

pub(crate) fn unsupported_type_ops() -> Vec<Option<Operation>> {
    vec![
        Some(Operation {
            operation_identifier: Some(OperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "STAKE".into(),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
        Some(Operation {
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
    ]
}

pub(crate) fn invalid_ops() -> Vec<Option<Operation>> {
    vec![
        Some(Operation {
            operation_identifier: Some(OperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
        Some(Operation {
            operation_identifier: Some(OperationIdentifier {
                index: 1,
                ..Default::default()
            }),
            related_operations: vec![Some(OperationIdentifier {
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

pub(crate) fn valid_signatures() -> Vec<Option<Signature>> {
    vec![Some(Signature {
        signing_payload: Some(SigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: SignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn signature_type_mismatch() -> Vec<Option<Signature>> {
    vec![Some(Signature {
        signing_payload: Some(SigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: SignatureType::ECDSA_RECOVERY.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: SignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn signature_type_match() -> Vec<Option<Signature>> {
    vec![Some(Signature {
        signing_payload: Some(SigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: SignatureType::ED25519.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: SignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn empty_signature() -> Vec<Option<Signature>> {
    vec![Some(Signature {
        signing_payload: Some(SigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            signature_type: SignatureType::ED25519.into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: SignatureType::ED25519.into(),
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

struct NewWithOptionsTest {
    supported_operation_types: Vec<String>,
    supported_networks: Vec<Option<NetworkIdentifier>>,
    call_methods: Vec<String>,
}

impl NewWithOptionsTest {
    fn run(&self) -> AssertResult<Asserter> {
        Asserter::new_server(
            self.supported_operation_types.clone(),
            true,
            self.supported_networks
                .clone()
                .into_iter()
                .map(|ni| ni.ok_or(NetworkError::NetworkIdentifierIsNil))
                .collect::<Result<_, _>>()?,
            self.call_methods.clone(),
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
    let tests = [
        AsserterTest {
            name: "basic",
            payload: Some(Default::default()),
            err: None,
        },
        AsserterTest {
            name: "no call methods",
            payload: Some(NewWithOptionsTest {
                call_methods: vec![],
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterTest {
            name: "duplicate operation types",
            payload: Some(NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "PAYMENT".into()],
                ..Default::default()
            }),
            err: Some("Allow.OperationTypes contains a duplicate PAYMENT".into()),
        },
        AsserterTest {
            name: "empty operation type",
            payload: Some(NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "".into()],
                ..Default::default()
            }),
            err: Some("Allow.OperationTypes has an empty string".into()),
        },
        AsserterTest {
            name: "duplicate network identifier",
            payload: Some(NewWithOptionsTest {
                supported_networks: vec![valid_network_identifier(), valid_network_identifier()],
                ..Default::default()
            }),
            err: Some(ServerError::SupportedNetworksDuplicate.into()),
        },
        AsserterTest {
            name: "nil network identifier",
            payload: Some(NewWithOptionsTest {
                supported_networks: vec![valid_network_identifier(), None],
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "no supported networks",
            payload: Some(NewWithOptionsTest {
                supported_networks: vec![],
                ..Default::default()
            }),
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
    ];

    AsserterTest::run(&tests, |t| t.unwrap().run());
}

#[test]
fn test_supported_networks() {
    let tests = [
        AsserterTest {
            name: "valid networks",
            payload: Some(vec![valid_network_identifier(), wrong_network_identifier()]),
            ..Default::default()
        },
        AsserterTest {
            name: "no valid networks",
            payload: Some(Default::default()),
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
        AsserterTest {
            name: "invalid networks",
            payload: Some(vec![Some(NetworkIdentifier {
                blockchain: "blah".into(),
                network: "".into(),
                sub_network_identifier: None,
            })]),
            err: Some(NetworkError::NetworkIdentifierNetworkMissing.into()),
        },
        AsserterTest {
            name: "duplicate networks",
            payload: Some(vec![valid_network_identifier(), valid_network_identifier()]),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::SupportedNetworksDuplicate,
                    valid_network_identifier()
                )
                .into(),
            ),
        },
    ];

    // TODO: remove Some
    AsserterTest::run(&tests, |test| supported_networks(test.unwrap()));
}

#[test]
fn test_account_balance_request() {
    let tests = [
        CustomAsserterTest {
            name: "valid request",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(Currency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: Some(AccountBalanceRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::AccountBalanceRequestIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing network",
            payload: Some(AccountBalanceRequest {
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing account",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid historical request",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            extras: true,
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid historical request",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                block_identifier: Some(PartialBlockIdentifier::default()),
                ..Default::default()
            }),
            extras: true,
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
        CustomAsserterTest {
            name: "valid historical request when not enabled",
            payload: Some(AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            err: Some(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported.into()),
            ..Default::default()
        },
    ];

    let asserter = |allow_historical: &bool| {
        Asserter::new_server(
            vec!["PAYMENT".into()],
            *allow_historical,
            vec![valid_network_identifier().unwrap()],
            vec![],
            false,
            None,
        )
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(&tests, asserter, Asserter::account_balance_request);
}

#[test]
fn test_block_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request for block 0",
            payload: Some(BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(PartialBlockIdentifier {
                    index: Some(genesis_block_index()),
                    ..Default::default()
                }),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::BlockRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(BlockRequest {
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing block identifier",
            payload: Some(BlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid PartialBlockIdentifier request",
            payload: Some(BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(PartialBlockIdentifier::default()),
            }),
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::block_request);
}

#[test]
fn test_block_transaction_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(BlockTransactionRequest {
                network_identifier: wrong_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::BlockTransactionRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(BlockTransactionRequest {
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing block identifier",
            payload: Some(BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid BlockIdentifier request",
            payload: Some(BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(BlockIdentifier::default()),
                ..Default::default()
            }),
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::block_transaction_request);
}

#[test]
fn test_construction_metadata_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with public keys",
            payload: Some(ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(PublicKey {
                    bytes: "hello".into(),
                    curve_type: CurveType::SECP256K1.into(),
                })],
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionMetadataRequest {
                network_identifier: wrong_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            }),
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionMetadataRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(ConstructionMetadataRequest {
                options: Some(Default::default()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing options",
            payload: Some(ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: None,
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request with public keys",
            payload: Some(ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(PublicKey {
                    curve_type: CurveType::SECP256K1.into(),
                    ..Default::default()
                })],
            }),
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_metadata_request);
}

#[test]
fn test_construction_submit_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionSubmitRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "tx".into(),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionSubmitRequest {
                network_identifier: wrong_network_identifier(),
                signed_transaction: "tx".into(),
            }),
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionSubmitRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty tx",
            payload: Some(Default::default()),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_submit_request);
}

#[test]
fn test_mempool_transaction_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(MempoolTransactionRequest {
                network_identifier: wrong_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::MempoolTransactionRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(MempoolTransactionRequest {
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid TransactionIdentifier request",
            payload: Some(MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(Default::default()),
            }),
            err: Some(BlockError::TxIdentifierHashMissing.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::mempool_transaction_request);
}

#[test]
fn test_metadata_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(Default::default()),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::MetadataRequestIsNil.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::metadata_request);
}

#[test]
fn test_network_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(NetworkRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NetworkRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::NetworkRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(Default::default()),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::network_request);
}

#[test]
fn test_construction_derive_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(valid_public_key()),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionDeriveRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionDeriveRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil public key",
            payload: Some(ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty public key bytes",
            payload: Some(ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(PublicKey {
                    curve_type: CurveType::SECP256K1.into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_derive_request);
}

#[test]
fn test_construction_preprocess_request() {
    let positive_fee_multiplier = Some(1.1f64);
    let negative_fee_multiplier = Some(-1.1f64);

    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with suggested fee multiplier",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with max fee",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with suggested fee multiplier and max fee",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: vec![valid_amount()],
                suggested_fee_multiplier: positive_fee_multiplier,
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionPreprocessRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil operations",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty operations",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: Vec::new(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "unsupported operation type",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operations",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative suggested fee multiplier",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                suggested_fee_multiplier: negative_fee_multiplier,
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {}",
                    ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
                    negative_fee_multiplier.unwrap()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "max fee with duplicate currency",
            payload: Some(ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: vec![valid_amount(), valid_amount()],
                ..Default::default()
            }),
            err: Some(
                format!(
                    "currency {:?} used multiple times",
                    valid_amount().unwrap().currency
                )
                .into(),
            ),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_preprocess_request);
}

#[test]
fn test_construction_payloads_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with public keys",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(PublicKey {
                    bytes: "hello".into(),
                    curve_type: CurveType::SECP256K1.into(),
                })],
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionPayloadsRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil operations",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty operations",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: vec![],
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "unsupported operation type",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operations",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid request with public keys",
            payload: Some(ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(PublicKey {
                    curve_type: CurveType::SECP256K1.into(),
                    ..Default::default()
                })],
            }),
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_payload_request);
}

#[test]
fn test_construction_combine_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: valid_signatures(),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request 2",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(Signature {
                    signing_payload: Some(SigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    bytes: "blah".into(),
                })],
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request 3",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(Signature {
                    signing_payload: Some(SigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    bytes: "hello".into(),
                })],
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionCombineRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty unsigned transaction",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                signatures: valid_signatures(),
                ..Default::default()
            }),
            err: Some(ServerError::ConstructionCombineRequestUnsignedTxEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil signatures",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signatures",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![],
            }),
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "signature type mismatch",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_mismatch(),
            }),
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signature",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: empty_signature(),
            }),
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "signature type match",
            payload: Some(ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_match(),
            }),
            ..Default::default()
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_combine_request);
}

#[test]
fn test_construction_hash_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "blah".into(),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionHashRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionHashRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signed transaction",
            payload: Some(ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::ConstructionHashRequestSignedTxEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_hash_request);
}

#[test]
fn test_construction_parse_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                transaction: "blah".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(ConstructionParseRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::ConstructionParseRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signed transaction",
            payload: Some(ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::ConstructionParseRequestEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_parse_request);
}

#[test]
fn test_call_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request with params",
            payload: Some(CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                parameters: indexmap!("hello".into() => "test".into()),
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(CallRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "unsupported method",
            payload: Some(CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_debug".into(),
                ..Default::default()
            }),
            err: Some(ServerError::CallMethodUnsupported.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::CallRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty method",
            payload: Some(CallRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::CallMethodEmpty.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::call_request);
}

#[test]
fn test_account_coins_request() {
    let tests = [
        CustomAsserterTest {
            name: "valid request",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(Currency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    });
                    2
                ],
                ..Default::default()
            }),
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: Some(AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::AccountCoinsRequestIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing network",
            payload: Some(AccountCoinsRequest {
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing account",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid mempool lookup request",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: true,
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid mempool lookup request when not enabled",
            payload: Some(AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                include_mempool: true,
                ..Default::default()
            }),
            err: Some(ServerError::MempoolCoinsNotSupported.into()),
            ..Default::default()
        },
    ];

    let asserter = |allow_mempool: &bool| {
        Asserter::new_server(
            vec!["PAYMENT".into()],
            true,
            vec![valid_network_identifier().unwrap()],
            vec![],
            *allow_mempool,
            None,
        )
        .unwrap()
    };

    CustomAsserterTest::custom_asserter_tests(&tests, asserter, Asserter::account_coins_request);
}

#[test]
fn test_event_blocks_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(EventsBlocksRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::EventsBlocksRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative offset",
            payload: Some(EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                offset: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative limit",
            payload: Some(EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                limit: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::LimitIsNegative.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::events_block_request);
}

#[test]
fn test_search_transactions_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request no operator",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::AND.into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(SearchTransactionsRequest {
                network_identifier: wrong_network_identifier(),
                operator: Some(Operator::OR.into()),
                ..Default::default()
            }),
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::SearchTransactionsRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative max block",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                max_block: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::MaxBlockInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative offset",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                offset: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative limit",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                limit: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::LimitIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operator",
            payload: Some(SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some("nor".into()),
                ..Default::default()
            }),
            err: Some(ServerError::OperatorInvalid.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::search_transactions_request);
}
