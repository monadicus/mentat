use super::*;

/// PublicKey contains a public key byte array for a particular CurveType encoded in hex. Note that there is no PrivateKey struct as this is NEVER the concern of an implementation.
#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    /// Hex-encoded public key bytes in the format specified by the CurveType.
    pub hex_bytes: String,
    /// CurveType is the type of cryptographic curve associated with a PublicKey.
    pub curve_type: CurveType,
}