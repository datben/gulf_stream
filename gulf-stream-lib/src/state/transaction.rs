use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Add;

use crate::ed25519::{publickey::PublicKey, signature::Signature};
use crate::err::Result;
use crate::utils::serde::{BytesDeserialize, BytesSerialize};
use ed25519_dalek::{Digest, Sha512};

use super::block::TransactionState;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub blockheight: u64,
    pub gas: u64,
    pub msg: TransactionMessage,
    pub payer: PublicKey,
    pub signature: Signature,
}

impl Transaction {
    pub fn get_raw_txs(txs: &Vec<Self>) -> Vec<u8> {
        txs.iter().flat_map(|tx| tx.serialize()).collect()
    }

    pub fn get_balance_deltas_from_txs(txs: &Vec<Self>) -> HashMap<PublicKey, BalanceDelta> {
        txs.iter().fold(HashMap::new(), |mut res, tx| {
            BalanceDelta::update_table(tx, &mut res);
            res
        })
    }

    pub fn get_involved_pk_from_txs(txs: &Vec<Self>) -> Vec<PublicKey> {
        txs.iter().flat_map(|tx| tx.get_involved_pk()).collect()
    }
}

impl Transaction {
    pub fn is_valid(&self, payer_balance: u64) -> bool {
        self.sign_is_valid() && self.tx_msg_is_valid() && self.is_valid_for_payer(payer_balance)
    }

    pub fn get_balance_deltas(&self) -> HashMap<PublicKey, BalanceDelta> {
        let mut res = HashMap::new();
        BalanceDelta::update_table(self, &mut res);
        res
    }

    pub fn get_involved_pk(&self) -> Vec<PublicKey> {
        match &self.msg {
            TransactionMessage::Mint { .. } => vec![self.payer.clone()],
            TransactionMessage::Transfer { to, .. } => vec![self.payer.clone(), to.clone()],
        }
    }

    pub fn get_balance_delta_from_pk(&self, pk: &PublicKey) -> BalanceDelta {
        match &self.msg {
            TransactionMessage::Mint { amount } if self.payer.eq(pk) => BalanceDelta::Pos(*amount),
            TransactionMessage::Transfer { to, amount } if to.eq(pk) => BalanceDelta::Pos(*amount),
            TransactionMessage::Transfer { to, amount } if to.ne(pk) => BalanceDelta::Neg(*amount),
            _ => Default::default(),
        }
    }
    pub fn serialize_content(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.blockheight.serialize());
        vec.extend(self.gas.serialize());
        vec.extend(self.msg.serialize());
        vec
    }

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

    fn is_valid_for_payer(&self, payer_balance: u64) -> bool {
        match &self.msg {
            TransactionMessage::Mint { .. } => payer_balance.ge(&self.gas),
            TransactionMessage::Transfer { to: _, amount } => {
                payer_balance.ge(&(*amount + self.gas))
            }
        }
    }

    pub fn into_tx_state(self) -> TransactionState {
        self.into()
    }

    #[cfg(test)]
    pub fn random(blockheight: u64) -> Self {
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
            gas: 50,
            blockheight,
        }
    }
}

impl Into<TransactionState> for Transaction {
    fn into(self) -> TransactionState {
        TransactionState::Pending(self)
    }
}

impl BytesSerialize for Transaction {
    fn serialize(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend(self.serialize_content());
        vec.extend(self.payer.serialize());
        vec.extend(self.signature.serialize());
        vec
    }
}

impl BytesDeserialize for Transaction {
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        Ok(Self {
            blockheight: u64::deserialize(buf)?,
            gas: u64::deserialize(buf)?,
            msg: TransactionMessage::deserialize(buf)?,
            payer: PublicKey::deserialize(buf)?,
            signature: Signature::deserialize(buf)?,
        })
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gas.partial_cmp(&other.gas)
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.blockheight == other.blockheight
            && self.gas == other.gas
            && self.msg == other.msg
            && self.payer == other.payer
            && self.signature == other.signature
    }
}

impl Eq for Transaction {}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gas.cmp(&other.gas)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TransactionMessage {
    Mint { amount: u64 },
    Transfer { to: PublicKey, amount: u64 },
}

impl Default for TransactionMessage {
    fn default() -> Self {
        Self::Mint { amount: 77 }
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
                println!("here");
                return Ok(Self::Transfer {
                    to: PublicKey::deserialize(value)?,
                    amount: u64::deserialize(value)?,
                });
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

impl BalanceDelta {
    pub fn to_u64(self) -> Option<u64> {
        match self {
            BalanceDelta::Pos(a) => Some(a),
            BalanceDelta::Neg(_) => None,
        }
    }

    pub fn update_table(tx: &Transaction, table: &mut HashMap<PublicKey, BalanceDelta>) {
        let involved_pks = tx.get_involved_pk();
        involved_pks.iter().for_each(|pk| {
            table
                .entry(pk.clone())
                .and_modify(|e: &mut BalanceDelta| *e = e.add(tx.get_balance_delta_from_pk(pk)))
                .or_insert(tx.get_balance_delta_from_pk(pk));
        });
    }

    pub fn is_positive_or_nil(&self) -> bool {
        match self {
            BalanceDelta::Pos(_) => true,
            BalanceDelta::Neg(a) => 0.eq(a),
        }
    }
}

impl Default for BalanceDelta {
    fn default() -> Self {
        Self::Pos(0)
    }
}

impl PartialOrd for BalanceDelta {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (BalanceDelta::Pos(a), BalanceDelta::Pos(b)) => a.partial_cmp(b),
            (BalanceDelta::Neg(a), BalanceDelta::Neg(b)) => b.partial_cmp(a),
            (BalanceDelta::Pos(a), BalanceDelta::Neg(b)) if *a != 0 && *b != 0 => {
                Some(Ordering::Greater)
            }
            (BalanceDelta::Neg(a), BalanceDelta::Pos(b)) if *a != 0 && *b != 0 => {
                Some(Ordering::Less)
            }
            _ => Some(Ordering::Equal),
        }
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
        let tx = Transaction::random(1);
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
