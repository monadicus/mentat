use std::path::Path;

use indexmap::indexmap;

use super::test_utils::{
    custom_request_asserter_tests, default_request_asserter_tests, non_asserter_tests,
    AsserterTest, CustomAsserterTest,
};
use crate::{
    asserter::{
        asserter_tools::RequestAsserter,
        errors::{BlockError, ConstructionError, NetworkError, ServerError},
        server::supported_networks,
    },
    types::{
        AccountBalanceRequest, AccountCoinsRequest, AccountIdentifier, Amount, BlockIdentifier,
        BlockRequest, BlockTransactionRequest, CallRequest, ConstructionCombineRequest,
        ConstructionDeriveRequest, ConstructionHashRequest, ConstructionMetadataRequest,
        ConstructionParseRequest, ConstructionPayloadsRequest, ConstructionPreprocessRequest,
        ConstructionSubmitRequest, Currency,
        CurveType::Secp256k1,
        EventsBlocksRequest, MempoolTransactionRequest, NetworkIdentifier, NetworkRequest,
        Operation, OperationIdentifier, Operator, PartialBlockIdentifier, PublicKey,
        SearchTransactionsRequest, Signature,
        SignatureType::{EcdsaRecovery, Ed25519},
        SigningPayload, TransactionIdentifier,
    },
};

pub(crate) fn valid_network_identifier() -> NetworkIdentifier {
    NetworkIdentifier {
        blockchain: "Bitcoin".into(),
        network: "Mainnet".into(),
        sub_network_identifier: None,
    }
}

pub(crate) fn wrong_network_identifier() -> NetworkIdentifier {
    NetworkIdentifier {
        blockchain: "Bitcoin".into(),
        network: "Testnet".into(),
        sub_network_identifier: None,
    }
}

pub(crate) fn valid_account_identifier() -> Option<AccountIdentifier> {
    Some(AccountIdentifier {
        address: "acct1".into(),
        ..Default::default()
    })
}

pub(crate) const fn genesis_block_index() -> u64 {
    0
}

pub(crate) const fn valid_block_index() -> u64 {
    1000
}

pub(crate) fn valid_partial_block_identifier() -> PartialBlockIdentifier {
    PartialBlockIdentifier {
        index: Some(valid_block_index()),
        ..Default::default()
    }
}

pub(crate) fn valid_block_identifier() -> BlockIdentifier {
    BlockIdentifier {
        index: valid_block_index(),
        hash: "block 1".into(),
    }
}

pub(crate) fn valid_transaction_identifier() -> TransactionIdentifier {
    TransactionIdentifier { hash: "tx1".into() }
}

pub(crate) fn valid_public_key() -> PublicKey {
    PublicKey {
        bytes: "hello".into(),
        curve_type: Secp256k1,
    }
}

