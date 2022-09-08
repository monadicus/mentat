//! The module defines the `ConstructionParseResponse` response.

use serde::ser::SerializeStruct;

use super::*;

/// [`ConstructionParseResponse`] contains an array of operations that occur in
/// a transaction blob. This should match the array of operations provided to
/// `/construction/preprocess` and `/construction/payloads`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Unchecked)]
pub struct UncheckedConstructionParseResponse {
    #[allow(clippy::missing_docs_in_private_items)]
    pub operations: Vec<Option<UncheckedOperation>>,
    /// [DEPRECATED by `account_identifier_signers` in v1.4.4] All signers
    /// (addresses) of a particular transaction. If the transaction is unsigned,
    /// it should be empty.
    pub signers: Vec<String>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub account_identifier_signers: Vec<Option<AccountIdentifier>>,
    #[allow(clippy::missing_docs_in_private_items)]
    pub metadata: Metadata,
}

impl Serialize for UncheckedConstructionParseResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut field_count = 2;
        if !self.account_identifier_signers.is_empty() {
            field_count += 2;
        }
        let mut state = serializer.serialize_struct("ConstructionParseResponse", field_count)?;

        state.serialize_field("operations", &self.operations)?;
        state.serialize_field("metadata", &self.metadata)?;
        if !self.account_identifier_signers.is_empty() {
            state.serialize_field(
                "account_identifier_signers",
                &self.account_identifier_signers,
            )?;
            state.serialize_field(
                "signers",
                &self
                    .account_identifier_signers
                    .iter()
                    .map(|ai| ai.clone().unwrap_or_default().address)
                    .collect::<Vec<String>>(),
            )?;
        }
        state.end()
    }
}

#[derive(Default, Deserialize)]
#[allow(clippy::missing_docs_in_private_items)]
#[serde(default)]
pub struct ConstructionParseResponsePre {
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub operations: Vec<Option<UncheckedOperation>>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub signers: Vec<String>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "null_default"
    )]
    pub account_identifier_signers: Vec<Option<AccountIdentifier>>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub metadata: Metadata,
}

impl<'de> Deserialize<'de> for UncheckedConstructionParseResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pre = ConstructionParseResponsePre::deserialize(deserializer)?;

        let account_identifier_signers =
            if pre.account_identifier_signers.is_empty() && !pre.signers.is_empty() {
                pre.signers
                    .into_iter()
                    .map(|address| {
                        Some(AccountIdentifier {
                            address,
                            ..Default::default()
                        })
                    })
                    .collect()
            } else {
                pre.account_identifier_signers
            };

        Ok(UncheckedConstructionParseResponse {
            operations: pre.operations,
            signers: Vec::new(),
            account_identifier_signers,
            metadata: pre.metadata,
        })
    }
}
