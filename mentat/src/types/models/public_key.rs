//! The module defines the `PublicKey` model.

use mentat_macros::Nullable;

use super::*;

/// `PublicKey` contains a public key byte array for a particular [`CurveType`]
/// encoded in hex. Note that there is no `PrivateKey` struct as this is NEVER
/// the concern of an implementation.
#[derive(Clone, Debug, Deserialize, Serialize, Default, Nullable)]
#[serde(default)]
pub struct NullablePublicKey {
    /// Hex-encoded public key bytes in the format specified by the
    /// [`CurveType`].
    #[serde(
        rename = "hex_bytes",
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "bytes_to_hex_str",
        deserialize_with = "null_default_bytes_to_hex"
    )]
    pub bytes: Vec<u8>,
    /// [`CurveType`] is the type of cryptographic curve associated with a
    /// PublicKey.
    #[serde(default = "NullableCurveType::default")]
    pub curve_type: NullableCurveType,
}