pub(crate) fn valid_amount() -> Amount {
    Amount {
        value: "1000".into(),
        currency: Currency {
            symbol: "BTC".into(),
            decimals: 8,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub(crate) fn valid_account() -> AccountIdentifier {
    AccountIdentifier {
        address: "test".into(),
        ..Default::default()
    }
}

pub(crate) fn valid_ops() -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: OperationIdentifier {
                index: 0,
                ..Default::default()
            },
            type_: "PAYMENT".into(),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
        Operation {
            operation_identifier: OperationIdentifier {
                index: 1,
                ..Default::default()
            },
            type_: "PAYMENT".into(),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
    ]
}

pub(crate) fn unsupported_type_ops() -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: OperationIdentifier {
                index: 0,
                ..Default::default()
            },
            type_: "STAKE".into(),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
        Operation {
            operation_identifier: OperationIdentifier {
                index: 1,
                ..Default::default()
            },
            related_operations: Some(vec![OperationIdentifier {
                index: 0,
                ..Default::default()
            }]),
            type_: "PAYMENT".into(),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
    ]
}

pub(crate) fn invalid_ops() -> Vec<Operation> {
    vec![
        Operation {
            operation_identifier: OperationIdentifier {
                index: 0,
                ..Default::default()
            },
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
        Operation {
            operation_identifier: OperationIdentifier {
                index: 1,
                ..Default::default()
            },
            related_operations: Some(vec![OperationIdentifier {
                index: 0,
                ..Default::default()
            }]),
            type_: "PAYMENT".into(),
            status: Some("SUCCESS".into()),
            account: Some(valid_account()),
            amount: Some(valid_amount()),
            ..Default::default()
        },
    ]
}

pub(crate) fn valid_signatures() -> Vec<Signature> {
    vec![Signature {
        signing_payload: SigningPayload {
            account_identifier: Some(valid_account()),
            bytes: "blah".into(),
            ..Default::default()
        },
        public_key: valid_public_key(),
        signature_type: Ed25519,
        bytes: "hello".into(),
    }]
}

pub(crate) fn signature_type_mismatch() -> Vec<Signature> {
    vec![Signature {
        signing_payload: SigningPayload {
            account_identifier: Some(valid_account()),
            bytes: "blah".into(),
            signature_type: Some(EcdsaRecovery),
            ..Default::default()
        },
        public_key: valid_public_key(),
        signature_type: Ed25519,
        bytes: "hello".into(),
    }]
}

pub(crate) fn signature_type_match() -> Vec<Signature> {
    vec![Signature {
        signing_payload: SigningPayload {
            account_identifier: Some(valid_account()),
            bytes: "blah".into(),
            signature_type: Some(Ed25519),
            ..Default::default()
        },
        public_key: valid_public_key(),
        signature_type: Ed25519,
        ..Default::default()
    }]
}

pub(crate) fn empty_signature() -> Vec<Signature> {
    vec![Signature {
        signing_payload: SigningPayload {
            account_identifier: Some(valid_account()),
            bytes: "blah".into(),
            signature_type: Some(Ed25519),
            ..Default::default()
        },
        public_key: valid_public_key(),
        signature_type: Ed25519,
        ..Default::default()
    }]
}

pub(crate) fn request_asserter() -> RequestAsserter {
    RequestAsserter::new_server(
        vec!["PAYMENT".into()],
        true,
        vec![valid_network_identifier()],
        vec!["eth_call".into()],
        false,
        Path::new(""),
    )
    .unwrap()
}

struct NewWithOptionsTest {
    supported_operation_types: Vec<String>,
    supported_networks: Vec<NetworkIdentifier>,
    call_methods: Vec<String>,
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
            ..Default::default()
        },
        AsserterTest {
            name: "no call methods",
            payload: NewWithOptionsTest {
                call_methods: vec![],
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "duplicate operation types",
            payload: NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "PAYMENT".into()],
                ..Default::default()
            },
            err: Some("Allow.OperationTypes contains a duplicate PAYMENT".into()),
        },
        AsserterTest {
            name: "empty operation type",
            payload: NewWithOptionsTest {
                supported_operation_types: vec!["PAYMENT".into(), "".into()],
                ..Default::default()
            },
            err: Some("Allow.OperationTypes has an empty string".into()),
        },
        AsserterTest {
            name: "duplicate network identifier",
            payload: NewWithOptionsTest {
                supported_networks: vec![valid_network_identifier(), valid_network_identifier()],
                ..Default::default()
            },
            err: Some(ServerError::SupportedNetworksDuplicate.into()),
        },
        // TODO
        // "nil network identifier"=> ServerTest {
        //     supported_networks: vec!(valid_network_identifier(), todo!()),
        //     err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        //     ..Default::default()
        // },
        AsserterTest {
            name: "no supported networks",
            payload: NewWithOptionsTest {
                supported_networks: vec![],
                ..Default::default()
            },
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
    ];

    let asserter = |test: &NewWithOptionsTest| {
        RequestAsserter::new_server(
            test.supported_operation_types.clone(),
            true,
            test.supported_networks.clone(),
            test.call_methods.clone(),
            false,
            Path::new(""),
        )
    };

    non_asserter_tests(&tests, asserter);
}

