mod edwards25519;
pub use edwards25519::SignerEdwards25519;

mod pallas;
pub use self::pallas::SignerPallas;

mod secp256k1;
pub use self::secp256k1::*;

mod secp256r1;
pub use self::secp256r1::*;

mod signer;
pub use signer::*;

use crate::{
    errors::{KeysError, KeysResult},
    signers::signer::SignerInterface,
    types::{KeyPair, UncheckedKeyPair},
};

#[cfg(test)]
#[path = ""]
mod tests {
    use super::*;
    mod edwards25519_test;
    mod pallas_test;
    mod secp256k1_test;
    mod secp256r1_test;

    mod test_utils;
    pub use test_utils::*;
}
