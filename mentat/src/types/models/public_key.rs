//! The module defines the `PublicKey` model.

use super::*;

/// `PublicKey` contains a public key byte array for a particular [`CurveType`]
/// encoded in hex. Note that there is no `PrivateKey` struct as this is NEVER
/// the concern of an implementation.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct PublicKey {
    /// Hex-encoded public key bytes in the format specified by the
    /// [`CurveType`].
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "hex::serialize",
        deserialize_with = "null_default_bytes_to_hex"
    )]
    pub bytes: Vec<u8>,
    /// [`CurveType`] is the type of cryptographic curve associated with a
    /// PublicKey.
    pub curve_type: CurveType,
}
