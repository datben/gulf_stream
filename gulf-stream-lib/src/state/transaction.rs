use crate::ed25519::{publickey::PublicKey, signature::Signature};
use crate::err::Result;
use ed25519_dalek::{Digest, Sha512};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub payer: PublicKey,
    pub msg: TransactionMessage,
    pub signature: Signature,
}

impl Transaction {
    pub fn is_valid(&self) -> Result<bool> {
        let mut prehashed: Sha512 = Sha512::new();
        prehashed.update(&bincode::serialize(&self.msg)?[..]);
        Ok(self
            .payer
            .0
            .verify_prehashed(prehashed, None, &self.signature.0)
            .is_ok())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionMessage {
    Mint { amount: u64 },
    Transfer { to: PublicKey, amount: u64 },
}

impl Default for TransactionMessage {
    fn default() -> Self {
        Self::Transfer {
            to: PublicKey::default(),
            amount: 77,
        }
    }
}

impl TransactionMessage {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(Into::into)
    }
}

impl Transaction {
    pub fn try_get_raw_txs(txs: &Vec<Self>) -> Result<Vec<u8>> {
        Ok(txs
            .iter()
            .map(|tx| bincode::serialize(&tx).map_err(Into::into))
            .collect::<Result<Vec<Vec<u8>>>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn se_de_tx() {
        let tx = TransactionMessage::default();
        let se = bincode::serialize(&tx).unwrap();
        let de: TransactionMessage = bincode::deserialize(se.as_slice()).unwrap();
        assert_eq!(de, tx);
    }
}
