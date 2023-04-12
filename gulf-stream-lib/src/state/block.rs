use std::ops::Add;

use crate::{ed25519::publickey::PublicKey, utils::serde::BytesSerialize};

use super::{
    blockhash::Blockhash,
    transaction::{BalanceDelta, Transaction},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: u64,
    pub blockhash: Blockhash,
    pub previous_blockhash: Blockhash,
    pub transactions: Vec<TransactionState>,
    pub nonce: u64,
}

impl Block {
    pub fn create_block(
        index: u64,
        previous_blockhash: &Blockhash,
        transactions: Vec<TransactionState>,
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
            |res: BalanceDelta, tx: &TransactionState| res.add(tx.into_tx().get_balance_delta(pk)),
        )
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::genesis()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Success(Transaction),
    Fail(Transaction),
    Pending(Transaction),
}

impl Into<Transaction> for TransactionState {
    fn into(self) -> Transaction {
        match self {
            TransactionState::Success(tx) => tx,
            TransactionState::Fail(tx) => tx,
            TransactionState::Pending(tx) => tx,
        }
    }
}

impl<'a> Into<&'a Transaction> for &'a TransactionState {
    fn into(self) -> &'a Transaction {
        match self {
            TransactionState::Success(tx) => tx,
            TransactionState::Fail(tx) => tx,
            TransactionState::Pending(tx) => tx,
        }
    }
}

impl<'a> TransactionState {
    pub fn into_tx(&'a self) -> &'a Transaction {
        Into::<&Transaction>::into(self)
    }

    pub fn get_raw_txs(txs: &Vec<TransactionState>) -> Vec<u8> {
        txs.iter().flat_map(|tx| tx.into_tx().serialize()).collect()
    }

    pub fn success(self) -> Self {
        match self {
            TransactionState::Success(_) => self,
            TransactionState::Fail(tx) => TransactionState::Success(tx),
            TransactionState::Pending(tx) => TransactionState::Success(tx),
        }
    }

    pub fn is_pending(&self) -> bool {
        match &self {
            TransactionState::Pending(_) => true,
            _ => false,
        }
    }
}
