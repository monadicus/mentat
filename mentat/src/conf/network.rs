use std::fmt;

use super::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Network {
    Mainnet,
    Testnet,
    Other(String),
}

impl<'de> Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_uppercase();

        Ok(Network::from(s))
    }
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

impl Serialize for Network {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
