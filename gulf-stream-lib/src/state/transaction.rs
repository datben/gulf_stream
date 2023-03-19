use ed25519_dalek::{PublicKey, Signature, Verifier};

pub struct Transaction {
    pub payer: PublicKey,
    pub msg: Vec<u8>,
    pub signature: Signature,
}

impl Transaction {
    pub fn is_valid(&self) -> bool {
        self.payer.verify(&self.msg[..], &self.signature).is_ok()
    }
}
