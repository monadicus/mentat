use super::*;

/// SigningPayload is signed by the client with the keypair associated with an
/// AccountIdentifier using the specified SignatureType. SignatureType can be
/// optionally populated if there is a restriction on the signature scheme that
/// can be used to sign the payload.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SigningPayload {
    /// [DEPRECATED by account_identifier in v1.4.4] The network-specific
    /// address of the account that should sign the payload.
    pub address: Option<String>,
    /// The account_identifier uniquely identifies an account within a network.
    /// All fields in the account_identifier are utilized to determine this
    /// uniqueness (including the metadata field, if populated).
    pub account_identifier: Option<AccountIdentifier>,
    pub hex_bytes: String,
    /// SignatureType is the type of a cryptographic signature.
    pub signature_type: Option<SignatureType>,
}
