use ed25519_dalek::{Digest, Sha512};
use ed25519_dalek::{PublicKey, Signature};

pub struct Transaction {
    pub payer: PublicKey,
    pub msg: Vec<u8>,
    pub signature: Signature,
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
