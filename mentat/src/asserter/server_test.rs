use crate::types::{AccountIdentifier, NetworkIdentifier, PartialBlockIdentifier};

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

pub(crate) const fn genesis_block_index() -> Option<u64> {
    Some(0)
}

pub(crate) const fn valid_block_index() -> Option<u64> {
    Some(1000)
}

pub(crate) fn valid_partial_block_identifier() -> PartialBlockIdentifier {
    PartialBlockIdentifier {
        index: valid_block_index(),
        ..Default::default()
    }
}
