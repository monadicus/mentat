use super::*;

/// Signature contains the payload that was signed, the public keys of the
/// keypairs used to produce the signature, the signature (encoded in hex), and
/// the SignatureType. PublicKey is often times not known during construction of
/// the signing payloads but may be needed to combine signatures properly.
#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    /// SigningPayload is signed by the client with the keypair associated with
    /// an AccountIdentifier using the specified SignatureType. SignatureType
    /// can be optionally populated if there is a restriction on the signature
    /// scheme that can be used to sign the payload.
    pub signing_payload: SigningPayload,
    /// PublicKey contains a public key byte array for a particular CurveType
    /// encoded in hex. Note that there is no PrivateKey struct as this is NEVER
    /// the concern of an implementation.
    pub public_key: PublicKey,
    /// SignatureType is the type of a cryptographic signature.
    pub signature_type: SignatureType,
    pub hex_bytes: String,
}
