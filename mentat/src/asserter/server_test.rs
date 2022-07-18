use std::path::Path;

use axum::{http::request, Server};
use indexmap::{indexmap, IndexMap};

use crate::{
    asserter::{
        asserter_tools::{Asserter, RequestAsserter},
        errors::{AssertResult, AsserterError, BlockError, NetworkError, ServerError},
        network::network_identifier,
        server::supported_networks,
    },
    types::{
        AccountBalanceRequest, AccountIdentifier, Amount, BlockIdentifier, BlockRequest,
        BlockTransactionRequest, Currency,
        CurveType::Secp256k1,
        NetworkIdentifier, Operation, OperationIdentifier, PartialBlockIdentifier, PublicKey,
        Signature,
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

pub(crate) fn a() -> AssertResult<Asserter> {
    Asserter::new_server(
        vec!["PAYMENT".into()],
        true,
        vec![valid_network_identifier()],
        vec!["eth_call".into()],
        false,
        Path::new(""),
    )
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

struct SupportedNetworksTest {
    networks: Vec<NetworkIdentifier>,
    err: Option<AsserterError>,
}

#[test]
fn test_supported_networks() {
    let tests = indexmap!(
        "valid networks" => SupportedNetworksTest {
            networks: vec!(valid_network_identifier(), wrong_network_identifier()),
            err: None,
        },
        "no valid networks" => SupportedNetworksTest {
            networks: vec!(),
            err: Some(ServerError::NoSupportedNetworks.into()),
        },
        "invalid networks" => SupportedNetworksTest {
            networks: vec!(NetworkIdentifier{
                blockchain: "blah".into(),
                network: "".into(),
                sub_network_identifier: None
            }),
            err: None,
        },
        "duplicate networks" => SupportedNetworksTest {
            networks: vec!(valid_network_identifier(), valid_network_identifier()),
            err: Some(format!("{}: {:?}", ServerError::SupportedNetworksDuplicate, valid_network_identifier()).into()),
        },
    );

    for (name, test) in tests {
        println!("test: {name}");

        let res = supported_networks(&test.networks);

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

#[derive(Default)]
struct BlockRequestTest {
    request: BlockRequest,
    err: Option<AsserterError>,
}

#[test]
fn test_block_request() {
    let tests = indexmap!(
        "valid request" => BlockRequestTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: valid_partial_block_identifier(),
            },
            ..Default::default()
        },
        "valid request for block 0" => BlockRequestTest {
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
        // "nil request" => BlockRequestTest {
        //     request: todo!(),
        //     err: (ServerError::BlockRequestIsNil.into()),
        //     ..Default::default()
        // },
        "missing network" => BlockRequestTest {
            request: BlockRequest {
                block_identifier: valid_partial_block_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "missing block identifier" => BlockRequestTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::PartialBlockIdentifierIsNil.into()),
        },
        "invalid PartialBlockIdentifier request" => BlockRequestTest {
            request: BlockRequest {
                network_identifier: valid_network_identifier(),
                block_identifier: PartialBlockIdentifier::default(),
            },
            err: Some(BlockError::PartialBlockIdentifierFieldsNotSet.into()),
        },
    );

    let server = RequestAsserter::new_server(
        vec!["PAYMENT".into()],
        false,
        vec![valid_network_identifier()],
        vec![],
        false,
        Path::new(""),
    )
    .unwrap();

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

#[derive(Default)]
struct BlockTransactionRequestTest {
    request: BlockTransactionRequest,
    err: Option<AsserterError>,
}

#[test]
fn test_block_transaction_request() {
    let tests = indexmap!(
        "valid request" => BlockTransactionRequestTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                block_identifier: valid_block_identifier(),
                transaction_identifier: valid_transaction_identifier(),
            },
            ..Default::default()
        },
        "invalid request wrong network" => BlockTransactionRequestTest {
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
        // "nil request" => BlockTransactionRequestTest {
        //     request: todo!(),
        //     err: Some(ServerError::BlockTransactionRequestIsNil.into()),
        // },
        "missing network" => BlockTransactionRequestTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(NetworkError::NetworkIdentifierIsNil.into()),
        },
        "missing block identifier" => BlockTransactionRequestTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                transaction_identifier: valid_transaction_identifier(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierIsNil.into()),
        },
        "invalid BlockIdentifier request" => BlockTransactionRequestTest {
            request: BlockTransactionRequest{
                network_identifier: valid_network_identifier(),
                block_identifier: BlockIdentifier::default(),
                ..Default::default()
            },
            err: Some(BlockError::BlockIdentifierHashMissing.into()),
        },
    );

    let server = RequestAsserter::new_server(
        vec!["PAYMENT".into()],
        false,
        vec![valid_network_identifier()],
        vec![],
        false,
        Path::new(""),
    )
    .unwrap();

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
