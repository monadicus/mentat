use indexmap::IndexMap;

use super::*;

/// Amount is some Value of a Currency. It is considered invalid to specify a
/// Value without a Currency.
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Amount {
    /// Value of the transaction in atomic units represented as an
    /// arbitrary-sized signed integer. For example, 1 BTC would be represented
    /// by a value of 100000000.
    pub value: String,
    /// Currency is composed of a canonical Symbol and Decimals. This Decimals
    /// value is used to convert an Amount.Value from atomic units (Satoshis) to
    /// standard units (Bitcoins).
    pub currency: Currency,
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
