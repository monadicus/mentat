//! The module defines the `ConstructionDeriveResponse` response.

use serde::ser::SerializeStruct;

use super::*;

/// [`ConstructionDeriveResponse`] is returned by the `/construction/derive`
/// endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq, Unchecked)]
pub struct UncheckedConstructionDeriveResponse {
    /// [DEPRECATED by `account_identifier` in v1.4.4] Address in
    /// network-specific format.
    #[unchecked(retain)]
    pub address: Option<String>,
    /// The [`AccountIdentifier`] uniquely identifies an account within a
    /// network. All fields in the `account_identifier` are utilized to
    /// determine this uniqueness (including the metadata field, if
    /// populated).
    #[unchecked(retain)]
    pub account_identifier: Option<AccountIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: Metadata,
}

impl Serialize for UncheckedConstructionDeriveResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut field_count = 1;
        if self.account_identifier.is_some() {
            field_count += 2;
        }
        let mut state = serializer.serialize_struct("ConstructionDeriveResponse", field_count)?;

        state.serialize_field("metadata", &self.metadata)?;
        if let Some(ai) = self.account_identifier.as_ref() {
            state.serialize_field("account_identifier", &ai)?;
            state.serialize_field("address", &ai.address)?;
        }
        state.end()
    }
}

/// [`ConstructionDeriveResponse`] is returned by the `/construction/derive`
/// endpoint.
#[derive(Default, Deserialize, Serialize)]
#[allow(clippy::missing_docs_in_private_items)]
#[serde(default)]
pub struct ConstructionDeriveResponsePre {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    #[allow(clippy::missing_docs_in_private_items)]
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: Metadata,
}

impl<'de> Deserialize<'de> for UncheckedConstructionDeriveResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pre = ConstructionDeriveResponsePre::deserialize(deserializer)?;

        let account_identifier = if let Some(account_identifier) = pre.account_identifier {
            Some(account_identifier)
        } else {
            pre.address.map(|address| AccountIdentifier {
                address,
                ..Default::default()
            })
        };

        Ok(UncheckedConstructionDeriveResponse {
            address: None,
            account_identifier,
            metadata: pre.metadata,
        })
    }
}
