use crate::ed25519::{publickey::PublicKey, signature::Signature};
use crate::err::Result;
use crate::utils::serde::{BytesDeserialize, BytesSerialize};
use ed25519_dalek::{Digest, Keypair, Sha512};
use rand::rngs::OsRng;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub payer: PublicKey,
    pub msg: TransactionMessage,
    pub signature: Signature,
}

impl Transaction {
    pub fn is_valid(&self) -> Result<bool> {
        let mut prehashed: Sha512 = Sha512::new();
        prehashed.update(&TransactionMessage::serialize(&self.msg)[..]);
        Ok(self
            .payer
            .0
            .verify_prehashed(prehashed, None, &self.signature.0)
            .is_ok())
    }

    fn random() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let msg = TransactionMessage::default();

        let mut prehashed: Sha512 = Sha512::new();

        prehashed.update(msg.serialize());

        let signature = keypair.sign_prehashed(prehashed, None).unwrap();

        Transaction {
            payer: keypair.public.into(),
            msg,
            signature: signature.into(),
        }
    }
}

impl BytesSerialize for Transaction {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.payer.serialize());
        vec.extend(self.msg.serialize());
        vec.extend(self.signature.serialize());
        vec
    }
}

impl BytesDeserialize for Transaction {
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        let payer = PublicKey::deserialize(buf)?;
        let msg = TransactionMessage::deserialize(buf)?;
        let signature = Signature::deserialize(buf)?;
        Ok(Self {
            payer,
            msg,
            signature,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
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

impl BytesSerialize for TransactionMessage {
    fn serialize(&self) -> Vec<u8> {
        match self {
            TransactionMessage::Mint { amount } => {
                let mut vec = vec![0];
                vec.extend(amount.serialize());
                return vec;
            }
            TransactionMessage::Transfer { to, amount } => {
                let mut vec = vec![1];
                vec.extend(to.serialize());
                vec.extend(amount.serialize());
                return vec;
            }
        }
    }
}

impl BytesDeserialize for TransactionMessage {
    fn deserialize(value: &mut &[u8]) -> Result<Self> {
        let index = value[0];
        *value = &value[1..];
        match index {
            0 => {
                return Ok(Self::Mint {
                    amount: u64::deserialize(value)?,
                });
            }
            1 => {
                return Ok(Self::Transfer {
                    to: PublicKey::deserialize(value)?,
                    amount: u64::deserialize(value)?,
                })
            }
            _ => Err(crate::err::GulfStreamError::Default),
        }
    }
}

impl Transaction {
    pub fn get_raw_txs(txs: &Vec<Self>) -> Vec<u8> {
        txs.iter().flat_map(|tx| tx.serialize()).collect()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn se_de_txm() {
        let txm = TransactionMessage::default();
        let se = txm.serialize();
        let de = TransactionMessage::deserialize(&mut se.as_slice()).unwrap();
        assert_eq!(de, txm);
    }

    #[test]
    fn se_de_tx() {
        let tx = Transaction::random();
        let se = tx.serialize();
        let de = Transaction::deserialize(&mut se.as_slice()).unwrap();
        assert_eq!(de, tx);
    }
}
