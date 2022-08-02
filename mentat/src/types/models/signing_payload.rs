//! The module defines the `SigningPayload` model.

use mentat_macros::Nullable;
use serde::ser::SerializeStruct;

use super::*;

// TODO add `#[nullable(skip_serde)]` and copy paste serde impls for
// non-nullable struct
/// [`SigningPayload`] is signed by the client with the keypair associated with
/// an [`AccountIdentifier`] using the specified [`SignatureType`].
/// [`SignatureType`] can be optionally populated if there is a restriction on
/// the signature scheme that can be used to sign the payload.
#[derive(Clone, Debug, Default, PartialEq, Eq, Nullable)]
pub struct NullableSigningPayload {
    /// [DEPRECATED by account_identifier in v1.4.4] The network-specific
    /// address of the account that should sign the payload.
    #[nullable(retain)]
    pub address: Option<String>,
    /// The `AccountIdentifier` uniquely identifies an account within a
    /// network. All fields in the account_identifier are utilized to
    /// determine this uniqueness (including the metadata field, if
    /// populated).
    #[nullable(retain)]
    pub account_identifier: Option<AccountIdentifier>,
    /// The hex bytes of the Signing Payload.
    #[nullable(bytes)]
    pub bytes: Vec<u8>,
    /// `SignatureType` is the type of a cryptographic signature.
    #[nullable(option_enum)]
    pub signature_type: NullableSignatureType,
}

impl Serialize for NullableSigningPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut field_count = 2;
        if self.account_identifier.is_some() {
            field_count += 2;
        }
        let mut state = serializer.serialize_struct("SigningPayload", field_count)?;

        let hex = encode_to_hex_string(&self.bytes);
        state.serialize_field("hex_bytes", &hex)?;
        state.serialize_field("signature_type", &self.signature_type)?;
        if let Some(ai) = self.account_identifier.as_ref() {
            state.serialize_field("account_identifier", &ai)?;
            state.serialize_field("address", &ai.address)?;
        }
        state.end()
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
#[allow(clippy::missing_docs_in_private_items)]
struct SigningPayloadPre {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    #[serde(
        rename = "hex_bytes",
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "bytes_to_hex_str",
        deserialize_with = "null_default_bytes_to_hex"
    )]
    pub bytes: Vec<u8>,
    pub signature_type: NullableSignatureType,
}

impl<'de> Deserialize<'de> for NullableSigningPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pre = SigningPayloadPre::deserialize(deserializer)?;

        let account_identifier = if let Some(account_identifier) = pre.account_identifier {
            Some(account_identifier)
        } else {
            pre.address.map(|address| AccountIdentifier {
                address,
                ..Default::default()
            })
        };

        Ok(NullableSigningPayload {
            address: None,
            account_identifier,
            bytes: pre.bytes,
            signature_type: pre.signature_type,
        })
    }
}
