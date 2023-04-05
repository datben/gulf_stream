use ed25519_dalek::{Digest, Sha512};
use ed25519_dalek::{PublicKey, Signature};

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub payer: PublicKey,
    pub msg: Vec<u8>,
    pub signature: Signature,
}

impl Into<Vec<u8>> for Transaction {
    fn into(self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.payer.as_bytes());
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
            .verify_prehashed(prehashed, None, &self.signature)
            .is_ok()
    }
}
