use std::ops::Add;

use crate::ed25519::{publickey::PublicKey, signature::Signature};
use crate::err::Result;
use crate::utils::serde::{BytesDeserialize, BytesSerialize};
use ed25519_dalek::{Digest, Sha512};

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub payer: PublicKey,
    pub msg: TransactionMessage,
    pub signature: Signature,
}

impl Transaction {
    pub fn sign_is_valid(&self) -> bool {
        let mut prehashed: Sha512 = Sha512::new();
        prehashed.update(&TransactionMessage::serialize(&self.msg)[..]);
        self.payer
            .0
            .verify_prehashed(prehashed, None, &self.signature.0)
            .is_ok()
    }

    pub fn tx_msg_is_valid(&self) -> bool {
        match &self.msg {
            TransactionMessage::Mint { .. } => true,
            TransactionMessage::Transfer { to, .. } => self.payer.ne(to),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.sign_is_valid() && self.tx_msg_is_valid()
    }

    pub fn get_raw_txs(txs: &Vec<Self>) -> Vec<u8> {
        txs.iter().flat_map(|tx| tx.serialize()).collect()
    }

    pub fn get_balance_delta(&self, pk: &PublicKey) -> BalanceDelta {
        match &self.msg {
            TransactionMessage::Mint { amount } if self.payer.eq(pk) => BalanceDelta::Pos(*amount),
            TransactionMessage::Transfer { to, amount } if to.eq(pk) => BalanceDelta::Pos(*amount),
            TransactionMessage::Transfer { to, amount } if to.ne(pk) => BalanceDelta::Neg(*amount),
            _ => Default::default(),
        }
    }

    #[cfg(test)]
    pub fn random() -> Self {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BalanceDelta {
    Pos(u64),
    Neg(u64),
}

impl Default for BalanceDelta {
    fn default() -> Self {
        Self::Pos(0)
    }
}

impl Add for BalanceDelta {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (BalanceDelta::Pos(a), BalanceDelta::Pos(b)) => Self::Pos(a + b),
            (BalanceDelta::Pos(a), BalanceDelta::Neg(b)) if a > b => Self::Pos(a - b),
            (BalanceDelta::Pos(a), BalanceDelta::Neg(b)) => Self::Neg(b - a),
            (BalanceDelta::Neg(a), BalanceDelta::Pos(b)) if a > b => Self::Neg(a - b),
            (BalanceDelta::Neg(a), BalanceDelta::Pos(b)) => Self::Pos(b - a),
            (BalanceDelta::Neg(a), BalanceDelta::Neg(b)) => Self::Neg(a + b),
        }
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

    mod balance_delta {
        use super::*;

        #[test]
        fn add() {
            let a = BalanceDelta::Pos(10);
            let b = BalanceDelta::Neg(5);

            assert_eq!(a.add(b), BalanceDelta::Pos(5));
            assert_eq!(b.add(a), BalanceDelta::Pos(5));

            let a = BalanceDelta::Pos(5);
            let b = BalanceDelta::Neg(10);

            assert_eq!(a.add(b), BalanceDelta::Neg(5));
            assert_eq!(b.add(a), BalanceDelta::Neg(5));
        }
    }
}