#[test]
fn test_supported_networks() {
    let tests = [
        AsserterTest {
            name: "valid networks",
            payload: vec![valid_network_identifier(), wrong_network_identifier()],
            err: None,
        },
        AsserterTest {
            name: "no valid networks",
            payload: vec![],
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
        AsserterTest {
            name: "invalid networks",
            payload: vec![NetworkIdentifier {
                blockchain: "blah".into(),
                network: "".into(),
                sub_network_identifier: None,
            }],
            err: None,
        },
        AsserterTest {
            name: "duplicate networks",
            payload: vec![valid_network_identifier(), valid_network_identifier()],
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

    non_asserter_tests(&tests, |test| supported_networks(test.as_slice()));
}

#[test]
fn test_account_balance_request() {
    let tests = [
        CustomAsserterTest {
            name: "valid request",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec![
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    },
                    Currency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec![
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    },
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: AccountBalanceRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
            ..Default::default()
        },
        // TODO
        // "nil request" => AccountBalanceRequestTest {
        //     request: todo!(),
        //     err: Some(ServerError::AccountBalanceRequestIsNil.into()),
        //     ..Default::default()
        // },
        CustomAsserterTest {
            name: "missing network",
            payload: AccountBalanceRequest {
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing account",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid historical request",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            },
            extras: true,
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid historical request",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(PartialBlockIdentifier::default()),
                ..Default::default()
            },
            extras: true,
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
        CustomAsserterTest {
            name: "valid historical request when not enabled",
            payload: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            },
            err: Some(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported.into()),
            ..Default::default()
        },
    ];

    let asserter = |allow_historical: &bool| {
        RequestAsserter::new_server(
            vec!["PAYMENT".into()],
            *allow_historical,
            vec![valid_network_identifier()],
            vec![],
            false,
            Path::new(""),
        )
        .unwrap()
    };

    custom_request_asserter_tests(asserter, &tests, RequestAsserter::account_balance_request);
}

#[test]
fn test_block_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_partial_block_identifier(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request for block 0",
            payload: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: PartialBlockIdentifier {
                    index: Some(genesis_block_index()),
                    ..Default::default()
                },
            },
            ..Default::default()
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: (ServerError::BlockRequestIsNil.into()),
        //     ..Default::default()
        // },
        AsserterTest {
            name: "missing network",
            payload: BlockRequest {
                block_identifier: valid_partial_block_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "missing block identifier",
            payload: BlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "invalid PartialBlockIdentifier request",
            payload: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: PartialBlockIdentifier::default(),
            },
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::block_request);
}

#[test]
fn test_block_transaction_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: BlockTransactionRequest {
                network_identifier: wrong_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::BlockTransactionRequestIsNil.into()),
        // },
        AsserterTest {
            name: "missing network",
            payload: BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "missing block identifier",
            payload: BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "invalid BlockIdentifier request",
            payload: BlockTransactionRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: BlockIdentifier::default(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::block_transaction_request);
}

#[test]
fn test_construction_metadata_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with public keys",
            payload: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: Some(vec![PublicKey {
                    bytes: "hello".into(),
                    curve_type: Secp256k1,
                }]),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionMetadataRequest {
                network_identifier: wrong_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            },
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        // },
        AsserterTest {
            name: "missing network",
            payload: ConstructionMetadataRequest {
                options: Some(Default::default()),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "missing options",
            payload: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: None,
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request with public keys",
            payload: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: Some(vec![PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                }]),
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_metadata_request);
}

#[test]
fn test_construction_submit_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionSubmitRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "tx".into(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionSubmitRequest {
                network_identifier: wrong_network_identifier(),
                signed_transaction: "tx".into(),
            },
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionSubmitRequestIsNil.into()),
        // },
        AsserterTest {
            name: "empty tx",
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_submit_request);
}

#[test]
fn test_mempool_transaction_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: MempoolTransactionRequest {
                network_identifier: wrong_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::MempoolTransactionRequestIsNil.into()),
        // },
        AsserterTest {
            name: "missing network",
            payload: MempoolTransactionRequest {
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        AsserterTest {
            name: "invalid TransactionIdentifier request",
            payload: MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::TxIdentifierHashMissing.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::mempool_transaction_request);
}

#[test]
fn test_metadata_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            ..Default::default()
        }, // TODO
           // "nil request" => ServerTest {
           //     request: todo!(),
           //     err: Some(ServerError::MetadataRequestIsNil.into()),
           // }
    ];

    default_request_asserter_tests(&tests, RequestAsserter::metadata_request);
}

