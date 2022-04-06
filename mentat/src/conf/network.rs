//! This module contains the possible networks a node can run on.

use std::fmt;

use super::{Deserialize, Serialize};

///
/// The possible modes a node network can run in.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Network {
    Mainnet,
    Testnet,
    Other(String),
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "MAINNET"),
            Self::Testnet => write!(f, "TESTNET"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<String> for Network {
    fn from(item: String) -> Self {
        use Network::*;

        match item.as_ref() {
            "MAINNET" => Mainnet,
            "TESTNET" => Testnet,
            _ => Other(item),
        }
    }
}
