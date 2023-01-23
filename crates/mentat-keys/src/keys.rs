use mentat_types::{decode_from_hex_string, CurveType, UncheckedCurveType, UncheckedPublicKey};

use crate::{
    errors::{KeysError, KeysResult},
    types::{KeyPair, UncheckedKeyPair},
    Signer,
    SignerEdwards25519,
};

/// `PRIV_KEY_BYTES_LEN` are 32-bytes for all supported curve types.
const PRIV_KEY_BYTES_LEN: usize = 32;

fn private_key_valid(private_key: &[u8]) -> KeysResult<()> {
    // We will need to add a match statement here if we add support
    // for CurveTypes that have a different private key length than
    // `PRIV_KEY_BYTES_LEN`.
    if private_key.len() != PRIV_KEY_BYTES_LEN {
        Err(format!(
            "expected {PRIV_KEY_BYTES_LEN} bytes for private key but got {}: {}",
            private_key.len(),
            KeysError::ErrPrivKeyLengthInvalid,
        ))?;
    }

    if mentat_asserter::bytes_array_zero(private_key) {
        Err(KeysError::ErrPrivKeyZero)?;
    }

    Ok(())
}

impl UncheckedKeyPair {
    // better for our purposes to turn this into a KeyPair and return it
    pub fn is_valid(self) -> KeysResult<KeyPair> {
        mentat_asserter::public_key(self.public_key.as_ref())
            .map_err(|err| format!("public key is invalid: {err}"))?;

        private_key_valid(&self.private_key)
            .map_err(|err| format!("private key is invalid: {err}"))?;
        Ok(self.into())
    }
}

impl KeyPair {
    /// `import_private_key` returns a `KeyPair` from a hex-encoded private key
    /// string.
    pub fn import_private_key(private_key_hex: String, curve: CurveType) -> KeysResult<Self> {
        let private_key = decode_from_hex_string(private_key_hex).map_err(|_| {
            format!(
                "failed to decode private key hex: {}",
                KeysError::ErrPrivKeyUndecodable
            )
        })?;

        // We check the parsed private key length to ensure we don't panic (most
        // crypto libraries panic with incorrect private key lengths instead of
        // throwing an error).
        private_key_valid(&private_key).map_err(|err| format!("private key is invalid: {err}"))?;

        // TODO maybe move to their types
        let key_pair: UncheckedKeyPair = match curve {
            CurveType::Edwards25519 => {
                // TODO theirs doesn't error here
                let seed = ed25519_compact::Seed::from_slice(&private_key).unwrap();
                let key_pair = ed25519_compact::KeyPair::from_seed(seed);

                UncheckedKeyPair {
                    public_key: Some(UncheckedPublicKey {
                        bytes: key_pair.pk.to_vec(),
                        curve_type: UncheckedCurveType::from(UncheckedCurveType::EDWARDS25519),
                    }),
                    private_key: seed.to_vec(),
                }
            }
            CurveType::Secp256k1 => todo!(),
            CurveType::Secp256r1 => todo!(),
            CurveType::Tweedle => todo!(),
            CurveType::Pallas => todo!(),
        };

        let valid = key_pair
            .is_valid()
            .map_err(|err| format!("key pair is invalid: {err}"))?;

        Ok(valid)
    }

    /// generate returns a `KeyPair` of a specified `CurveType`
    pub fn generate(curve: CurveType) -> KeysResult<Self> {
        // TODO maybe move to their types
        let key_pair: UncheckedKeyPair = match curve {
            CurveType::Edwards25519 => {
                // todo this doesn't fail
                // "failed to generate key pair for edwards25519 curve type: %w"
                let seed = ed25519_compact::Seed::generate();
                let key_pair = ed25519_compact::KeyPair::from_seed(seed);

                UncheckedKeyPair {
                    public_key: Some(UncheckedPublicKey {
                        bytes: key_pair.pk.to_vec(),
                        curve_type: UncheckedCurveType::from(UncheckedCurveType::EDWARDS25519),
                    }),
                    private_key: seed.to_vec(),
                }
            }
            CurveType::Secp256k1 => todo!(),
            CurveType::Secp256r1 => todo!(),
            CurveType::Tweedle => todo!(),
            CurveType::Pallas => todo!(),
        };

        let valid = key_pair
            .is_valid()
            .map_err(|err| format!("key pair is invalid: {err}"))?;

        Ok(valid)
    }

    // TODO can we remove dynamic dispatch somehow?
    /// signer returns the constructs a `Signer` for the `KeyPair`.
    pub fn signer(self) -> KeysResult<Box<dyn Signer>> {
        match self.public_key.curve_type {
            CurveType::Edwards25519 => Ok(Box::new(SignerEdwards25519 {
                key_pair: self.into(),
            })),
            CurveType::Secp256k1 => todo!(),
            CurveType::Secp256r1 => todo!(),
            CurveType::Tweedle => todo!(),
            CurveType::Pallas => todo!(),
        }
    }
}
