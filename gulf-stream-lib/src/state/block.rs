use std::ops::Add;

use crate::ed25519::publickey::PublicKey;

use super::{
    blockhash::Blockhash,
    transaction::{BalanceDelta, Transaction},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: u64,
    pub blockhash: Blockhash,
    pub previous_blockhash: Blockhash,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn create_block(
        index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Self {
        Self {
            index,
            blockhash: Blockhash::from_data(index, previous_blockhash, &transactions, nonce),
            transactions,
            previous_blockhash: previous_blockhash.to_owned(),
            nonce,
        }
    }

    pub fn compute_blockhash(&self) -> Blockhash {
        Blockhash::from_data(
            self.index,
            &self.previous_blockhash,
            &self.transactions,
            self.nonce,
        )
    }

    pub fn genesis() -> Self {
        let previous_blockhash = Blockhash("genesis".as_bytes().to_vec());
        Self {
            index: 0,
            blockhash: Blockhash::from_data(0, &previous_blockhash, &vec![], 0),
            transactions: vec![],
            previous_blockhash,
            nonce: 0,
        }
    }

    pub fn get_balance_delta(&self, pk: &PublicKey) -> BalanceDelta {
        self.transactions.iter().fold(
            BalanceDelta::default(),
            |res: BalanceDelta, tx: &Transaction| res.add(tx.get_balance_delta(pk)),
        )
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::genesis()
    }
}
