use std::path::Path;

use axum::{http::request, Server};
use indexmap::{indexmap, IndexMap};

use super::test_utils::ServerTest;
use crate::{
    asserter::{
        asserter_tools::{Asserter, RequestAsserter},
        errors::{
            AssertResult, AsserterError, BlockError, ConstructionError, NetworkError, ServerError,
        },
        network::network_identifier,
        server::supported_networks,
    },
    types::{
        AccountBalanceRequest, AccountCoinsRequest, AccountIdentifier, Amount, BlockIdentifier,
        BlockRequest, BlockTransactionRequest, CallRequest, ConstructionCombineRequest,
        ConstructionDeriveRequest, ConstructionHashRequest, ConstructionMetadataRequest,
        ConstructionParseRequest, ConstructionPayloadsRequest, ConstructionPreprocessRequest,
        ConstructionSubmitRequest, Currency,
        CurveType::Secp256k1,
        EventsBlocksRequest, MempoolTransactionRequest, MetadataRequest, NetworkIdentifier,
        NetworkRequest, Operation, OperationIdentifier, Operator, PartialBlockIdentifier,
        PublicKey, SearchTransactionsRequest, Signature,
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
    historical_balance_lookup: bool,
    supported_networks: Vec<NetworkIdentifier>,
    call_methods: Vec<String>,
    err: Option<AsserterError>,
}

impl Default for NewWithOptionsTest {
    fn default() -> Self {
        Self {
            supported_operation_types: vec!["PAYMENT".into()],
            historical_balance_lookup: true,
            supported_networks: vec![valid_network_identifier()],
            call_methods: vec!["eth_call".into()],
            err: None,
        }
    }
}

#[test]
fn test_new_with_options() {
    let tests: IndexMap<&str, NewWithOptionsTest> = indexmap!(
        "basic" => Default::default(),
        "no call methods" => NewWithOptionsTest {
            call_methods: vec!(),
            ..Default::default()
        },
        "duplicate operation types" => NewWithOptionsTest {
            supported_operation_types: vec!("PAYMENT".into(), "PAYMENT".into()),
            err: Some("Allow.OperationTypes contains a duplicate PAYMENT".into()),
            ..Default::default()
        },
        "empty operation type" => NewWithOptionsTest {
            supported_operation_types: vec!("PAYMENT".into(), "".into()),
            err: Some("Allow.OperationTypes has an empty string".into()),
            ..Default::default()
        },
        "duplicate network identifier"=> NewWithOptionsTest {
            supported_networks: vec!(
                valid_network_identifier(),
                valid_network_identifier(),
            ),
            err: Some(ServerError::SupportedNetworksDuplicate.into()),
            ..Default::default()
        },
        // TODO
        // "nil network identifier"=> ServerTest {
        //     supported_networks: vec!(valid_network_identifier(), todo!()),
        //     err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        //     ..Default::default()
        // },
        "no supported networks" => NewWithOptionsTest {
            supported_networks: vec!(),
            err: Some(ServerError::NoSupportedNetworks.into()),
            ..Default::default()
        }
    );

    for (name, test) in tests {
        println!("test: {name}");
        let res = Asserter::new_server(
            test.supported_operation_types,
            test.historical_balance_lookup,
            test.supported_networks,
            test.call_methods,
            false,
            Path::new(""),
        );
        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_supported_networks() {
    let tests = indexmap!(
        "valid networks" => ServerTest {
            request: vec!(valid_network_identifier(), wrong_network_identifier()),
            err: None,
        },
        "no valid networks" => ServerTest {
            request: vec!(),
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
        "invalid networks" => ServerTest {
            request: vec!(NetworkIdentifier{
                blockchain: "blah".into(),
                network: "".into(),
                sub_network_identifier: None
            }),
            err: None,
        },
        "duplicate networks" => ServerTest {
            request: vec!(valid_network_identifier(), valid_network_identifier()),
            err: Some(format!("{}: {:?}", ServerError::SupportedNetworksDuplicate, valid_network_identifier()).into()),
        },
    );

    for (name, test) in tests {
        println!("test: {name}");

        let res = supported_networks(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[derive(Default)]
struct AccountBalanceRequestTest {
    request: AccountBalanceRequest,
    allow_historical: bool,
    err: Option<AsserterError>,
}

#[test]
fn test_account_balance_request() {
    let tests = indexmap!(
        "valid request" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with currencies" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec!(
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
                )),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with duplicate currencies" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec!(
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
                )),
                ..Default::default()
            },
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        "invalid request wrong network" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
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
        "missing network" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        "missing account" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        "valid historical request" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            },
            allow_historical: true,
            ..Default::default()
        },
        "invalid historical request" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(PartialBlockIdentifier::default()),
                ..Default::default()
            },
            allow_historical: true,
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
        "valid historical request when not enabled" => AccountBalanceRequestTest {
            request: AccountBalanceRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                block_identifier: Some(valid_partial_block_identifier()),
                ..Default::default()
            },
            err: Some(ServerError::AccountBalanceRequestHistoricalBalanceLookupNotSupported.into()),
            ..Default::default()
        },
    );

    for (name, test) in tests {
        println!("test: {name}");

        let asserter = RequestAsserter::new_server(
            vec!["PAYMENT".into()],
            test.allow_historical,
            vec![valid_network_identifier()],
            vec![],
            false,
            Path::new(""),
        )
        .unwrap();

        let res = asserter.account_balance_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_block_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_partial_block_identifier(),
            },
            ..Default::default()
        },
        "valid request for block 0" => ServerTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: PartialBlockIdentifier {
                    index: Some(genesis_block_index()),
                    ..Default::default()
                }
            },
            ..Default::default()
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: (ServerError::BlockRequestIsNil.into()),
        //     ..Default::default()
        // },
        "missing network" => ServerTest {
            request: BlockRequest {
                block_identifier: valid_partial_block_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "missing block identifier" => ServerTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        "invalid PartialBlockIdentifier request" => ServerTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: PartialBlockIdentifier::default(),
            },
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.block_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_block_transaction_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: BlockTransactionRequest{
                network_identifier: wrong_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            err: Some(
                format!("{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::BlockTransactionRequestIsNil.into()),
        // },
        "missing network" => ServerTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "missing block identifier" => ServerTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        "invalid BlockIdentifier request" => ServerTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                block_identifier: BlockIdentifier::default(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.block_transaction_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_metadata_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with public keys" => ServerTest {
            request: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: Some(vec!(PublicKey {
                    bytes: "hello".into(),
                    curve_type: Secp256k1,
                })),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionMetadataRequest {
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
        "missing network" => ServerTest {
            request: ConstructionMetadataRequest {
                options: Some(Default::default()),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "missing options" => ServerTest {
            request: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: None,
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request with public keys" => ServerTest {
            request: ConstructionMetadataRequest {
                network_identifier: valid_network_identifier(),
                options: Some(Default::default()),
                public_keys: Some(vec!(PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                })),
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_metadata_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_submit_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionSubmitRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "tx".into(),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionSubmitRequest {
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
        "empty tx" => ServerTest {
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_submit_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_mempool_transaction_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: MempoolTransactionRequest {
                network_identifier: wrong_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            err: Some(format!("{}: {:?}", ServerError::RequestedNetworkNotSupported, wrong_network_identifier()).into())
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::MempoolTransactionRequestIsNil.into()),
        // },
        "missing network" => ServerTest {
            request: MempoolTransactionRequest {
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "invalid TransactionIdentifier request" => ServerTest {
            request: MempoolTransactionRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::TxIdentifierHashMissing.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.mempool_transaction_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_metadata_request() {
    let tests = indexmap!(
        "valid request" => ServerTest::default(),
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::MetadataRequestIsNil.into()),
        // }
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.metadata_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_network_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: NetworkRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: NetworkRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(format!("{}: {:?}", ServerError::RequestedNetworkNotSupported, wrong_network_identifier()).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::NetworkRequestIsNil.into()),
        // },
        "missing network" => ServerTest {
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.network_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

fn test_construction_derive_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: valid_public_key(),
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionDeriveRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!("{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()).into()
            ),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionDeriveRequestIsNil.into()),
        // },
        "nil public key" => ServerTest {
            request: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        },
        "empty public key bytes" => ServerTest {
            request: ConstructionDeriveRequest {
                network_identifier: valid_network_identifier(),
                public_key: PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                },
                ..Default::default()
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_derive_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_preprocess_request() {
    let positive_fee_multiplier = Some(1.1f64);
    let negative_fee_multiplier = Some(-1.1f64);

    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with suggested fee multiplier" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with max fee" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with suggested fee multiplier and max fee" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: Some(vec!(valid_amount())),
                suggested_fee_multiplier: positive_fee_multiplier,
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionPreprocessRequestIsNil.into()),
        // },
        "nil operations" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into())
        },
        "empty operations" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into())
        },
        "unsupported operation type" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationTypeInvalid.into())
        },
        "invalid operations" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        "negative suggested fee multiplier" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                suggested_fee_multiplier: negative_fee_multiplier,
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::ConstructionPreprocessRequestSuggestedFeeMultiplierIsNeg,
                negative_fee_multiplier).into()
            ),
        },
        "max fee with duplicate currency" => ServerTest {
            request: ConstructionPreprocessRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                max_fee: Some(vec!(valid_amount(), valid_amount())),
                ..Default::default()
            },
            err: Some(format!("currency {:?} used multiple times", valid_amount().currency).into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_preprocess_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_payloads_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with public keys" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: Some(vec!(PublicKey {
                    bytes: "hello".into(),
                    curve_type: Secp256k1
                })),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionPayloadsRequestIsNil.into()),
        // },
        "nil operations" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        "empty operations" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: vec!(Operation::default()),
                ..Default::default()
            },
            err: Some(BlockError::NoOperationsForConstruction.into()),
        },
        "unsupported operation type" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: unsupported_type_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationTypeInvalid.into()),
        },
        "invalid operations" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: invalid_ops(),
                ..Default::default()
            },
            err: Some(BlockError::OperationStatusNotEmptyForConstruction.into()),
        },
        "invalid request with public keys" => ServerTest {
            request: ConstructionPayloadsRequest {
                network_identifier: valid_network_identifier(),
                operations: valid_ops(),
                metadata: indexmap!("test".into() => "hello".into()),
                public_keys: Some(vec!(PublicKey {
                    curve_type: Secp256k1,
                    ..Default::default()
                })),
            },
            err: Some(ConstructionError::PublicKeyBytesEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_payload_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_combine_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: valid_signatures(),
            },
            ..Default::default()
        },
        "valid request 2" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec!(Signature {
                    signing_payload: SigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    },
                    public_key: valid_public_key(),
                    signature_type: Ed25519,
                    bytes: "blah".into()
                }),
            },
            ..Default::default()
        },
        "valid request 3" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec!(Signature {
                    signing_payload: SigningPayload {
                        account_identifier: Some(valid_account()),
                        bytes: "blah".into(),
                        ..Default::default()
                    },
                    public_key: valid_public_key(),
                    signature_type: Ed25519,
                    bytes: "hello".into()
                }),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!("{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionCombineRequestIsNil.into()),
        // },
        "empty unsigned transaction" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                signatures: valid_signatures(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionCombineRequestUnsignedTxEmpty.into()),
        },
        "nil signatures" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                ..Default::default()
            },
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        "empty signatures" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: vec!(Signature::default()),
            },
            err: Some(ConstructionError::SignaturesEmpty.into()),
        },
        "signature type mismatch" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_mismatch(),
            },
            err: Some(ConstructionError::SignaturesReturnedSigMismatch.into()),
        },
        "empty signature" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: empty_signature(),
            },
            err: Some(ConstructionError::SignatureBytesEmpty.into()),
        },
        "signature type match" => ServerTest {
            request: ConstructionCombineRequest {
                network_identifier: valid_network_identifier(),
                unsigned_transaction: "blah".into(),
                signatures: signature_type_match(),
            },
            ..Default::default()
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_combine_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_hash_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                signed_transaction: "blah".into(),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionHashRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::ConstructionHashRequestIsNil.into()),
        // },
        "empty signed transaction" => ServerTest {
            request: ConstructionHashRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionHashRequestSignedTxEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_hash_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_construction_parse_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                transaction: "blah".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: ConstructionParseRequest {
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
        "empty signed transaction" => ServerTest {
            request: ConstructionParseRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::ConstructionParseRequestEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.construction_parse_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_call_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with params" => ServerTest {
            request: CallRequest {
                network_identifier: valid_network_identifier(),
                method: "eth_call".into(),
                parameters: indexmap!("hello".into() => "test".into()),
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: CallRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        "unsupported method" => ServerTest {
            request: CallRequest {
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
        "empty method" => ServerTest {
            request: CallRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(ServerError::CallMethodEmpty.into()),
        },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.call_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[derive(Default)]
struct AccountCoinsRequestTest {
    request: AccountCoinsRequest,
    allow_mempool: bool,
    err: Option<AsserterError>,
}

#[test]
fn test_account_coins_request() {
    let tests = indexmap!(
        "valid request" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with currencies" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec!(
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    },
                    Currency {
                        symbol: "ETH".into(),
                        decimals: 18,
                        ..Default::default()
                    }
                )),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request with duplicate currencies" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                currencies: Some(vec!(
                    Currency {
                        symbol: "BTC".into(),
                        decimals: 8,
                        ..Default::default()
                    }; 2
                )),
                ..Default::default()
            },
            err: Some(ServerError::DuplicateCurrency.into()),
            ..Default::default()
        },
        "invalid request wrong network" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
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
        "missing network" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
            ..Default::default()
        },
        "missing account" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::AccountIsNil.into()),
            ..Default::default()
        },
        "valid mempool lookup request" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                ..Default::default()
            },
            allow_mempool: true,
            ..Default::default()
        },
        "valid mempool lookup request when not enabled" => AccountCoinsRequestTest {
            request: AccountCoinsRequest {
                network_identifier: wrong_network_identifier(),
                account_identifier: valid_account_identifier().unwrap(),
                include_mempool: true,
                ..Default::default()
            },
            err: Some(ServerError::MempoolCoinsNotSupported.into()),
            ..Default::default()
        },
    );

    for (name, test) in tests {
        println!("test: {name}");

        let asserter = RequestAsserter::new_server(
            vec!["PAYMENT".into()],
            true,
            vec![valid_network_identifier()],
            vec![],
            test.allow_mempool,
            Path::new(""),
        )
        .unwrap();

        let res = asserter.account_coins_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_event_blocks_request() {
    let tests = indexmap!(
        "valid request" => ServerTest {
            request: EventsBlocksRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: EventsBlocksRequest {
                network_identifier: wrong_network_identifier(),
                ..Default::default()
            },
            err: Some(
                format!("{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into()),
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::EventsBlocksRequestIsNil.into()),
        // },
        // TODO
        // "negative offset" => ServerTest {
        //     request: EventsBlocksRequest {
        //         network_identifier: valid_network_identifier(),
        //         offset: todo!(), // Some(-1),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::OffsetIsNegative.into()),
        // },
        // TODO
        // "negative limit" => ServerTest {
        //     request: EventsBlocksRequest {
        //         network_identifier: valid_network_identifier(),
        //         limit: todo!(), // Some(-1),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::LimitIsNegative.into()),
        // },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.events_block_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}

#[test]
fn test_search_transactions_request() {
    let tests = indexmap!(
        "valid request no operator" => ServerTest {
            request: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            ..Default::default()
        },
        "valid request" => ServerTest {
            request: SearchTransactionsRequest {
                network_identifier: valid_network_identifier(),
                operator: Some(Operator::And),
                ..Default::default()
            },
            ..Default::default()
        },
        "invalid request wrong network" => ServerTest {
            request: SearchTransactionsRequest {
                network_identifier: wrong_network_identifier(),
                operator: Some(Operator::Or),
                ..Default::default()
            },
            err: Some(format!(
                "{}: {:?}",
                ServerError::RequestedNetworkNotSupported,
                wrong_network_identifier()
            ).into())
        },
        // TODO
        // "nil request" => ServerTest {
        //     request: todo!(),
        //     err: Some(ServerError::SearchTransactionsRequestIsNil.into()),
        // },
        // TODO
        // "negative max block" => ServerTest {
        //     request: SearchTransactionsRequest {
        //         network_identifier: valid_network_identifier(),
        //         operator: Some(Operator::Or),
        //         max_block: todo!(), // Some(-1),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::MaxBlockInvalid.into()),
        // },
        // TODO
        // "negative offset" => ServerTest {
        //     request: SearchTransactionsRequest {
        //         network_identifier: valid_network_identifier(),
        //         operator: Some(Operator::Or),
        //         offset: Some(-1),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::OffsetIsNegative.into()),
        // },
        // TODO
        // "negative limit" => ServerTest {
        //         request: SearchTransactionsRequest {
        //         network_identifier: valid_network_identifier(),
        //         operator: Some(Operator::Or),
        //         limit: Some(-1),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::LimitIsNegative.into()),
        // },
        // TODO
        // "invalid operator" => ServerTest {
        //     request: SearchTransactionsRequest {
        //         network_identifier: valid_network_identifier(),
        //         operator: Some(Operator::Nor),
        //         ..Default::default()
        //     },
        //     err: Some(ServerError::OperatorInvalid.into()),
        // },
    );

    let server = request_asserter();

    for (name, test) in tests {
        println!("test: {name}");

        let res = server.search_transactions_request(&test.request);

        if let Err(err) = res {
            assert!(test
                .err
                .map(|e| err.to_string().contains(&e.to_string()))
                .unwrap_or_default());
        } else {
            assert_eq!(None, test.err);
        }
    }
}