#[test]
fn test_network_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: NetworkRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: NetworkRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::NetworkRequestIsNil.into()),
        // },
        AsserterTest {
            name: "missing network",
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::network_request);
}

#[test]
fn test_construction_derive_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: valid_public_key(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionDeriveRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionDeriveRequestIsNil.into()),
        // },
        AsserterTest {
            name: "nil public key",
            payload: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        },
        AsserterTest {
            name: "empty public key bytes",
            payload: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                },
                ..Default::default()
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_derive_request);
}

#[test]
fn test_construction_preprocess_request() {
    let positive_fee_multiplier = Some(1.1f64);
    let negative_fee_multiplier = Some(-1.1f64);

    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with suggested fee multiplier",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with max fee",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with suggested fee multiplier and max fee",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: Some(vec![valid_amount()]),
                suggested_fee_multiplier: positive_fee_multiplier,
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionPreprocessRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionPreprocessRequestIsNil.into()),
        // },
        AsserterTest {
            name: "nil operations",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterTest {
            name: "empty operations",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterTest {
            name: "unsupported operation type",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterTest {
            name: "invalid operations",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterTest {
            name: "negative suggested fee multiplier",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                suggested_fee_multiplier: negative_fee_multiplier,
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
                    negative_fee_multiplier
                )
                .into(),
            ),
        },
        AsserterTest {
            name: "max fee with duplicate currency",
            payload: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: Some(vec![valid_amount(), valid_amount()]),
                ..Default::default()
            },
            err: Some(format!("currency {:?} used multiple times", valid_amount().currency).into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_preprocess_request);
}

#[test]
fn test_construction_payloads_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with public keys",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: Some(vec![PublicKey {
                    bytes: "hello".into(),
                    curve_type: Secp256k1,
                }]),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionPayloadsRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionPayloadsRequestIsNil.into()),
        // },
        AsserterTest {
            name: "nil operations",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterTest {
            name: "empty operations",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: vec![Operation::default()],
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        AsserterTest {
            name: "unsupported operation type",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        AsserterTest {
            name: "invalid operations",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        AsserterTest {
            name: "invalid request with public keys",
            payload: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: Some(vec![PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                }]),
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_payload_request);
}

#[test]
fn test_construction_combine_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: valid_signatures(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request 2",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Signature {
                    signing_payload: SigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    },
                    public_key: valid_public_key(),
                    signature_type: Ed25519,
                    bytes: "blah".into(),
                }],
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request 3",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Signature {
                    signing_payload: SigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    },
                    public_key: valid_public_key(),
                    signature_type: Ed25519,
                    bytes: "hello".into(),
                }],
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionCombineRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        // },
        AsserterTest {
            name: "empty unsigned transaction",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                signatures: valid_signatures(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionCombineRequestUnsignedTxEmpty.into()),
        },
        AsserterTest {
            name: "nil signatures",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                ..Default::default()
            },
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterTest {
            name: "empty signatures",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec![Signature::default()],
            },
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        AsserterTest {
            name: "signature type mismatch",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_mismatch(),
            },
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
        AsserterTest {
            name: "empty signature",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: empty_signature(),
            },
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        AsserterTest {
            name: "signature type match",
            payload: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_match(),
            },
            ..Default::default()
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_combine_request);
}

#[test]
fn test_construction_hash_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "blah".into(),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionHashRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionHashRequestIsNil.into()),
        // },
        AsserterTest {
            name: "empty signed transaction",
            payload: ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionHashRequestSignedTxEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_hash_request);
}

#[test]
fn test_construction_parse_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                transaction: "blah".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: ConstructionParseRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionParseRequestIsNil.into()),
        // },
        AsserterTest {
            name: "empty signed transaction",
            payload: ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionParseRequestEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::construction_parse_request);
}

