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

pub(crate) fn valid_public_key() -> NullablePublicKey {
    NullablePublicKey {
        bytes: "hello".into(),
        curve_type: CurveType::SECP256K1.into(),
    }
}

pub(crate) fn valid_amount() -> Option<NullableAmount> {
    Some(NullableAmount {
        value: "1000".into(),
        currency: Some(NullableCurrency {
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

pub(crate) fn valid_ops() -> Vec<Option<NullableOperation>> {
    vec![
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
            type_: "PAYMENT".into(),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
    ]
}

pub(crate) fn unsupported_type_ops() -> Vec<Option<NullableOperation>> {
    vec![
        Some(NullableOperation {
            operation_identifier: Some(OperationIdentifier {
                index: 0,
                ..Default::default()
            }),
            type_: "STAKE".into(),
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
    ]
}

pub(crate) fn invalid_ops() -> Vec<Option<NullableOperation>> {
    vec![
        Some(NullableOperation {
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
            status: Some("SUCCESS".into()),
            account: valid_account(),
            amount: valid_amount(),
            ..Default::default()
        }),
    ]
}

pub(crate) fn valid_signatures() -> Vec<Option<NullableSignature>> {
    vec![Some(NullableSignature {
        signing_payload: Some(NullableSigningPayload {
            account_identifier: valid_account(),
            bytes: "blah".into(),
            ..Default::default()
        }),
        public_key: Some(valid_public_key()),
        signature_type: SignatureType::ED25519.into(),
        bytes: "hello".into(),
    })]
}

pub(crate) fn signature_type_mismatch() -> Vec<Option<NullableSignature>> {
    vec![Some(NullableSignature {
        signing_payload: Some(NullableSigningPayload {
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

pub(crate) fn signature_type_match() -> Vec<Option<NullableSignature>> {
    vec![Some(NullableSignature {
        signing_payload: Some(NullableSigningPayload {
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

pub(crate) fn empty_signature() -> Vec<Option<NullableSignature>> {
    vec![Some(NullableSignature {
        signing_payload: Some(NullableSigningPayload {
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
            err: None,
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
            err: None,
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
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: None,
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(NullableCurrency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(NullableCurrency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            extras: false,
            err: None,
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(NullableCurrency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(NullableCurrency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::DuplicateCurrency.into()),
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        CustomAsserterTest {
            name: "nil request",
            payload: None,
            extras: false,
            err: Some(ServerError::AccountBalanceRequestIsNil.into()),
        },
        CustomAsserterTest {
            name: "missing network",
            payload: Some(NullableAccountBalanceRequest {
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        CustomAsserterTest {
            name: "missing account",
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(BlockError::AccountIsNil.into()),
        },
        CustomAsserterTest {
            name: "valid historical request",
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            extras: true,
            err: None,
        },
        CustomAsserterTest {
            name: "invalid historical request",
            payload: Some(NullableAccountBalanceRequest {
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
            payload: Some(NullableAccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported.into()),
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
            payload: Some(NullableBlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(valid_partial_block_identifier()),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request for block 0",
            payload: Some(NullableBlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: Some(PartialBlockIdentifier {
                    index: Some(genesis_block_index()),
                    ..Default::default()
                }),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "nil request",
            payload: None,
            err: Some(ServerError::BlockRequestIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing network",
            payload: Some(NullableBlockRequest {
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing block identifier",
            payload: Some(NullableBlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid PartialBlockIdentifier request",
            payload: Some(NullableBlockRequest {
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
            payload: Some(NullableBlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableBlockTransactionRequest {
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
            payload: Some(NullableBlockTransactionRequest {
                block_identifier: valid_block_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing block identifier",
            payload: Some(NullableBlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid BlockIdentifier request",
            payload: Some(NullableBlockTransactionRequest {
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
            payload: Some(NullableConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with public keys",
            payload: Some(NullableConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(NullablePublicKey {
                    bytes: "hello".into(),
                    curve_type: CurveType::SECP256K1.into(),
                })],
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionMetadataRequest {
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
            payload: Some(NullableConstructionMetadataRequest {
                options: Some(Default::default()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "missing options",
            payload: Some(NullableConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: None,
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request with public keys",
            payload: Some(NullableConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: vec![Some(NullablePublicKey {
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
            payload: Some(NullableConstructionSubmitRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "tx".into(),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionSubmitRequest {
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
            payload: Some(NullableMempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: Some(valid_transaction_identifier()),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableMempoolTransactionRequest {
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
            payload: Some(NullableMempoolTransactionRequest {
                transaction_identifier: Some(valid_transaction_identifier()),
                ..Default::default()
            }),
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid TransactionIdentifier request",
            payload: Some(NullableMempoolTransactionRequest {
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
            err: None,
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
            payload: Some(NullableNetworkRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableNetworkRequest {
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
            payload: Some(NullableConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(valid_public_key()),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionDeriveRequest {
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
            payload: Some(NullableConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(ConstructionError::PublicKeyIsNil.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty public key bytes",
            payload: Some(NullableConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: Some(NullablePublicKey {
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
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with suggested fee multiplier",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with max fee",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with suggested fee multiplier and max fee",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: vec![valid_amount()],
                suggested_fee_multiplier: positive_fee_multiplier,
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionPreprocessRequest {
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
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty operations",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: Vec::new(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "unsupported operation type",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operations",
            payload: Some(NullableConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative suggested fee multiplier",
            payload: Some(NullableConstructionPreprocessRequest {
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
            payload: Some(NullableConstructionPreprocessRequest {
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
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with public keys",
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(NullablePublicKey {
                    bytes: "hello".into(),
                    curve_type: CurveType::SECP256K1.into(),
                })],
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionPayloadsRequest {
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
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty operations",
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: vec![],
                ..Default::default()
            }),
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "unsupported operation type",
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operations",
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            }),
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid request with public keys",
            payload: Some(NullableConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: vec![Some(NullablePublicKey {
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
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: valid_signatures(),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request 2",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    bytes: "blah".into(),
                })],
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request 3",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Some(NullableSignature {
                    signing_payload: Some(NullableSigningPayload {
                        account_identifier: valid_account(),
                        bytes: "blah".into(),
                        ..Default::default()
                    }),
                    public_key: Some(valid_public_key()),
                    signature_type: SignatureType::ED25519.into(),
                    bytes: "hello".into(),
                })],
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionCombineRequest {
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
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                signatures: valid_signatures(),
                ..Default::default()
            }),
            err: Some(ServerError::ConstructionCombineRequestUnsignedTxEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "nil signatures",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                ..Default::default()
            }),
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signatures",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![],
            }),
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "signature type mismatch",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_mismatch(),
            }),
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
        AsserterRequestDefaultTest {
            name: "empty signature",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: empty_signature(),
            }),
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        AsserterRequestDefaultTest {
            name: "signature type match",
            payload: Some(NullableConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_match(),
            }),
            err: None,
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::construction_combine_request);
}

#[test]
fn test_construction_hash_request() {
    let tests = [
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(NullableConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "blah".into(),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionHashRequest {
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
            payload: Some(NullableConstructionHashRequest {
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
            payload: Some(NullableConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                transaction: "blah".into(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableConstructionParseRequest {
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
            payload: Some(NullableConstructionParseRequest {
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
            payload: Some(NullableCallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request with params",
            payload: Some(NullableCallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                parameters: indexmap!("hello".into() => "test".into()),
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableCallRequest {
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
            payload: Some(NullableCallRequest {
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
            payload: Some(NullableCallRequest {
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
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: None,
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(NullableCurrency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }),
                    Some(NullableCurrency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            }),
            extras: false,
            err: None,
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                currencies: vec![
                    Some(NullableCurrency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    });
                    2
                ],
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::DuplicateCurrency.into()),
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        CustomAsserterTest {
            name: "nil request",
            payload: None,
            extras: false,
            err: Some(ServerError::AccountCoinsRequestIsNil.into()),
        },
        CustomAsserterTest {
            name: "missing network",
            payload: Some(NullableAccountCoinsRequest {
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        CustomAsserterTest {
            name: "missing account",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            extras: false,
            err: Some(BlockError::AccountIsNil.into()),
        },
        CustomAsserterTest {
            name: "valid mempool lookup request",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                ..Default::default()
            }),
            extras: true,
            err: None,
        },
        CustomAsserterTest {
            name: "valid mempool lookup request when not enabled",
            payload: Some(NullableAccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier(),
                include_mempool: true,
                ..Default::default()
            }),
            extras: false,
            err: Some(ServerError::MempoolCoinsNotSupported.into()),
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
            payload: Some(NullableEventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableEventsBlocksRequest {
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
            payload: Some(NullableEventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                offset: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative limit",
            payload: Some(NullableEventsBlocksRequest {
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
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "valid request",
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::AND.into()),
                ..Default::default()
            }),
            err: None,
        },
        AsserterRequestDefaultTest {
            name: "invalid request wrong network",
            payload: Some(NullableSearchTransactionsRequest {
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
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                max_block: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::MaxBlockInvalid.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative offset",
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                offset: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "negative limit",
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::OR.into()),
                limit: Some(-1),
                ..Default::default()
            }),
            err: Some(ServerError::LimitIsNegative.into()),
        },
        AsserterRequestDefaultTest {
            name: "invalid operator",
            payload: Some(NullableSearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some("nor".into()),
                ..Default::default()
            }),
            err: Some(ServerError::OperatorInvalid.into()),
        },
    ];

    AsserterRequestDefaultTest::run(&tests, Asserter::search_transactions_request);
}
