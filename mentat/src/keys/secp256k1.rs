use secp256k1::{
    ecdsa::Signature,
    hashes::sha256,
    schnorr::Signature as SchnorrSignature,
    KeyPair,
    Message,
    PublicKey,
    Secp256k1,
    SecretKey,
    XOnlyPublicKey,
};

use super::{Keys, KeysError};

const SECRET_KEY_LENGTH: usize = 32;

macro_rules! impl_secp_scheme {
    ($name:ident, $sig_logic:expr, $verify_logic:expr, $sig:ty) => {
        pub struct $name {
            priv_key: SecretKey,
            pub_key: PublicKey,
        }

        impl Keys for $name {
            type M = Vec<u8>;
            type S = $sig;

            fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
                if bytes.len() != SECRET_KEY_LENGTH {
                    return Err(KeysError::InvalidPrivateKeyBytes);
                }

                let secp = Secp256k1::new();
                let priv_key =
                    SecretKey::from_slice(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
                let pub_key = PublicKey::from_secret_key(&secp, &priv_key);
                Ok(Self { priv_key, pub_key })
            }

            fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
                let msg = Message::from_hashed_data::<sha256::Hash>(message);
                let secp = Secp256k1::new();
                $sig_logic(secp, &msg, self.priv_key.clone())
            }

            fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
                let msg = Message::from_hashed_data::<sha256::Hash>(message);
                let secp = Secp256k1::new();
                $verify_logic(secp, &msg, signature, self.priv_key.clone())
            }
        }
    };
}

// We currently provide 3 schemes, ECDSA, ECDSA with recoverable public key,
// and Schnorr.
impl_secp_scheme!(
    ECDSA,
    |secp: Secp256k1<_>, msg, priv_key| Ok(secp.sign_ecdsa(msg, &priv_key)),
    |secp: Secp256k1<_>, msg, signature, priv_key| {
        let pub_key = PublicKey::from_secret_key(&secp, &priv_key);
        Ok(secp.verify_ecdsa(msg, signature, &pub_key).is_ok())
    },
    Signature
);

impl_secp_scheme!(
    ECDSARecoverable,
    |secp: Secp256k1<_>, msg, priv_key| Ok(secp
        .sign_ecdsa_recoverable(msg, &priv_key)
        .to_standard()),
    |secp: Secp256k1<_>, msg, signature, priv_key| {
        let pub_key = PublicKey::from_secret_key(&secp, &priv_key);
        Ok(secp.verify_ecdsa(msg, signature, &pub_key).is_ok())
    },
    Signature
);
impl_secp_scheme!(
    Schnorr,
    |secp: Secp256k1<_>, msg, priv_key| {
        let keypair = KeyPair::from_secret_key(&secp, priv_key);
        Ok(secp.sign_schnorr(msg, &keypair))
    },
    |secp: Secp256k1<_>, msg, signature, priv_key| {
        let keypair = KeyPair::from_secret_key(&secp, priv_key);
        let xonly = XOnlyPublicKey::from_keypair(&keypair);
        Ok(secp.verify_schnorr(signature, msg, &xonly).is_ok())
    },
    SchnorrSignature
);

#[cfg(test)]
mod tests {
    use secp256k1::rand::{self, Rng};

    use super::*;

    #[test]
    fn import_ecdsa() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        assert!(ECDSA::import_private_key(&bytes).is_ok());
    }

    #[test]
    fn import_ecdsa_recoverable() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        assert!(ECDSARecoverable::import_private_key(&bytes).is_ok());
    }

    #[test]
    fn import_schnorr() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        assert!(Schnorr::import_private_key(&bytes).is_ok());
    }

    #[test]
    fn sign_verify_ecdsa() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        let keys = ECDSA::import_private_key(&bytes).unwrap();
        let message = (0..32)
            .map(|_| rand::thread_rng().gen::<u8>())
            .collect::<Vec<u8>>();
        let sig = keys.sign(&message).unwrap();
        assert!(keys.verify(&message, &sig).unwrap());
    }

    #[test]
    fn sign_verify_ecdsa_recoverable() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        let keys = ECDSARecoverable::import_private_key(&bytes).unwrap();
        let message = (0..32)
            .map(|_| rand::thread_rng().gen::<u8>())
            .collect::<Vec<u8>>();
        let sig = keys.sign(&message).unwrap();
        assert!(keys.verify(&message, &sig).unwrap());
    }

    #[test]
    fn sign_verify_schnorr() {
        let secp = Secp256k1::new();
        let (secret_key, _) = secp.generate_keypair(&mut rand::thread_rng());
        let bytes = secret_key.secret_bytes();
        let keys = Schnorr::import_private_key(&bytes).unwrap();
        let message = (0..32)
            .map(|_| rand::thread_rng().gen::<u8>())
            .collect::<Vec<u8>>();
        let sig = keys.sign(&message).unwrap();
        assert!(keys.verify(&message, &sig).unwrap());
    }
}
