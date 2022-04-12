use std::marker::PhantomData;

use mina_hasher::{Hashable, ROInput};
use mina_signer::{create_legacy, CompressedPubKey, Keypair, NetworkId, PubKey, Signature, Signer};

use super::{Keys, KeysError};

/// A keypair used for signing and verifying messages in the Mina signature
/// scheme. Note that the signature scheme needs to be instantiated with a
/// specific message structure beforehand; this is currently hardcoded on the
/// [`Transaction`] struct found below.
pub struct PallasKeys<H: Hashable> {
    keypair: Keypair,
    _phantom_data: PhantomData<H>,
}

impl<H: Hashable> Keys for PallasKeys<H> {
    type M = Transaction;
    type S = Signature;

    fn import_private_key(bytes: &[u8]) -> Result<Self, KeysError> {
        let hex_string =
            std::str::from_utf8(bytes).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        let keypair =
            Keypair::from_hex(hex_string).map_err(|_| KeysError::InvalidPrivateKeyBytes)?;
        Ok(Self {
            keypair,
            _phantom_data: PhantomData::default(),
        })
    }

    fn sign(&self, message: &Self::M) -> Result<Self::S, KeysError> {
        let mut signer = create_legacy::<Self::M>(NetworkId::MAINNET);
        Ok(signer.sign(&self.keypair, message))
    }

    fn verify(&self, message: &Self::M, signature: &Self::S) -> Result<bool, KeysError> {
        let mut signer = create_legacy::<Self::M>(NetworkId::MAINNET);
        Ok(signer.verify(signature, &self.keypair.public, message))
    }
}

// Mina signature scheme typically initializes its keys for specific
// structures. This transaction structure is an identical copy of the Mina one,
// and was taken from: https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs

const MEMO_BYTES: usize = 34;
const TAG_BITS: usize = 3;

/// A Mina transaction. Used for instantiation of the [`PallasKeys`].
#[derive(Clone, Copy)]
pub struct Transaction {
    // Common
    pub fee: u64,
    pub fee_token: u64,
    pub fee_payer_pk: CompressedPubKey,
    pub nonce: u32,
    pub valid_until: u32,
    pub memo: [u8; MEMO_BYTES],
    // Body
    pub tag: [bool; TAG_BITS],
    pub source_pk: CompressedPubKey,
    pub receiver_pk: CompressedPubKey,
    pub token_id: u64,
    pub amount: u64,
    pub token_locked: bool,
}

impl Hashable for Transaction {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new();

        roi.append_field(self.fee_payer_pk.x);
        roi.append_field(self.source_pk.x);
        roi.append_field(self.receiver_pk.x);

        roi.append_u64(self.fee);
        roi.append_u64(self.fee_token);
        roi.append_bool(self.fee_payer_pk.is_odd);
        roi.append_u32(self.nonce);
        roi.append_u32(self.valid_until);
        roi.append_bytes(&self.memo);

        for tag_bit in self.tag {
            roi.append_bool(tag_bit);
        }

        roi.append_bool(self.source_pk.is_odd);
        roi.append_bool(self.receiver_pk.is_odd);
        roi.append_u64(self.token_id);
        roi.append_u64(self.amount);
        roi.append_bool(self.token_locked);

        roi
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::OsRng, Rng};

    use super::*;

    #[test]
    fn import() {
        let keypair = Keypair::rand(&mut OsRng {});
        let bytes = format!("{keypair}");
        assert!(PallasKeys::<Transaction>::import_private_key(&bytes.as_bytes()).is_ok());
    }

    #[test]
    fn sign_verify() {
        let keypair = Keypair::rand(&mut OsRng {});
        let bytes = format!("{keypair}");
        let keys = PallasKeys::<Transaction>::import_private_key(&bytes.as_bytes()).unwrap();
        let message = Transaction::new_payment(keypair.public, keypair.public, 0, 0, 0);
        let sig = keys.sign(&message).unwrap();
        assert!(keys.verify(&message, &sig).unwrap());
    }

    const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];
    const DELEGATION_TX_TAG: [bool; TAG_BITS] = [false, false, true];

    impl Transaction {
        pub fn new_payment(from: PubKey, to: PubKey, amount: u64, fee: u64, nonce: u32) -> Self {
            Transaction {
                fee,
                fee_token: 1,
                fee_payer_pk: from.into_compressed(),
                nonce,
                valid_until: u32::MAX,
                memo: array_init::array_init(|i| (i == 0) as u8),
                tag: PAYMENT_TX_TAG,
                source_pk: from.into_compressed(),
                receiver_pk: to.into_compressed(),
                token_id: 1,
                amount,
                token_locked: false,
            }
        }

        pub fn new_delegation(from: PubKey, to: PubKey, fee: u64, nonce: u32) -> Self {
            Transaction {
                fee,
                fee_token: 1,
                fee_payer_pk: from.into_compressed(),
                nonce,
                valid_until: u32::MAX,
                memo: array_init::array_init(|i| (i == 0) as u8),
                tag: DELEGATION_TX_TAG,
                source_pk: from.into_compressed(),
                receiver_pk: to.into_compressed(),
                token_id: 1,
                amount: 0,
                token_locked: false,
            }
        }

        pub fn set_valid_until(mut self, global_slot: u32) -> Self {
            self.valid_until = global_slot;

            self
        }

        pub fn set_memo(mut self, memo: [u8; MEMO_BYTES - 2]) -> Self {
            self.memo[0] = 0x01;
            self.memo[1] = (MEMO_BYTES - 2) as u8;
            self.memo[2..].copy_from_slice(&memo[..]);

            self
        }

        pub fn set_memo_str(mut self, memo: &str) -> Self {
            self.memo[0] = 0x01;
            self.memo[1] = std::cmp::min(memo.len(), MEMO_BYTES - 2) as u8;
            let memo = format!("{:\0<32}", memo); // Pad user-supplied memo with zeros
            self.memo[2..]
                .copy_from_slice(&memo.as_bytes()[..std::cmp::min(memo.len(), MEMO_BYTES - 2)]);
            // Anything beyond MEMO_BYTES is truncated

            self
        }
    }
}
