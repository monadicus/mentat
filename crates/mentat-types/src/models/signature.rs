//! The module defines the `Signature` model.

use super::*;

/// [`Signature`] contains the payload that was signed, the public keys of the
/// keypairs used to produce the signature, the signature (encoded in hex), and
/// the SignatureType. [`PublicKey`] is often times not known during
/// construction of the signing payloads but may be needed to combine signatures
/// properly.
#[derive(Debug, Deserialize, Serialize, Default, Unchecked)]
#[serde(default, deny_unknown_fields)]
pub struct UncheckedSignature {
    /// [`SigningPayload`] is signed by the client with the keypair associated
    /// with an AccountIdentifier using the specified [`SignatureType`].
    /// [`SignatureType`] can be optionally populated if there is a restriction
    /// on the signature scheme that can be used to sign the payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_payload: Option<UncheckedSigningPayload>,
    /// [`PublicKey`] contains a public key byte array for a particular
    /// CurveType encoded in hex. Note that there is no `PrivateKey` struct as
    /// this is NEVER the concern of an implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<UncheckedPublicKey>,
    /// [`SignatureType`] is the type of a cryptographic signature.
    #[unchecked(option_enum)]
    pub signature_type: UncheckedSignatureType,
    /// The hex bytes for the `Signature`.
    #[serde(
        rename = "hex_bytes",
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "bytes_to_hex_str",
        deserialize_with = "null_default_bytes_to_hex"
    )]
    #[unchecked(bytes)]
    pub bytes: Vec<u8>,
}
