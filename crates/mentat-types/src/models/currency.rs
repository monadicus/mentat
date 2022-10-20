//! The module defines the `Currency` model.

use super::*;

/// [`Currency`] is composed of a canonical Symbol and Decimals. This Decimals
/// value is used to convert an Amount.Value from atomic units (Satoshis) to
/// standard units (Bitcoins).
#[derive(Clone, Debug, Default, Eq, Deserialize, PartialEq, Serialize, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedCurrency {
    /// Canonical symbol associated with a currency.
    pub symbol: String,
    /// Number of decimal places in the standard unit representation of the
    /// amount. For example, BTC has 8 decimals. Note that it is not possible to
    /// represent the value of some currency in atomic units that is not base
    /// 10.
    #[unchecked(usize)]
    pub decimals: isize,
    /// Any additional information related to the currency itself. For example,
    /// it would be useful to populate this object with the contract address of
    /// an ERC-20 token.
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: Metadata,
}

impl From<String> for UncheckedCurrency {
    fn from(symbol: String) -> Self {
        Self {
            symbol,
            ..Default::default()
        }
    }
}

impl Sortable for UncheckedCurrency {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.metadata.sort_keys();
        new
    }
}

impl Sortable for Currency {
    fn sort(&self) -> Self {
        let mut new = self.clone();
        new.metadata.sort_keys();
        new
    }
}

impl EstimateSize for Currency {
    fn estimated_size(&self) -> usize {
        size_of_val(self)
            + size_of_val(self.symbol.as_str())
            + estimated_metadata_size(&self.metadata)
    }
}
