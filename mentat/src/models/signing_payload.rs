//! The module defines the `SigningPayload` model.

use super::*;

/// `SigningPayload` is signed by the client with the keypair associated with an
/// [`AccountIdentifier`] using the specified [`SignatureType`].
/// [`SignatureType`] can be optionally populated if there is a restriction on
/// the signature scheme that can be used to sign the payload.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SigningPayload {
    /// [DEPRECATED by account_identifier in v1.4.4] The network-specific
    /// address of the account that should sign the payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The `AccountIdentifier` uniquely identifies an account within a
    /// network. All fields in the account_identifier are utilized to
    /// determine this uniqueness (including the metadata field, if
    /// populated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    /// The hex bytes of the Signing Payload.
    pub bytes: Vec<u8>,
    /// `SignatureType` is the type of a cryptographic signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
}
