//! The module defines the `Amount` model.

use indexmap::IndexMap;

use super::*;

/// Amount is some Value of a [`Currency`]. It is considered invalid to specify
/// a Value without a [`Currency`].
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Amount {
    /// Value of the transaction in atomic units represented as an
    /// arbitrary-sized signed integer. For example, 1 BTC would be represented
    /// by a value of 100000000.
    pub value: String,
    /// `Currency` is composed of a canonical Symbol and Decimals. This
    /// Decimals value is used to convert an Amount.Value from atomic units
    /// (Satoshis) to standard units (Bitcoins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    #[serde(default)]
    pub metadata: IndexMap<String, Value>,
}
