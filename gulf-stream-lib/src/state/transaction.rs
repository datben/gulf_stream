use ed25519_dalek::Signature;
use ed25519_dalek::{Digest, Sha512};
use serde::{Deserialize, Serialize};

use super::publick_key::PublicKey;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub payer: PublicKey,
    pub msg: Vec<u8>,
    pub signature: Signature,
}

impl Into<Vec<u8>> for Transaction {
    fn into(self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.payer.0.as_bytes());
        vec.extend(self.msg);
        vec.extend(self.signature.to_bytes());
        vec
    }
}

impl Transaction {
    pub fn is_valid(&self) -> bool {
        let mut prehashed: Sha512 = Sha512::new();
        prehashed.update(&self.msg[..]);
        self.payer
            .0
            .verify_prehashed(prehashed, None, &self.signature)
            .is_ok()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionMessage {
    Mint { amount: u64 },
    Transfer { to: PublicKey, amount: u64 },
}
