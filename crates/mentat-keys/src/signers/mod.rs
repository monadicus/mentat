mod edwards25519;
pub use edwards25519::SignerEdwards25519;

mod signer;
pub use signer::Signer;

#[cfg(test)]
#[path = ""]
mod tests {
    use super::*;

    mod edwards25519_test;
}