#[test]
fn test_call_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request with params",
            payload: CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                parameters: indexmap!("hello".into() => "test".into()),
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: CallRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        AsserterTest {
            name: "unsupported method",
            payload: CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_debug".into(),
                ..Default::default()
            },
            err: Some(ServerError::CallMethodUnsupported.into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::CallRequestIsNil.into()),
        // },
        AsserterTest {
            name: "empty method",
            payload: CallRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::CallMethodEmpty.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::call_request);
}

#[test]
fn test_account_coins_request() {
    let tests = [
        CustomAsserterTest {
            name: "valid request",
            payload: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with currencies",
            payload: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec![
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    },
                    Currency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            },
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid request with duplicate currencies",
            payload: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec![
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    };
                    2
                ]),
                ..Default::default()
            },
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "invalid request wrong network",
            payload: AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(ServerError::RequestedNetworkNotSupported.into()),
            ..Default::default()
        },
        // TODO
        // "nil request" => AccountCoinsRequestTest {
        //     request: todo!(),
        //     err: Some(ServerError::AccountBalanceRequestIsNil.into()),
        //     ..Default::default()
        // },
        CustomAsserterTest {
            name: "missing network",
            payload: AccountCoinsRequest {
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "missing account",
            payload: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid mempool lookup request",
            payload: AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            extras: true,
            ..Default::default()
        },
        CustomAsserterTest {
            name: "valid mempool lookup request when not enabled",
            payload: AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                include_mempool: true,
                ..Default::default()
            },
            err: Some(ServerError::MempoolCoinsNotSupported.into()),
            ..Default::default()
        },
    ];

    let asserter = |allow_mempool: &bool| {
        RequestAsserter::new_server(
            vec!["PAYMENT".into()],
            true,
            vec![valid_network_identifier()],
            vec![],
            *allow_mempool,
            Path::new(""),
        )
        .unwrap()
    };

    custom_request_asserter_tests(asserter, &tests, RequestAsserter::account_coins_request);
}

#[test]
fn test_event_blocks_request() {
    let tests = [
        AsserterTest {
            name: "valid request",
            payload: EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: EventsBlocksRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::EventsBlocksRequestIsNil.into()),
        // },
        AsserterTest {
            name: "negative offset",
            payload: EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                offset: Some(-1),
                ..Default::default()
            },
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterTest {
            name: "negative limit",
            payload: EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                limit: Some(-1),
                ..Default::default()
            },
            err: Some(ServerError::LimitIsNegative.into()),
        },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::events_block_request);
}

#[test]
fn test_search_transactions_request() {
    let tests = [
        AsserterTest {
            name: "valid request no operator",
            payload: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "valid request",
            payload: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::And),
                ..Default::default()
            },
            ..Default::default()
        },
        AsserterTest {
            name: "invalid request wrong network",
            payload: SearchTransactionsRequest {
                network_identifier: wrong_network_identifier(),
                operator: Some(Operator::Or),
                ..Default::default()
            },
            err: Some(
                format!(
                    "{}: {:?}",
                    ServerError::RequestedNetworkNotSupported,
                    wrong_network_identifier()
                )
                .into(),
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::SearchTransactionsRequestIsNil.into()),
        // },
        AsserterTest {
            name: "negative max block",
            payload: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::Or),
                max_block: Some(-1),
                ..Default::default()
            },
            err: Some(ServerError::MaxBlockInvalid.into()),
        },
        AsserterTest {
            name: "negative offset",
            payload: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::Or),
                offset: Some(-1),
                ..Default::default()
            },
            err: Some(ServerError::OffsetIsNegative.into()),
        },
        AsserterTest {
            name: "negative limit",
            payload: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::Or),
                limit: Some(-1),
                ..Default::default()
            },
            err: Some(ServerError::LimitIsNegative.into()),
        },
        // TODO
        // "invalid operator" => ServerTest {
        //     request: SearchTransactionsRequest {
        //         network_identifier: valid_network_identifier(),
        //         operator: Some(Operator::Nor),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::OperatorInvalid.into()),
        // },
    ];

    default_request_asserter_tests(&tests, RequestAsserter::search_transactions_request);
}
