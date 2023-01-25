mod edwards25519;
pub use edwards25519::SignerEdwards25519;

mod pallas;
pub use pallas::SignerPallas;

mod signer;
pub use signer::*;

#[cfg(test)]
#[path = ""]
mod tests {
    use super::*;

    mod edwards25519_test;
    mod pallas_test;
}
