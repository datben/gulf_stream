use sha2::{Digest, Sha256};

use super::transaction::Transaction;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Blockhash(pub Vec<u8>);

impl std::fmt::Display for Blockhash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().map(|byte| write!(f, "{:x?}", byte)).collect()
    }
}

impl From<Vec<u8>> for Blockhash {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl Into<Vec<u8>> for Blockhash {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl AsRef<[u8]> for Blockhash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Blockhash {
    pub fn from_data(
        index: u64,
        previous_blockhash: &Blockhash,
        transactions: &Vec<Transaction>,
        nonce: u64,
    ) -> Blockhash {
        let mut hasher = Sha256::new();
        hasher.update(nonce.to_be_bytes());
        hasher.update(index.to_be_bytes());
        hasher.update(previous_blockhash);
        transactions.iter().for_each(|tx| {
            hasher.update(Into::<Vec<u8>>::into(tx.to_owned()).as_slice());
        });
        hasher.finalize().to_vec().into()
    }
}
