use indexmap::IndexMap;

use super::*;

/// Currency is composed of a canonical Symbol and Decimals. This Decimals value
/// is used to convert an Amount.Value from atomic units (Satoshis) to standard
/// units (Bitcoins).
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Currency {
    /// Canonical symbol associated with a currency.
    pub symbol: String,
    /// Number of decimal places in the standard unit representation of the
    /// amount. For example, BTC has 8 decimals. Note that it is not possible to
    /// represent the value of some currency in atomic units that is not base
    /// 10.
    pub decimals: u32,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
