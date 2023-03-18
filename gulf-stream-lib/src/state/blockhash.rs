use sha2::{Digest, Sha256};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Blockhash(pub Vec<u8>);

impl std::fmt::Display for Blockhash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x?}", self.0)
    }
}

impl From<Vec<u8>> for Blockhash {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for Blockhash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Blockhash {
    pub fn from_data(index: u64, previous_blockhash: &Blockhash, nonce: u64) -> Blockhash {
        let mut hasher = Sha256::new();
        hasher.update(nonce.to_be_bytes());
        hasher.update(index.to_be_bytes());
        hasher.update(previous_blockhash);
        hasher.finalize().to_vec().into()
    }
}
