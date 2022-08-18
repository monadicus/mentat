#![allow(clippy::missing_docs_in_private_items)]

use std::sync::Arc;

use mentat_types::{
    AccountIdentifier, Amount, Block, BlockIdentifier, Currency, NetworkIdentifier, Operation,
    OperationIdentifier, Transaction, TransactionIdentifier,
};
use parking_lot::Mutex;

use crate::{
    errors::SyncerResult,
    types::{MockHelper, Syncer},
};

fn network_identifier() -> NetworkIdentifier {
    NetworkIdentifier {
        blockchain: "blah".into(),
        network: "testnet".into(),
        ..Default::default()
    }
}

fn currency() -> Currency {
    Currency {
        symbol: "blah".into(),
        decimals: 2,
        ..Default::default()
    }
}

fn recipient() -> AccountIdentifier {
    AccountIdentifier {
        address: "acct1".into(),
        ..Default::default()
    }
}

pub fn recipient_amount() -> Amount {
    Amount {
        value: "100".to_string(),
        currency: currency(),
        ..Default::default()
    }
}

pub fn recipient_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Success".into()),
        account: Some(recipient()),
        amount: Some(recipient_amount()),
        ..Default::default()
    }
}

pub fn recipient_failure_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 1,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Failure".to_string()),
        account: Some(recipient()),
        amount: Some(recipient_amount()),
        ..Default::default()
    }
}

pub fn recipient_transaction() -> Transaction {
    Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "tx1".to_string(),
        },
        operations: vec![recipient_operation(), recipient_failure_operation()],
        ..Default::default()
    }
}

pub fn sender() -> AccountIdentifier {
    AccountIdentifier {
        address: "acct2".to_string(),
        ..Default::default()
    }
}

pub fn sender_amount() -> Amount {
    Amount {
        value: "-100".to_string(),
        currency: currency(),
        ..Default::default()
    }
}

pub fn sender_operation() -> Operation {
    Operation {
        operation_identifier: OperationIdentifier {
            index: 0,
            ..Default::default()
        },
        type_: "Transfer".to_string(),
        status: Some("Success".to_string()),
        account: Some(sender()),
        amount: Some(sender_amount()),
        ..Default::default()
    }
}

pub fn sender_transaction() -> Transaction {
    Transaction {
        transaction_identifier: TransactionIdentifier {
            hash: "tx2".to_string(),
        },
        operations: vec![sender_operation()],
        ..Default::default()
    }
}

pub fn orphan_genesis() -> Block {
    Block {
        block_identifier: BlockIdentifier {
            hash: "1".to_string(),
            index: 1,
        },
        parent_block_identifier: BlockIdentifier {
            hash: "0a".to_string(),
            index: 0,
        },
        transactions: vec![],
        ..Default::default()
    }
}

pub fn block_sequence() -> Vec<Block> {
    vec![
        // genesis
        Block {
            block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "1".into(),
                index: 1,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            transactions: vec![recipient_transaction()],
            ..Default::default()
        },
        // reorg
        Block {
            block_identifier: BlockIdentifier {
                hash: "2".into(),
                index: 2,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "1a".into(),
                index: 1,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "1a".into(),
                index: 1,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "0".into(),
                index: 0,
            },
            ..Default::default()
        },
        Block {
            block_identifier: BlockIdentifier {
                hash: "3".into(),
                index: 3,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "2".into(),
                index: 2,
            },
            transactions: vec![sender_transaction()],
            ..Default::default()
        },
        // invalid block
        Block {
            block_identifier: BlockIdentifier {
                hash: "5".into(),
                index: 5,
            },
            parent_block_identifier: BlockIdentifier {
                hash: "4".into(),
                index: 4,
            },
            ..Default::default()
        },
    ]
}

fn last_block_identifier<Handler, Helper, F>(
    syncer: &Syncer<Handler, Helper, F>,
) -> &BlockIdentifier {
    syncer.past_blocks.back().unwrap()
}

#[test]
fn test_process_block() {
    let mut err_buff: Arc<Mutex<Option<SyncerResult<()>>>> = Arc::new(Mutex::new(None));

    let mut mock_helper = MockHelper::new();

    todo!()
}

fn create_blocks(start_index: i64, end_index: i64, add: &str) -> Vec<Block> {
    todo!()
}

fn assert_not_canceled() {
    todo!()
}

#[test]
fn test_sync_no_reorg() {
    todo!()
}

#[test]
fn test_sync_specific_start() {
    todo!()
}

#[test]
fn test_sync_cancel() {
    todo!()
}

#[test]
fn test_sync_reorg() {
    todo!()
}

#[test]
fn test_sync_manual_reorg() {
    todo!()
}

#[test]
fn test_sync_dynamic() {
    todo!()
}

#[test]
fn test_sync_dynamic_overhead() {
    todo!()
}
