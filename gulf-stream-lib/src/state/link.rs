use super::{block::Block, blockhash::Blockhash, transaction::BalanceDelta};
use crate::{ed25519::publickey::PublicKey, err::*};
use std::{
    fmt::Display,
    ops::Add,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct Link {
    pub block_parent: Option<Arc<Link>>,
    pub block: Block,
    pub next_blocks: Mutex<Vec<Arc<Link>>>,
}

impl Link {
    pub fn try_find_block(self: Arc<Link>, blockhash: &Blockhash, index: u64) -> Result<Arc<Link>> {
        if index == self.block.index {
            if blockhash.eq(&self.block.blockhash) {
                Ok(self.clone())
            } else {
                Err(GulfStreamError::BlockNotFound)
            }
        } else if index < self.block.index {
            return Err(GulfStreamError::WrongIndex);
        } else {
            return self.next_blocks.try_lock()?.iter().fold(
                Err(GulfStreamError::BlockNotFound),
                |res, link| {
                    return match res {
                        Ok(result) => Ok(result),
                        Err(_) => link.clone().try_find_block(blockhash, index),
                    };
                },
            );
        }
    }

    pub fn try_insert(self: Arc<Link>, block: &Block) -> Result<Arc<Link>> {
        return if block.index == self.block.index + 1 {
            if block.previous_blockhash.eq(&self.block.blockhash) {
                self.unsafe_insert(block.clone())
            } else {
                Err(GulfStreamError::WrongParentBlockhash)
            }
        } else {
            Err(GulfStreamError::WrongIndex)
        };
    }

    pub fn get_balance(&self, pk: &PublicKey) -> BalanceDelta {
        let current_delta = self.block.get_balance_delta(pk);
        return if let Some(block_parent) = &self.block_parent {
            let last_delta = block_parent.clone().get_balance(pk);
            current_delta.add(last_delta)
        } else {
            current_delta
        };
    }

    fn unsafe_insert(self: Arc<Link>, block: Block) -> Result<Arc<Link>> {
        let new_link = Arc::new(Self {
            block_parent: self.clone().into(),
            block,
            next_blocks: vec![].into(),
        });
        self.next_blocks.try_lock()?.push(new_link.clone());
        return Ok(new_link);
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "current {:?}\n next block {:?}",
            self.block,
            self.next_blocks
                .try_lock()
                .unwrap()
                .iter()
                .map(|link| { return link.block.clone() })
                .collect::<Vec<Block>>()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod get_balance {
        use crate::state::transaction::{Transaction, TransactionMessage};

        use super::*;

        #[test]
        fn casual() {
            let pk1 = PublicKey::random();
            let pk2 = PublicKey::random();

            let link = Arc::new(Link::default());
            let block1 = Block::create_block(
                1,
                &link.block.blockhash,
                vec![
                    Transaction {
                        msg: TransactionMessage::Mint { amount: 12 },
                        payer: pk1.to_owned(),
                        signature: Default::default(),
                        gas: 0,
                    }
                    .into_tx_state()
                    .success(),
                    Transaction {
                        msg: TransactionMessage::Mint { amount: 57 },
                        payer: pk2.to_owned(),
                        signature: Default::default(),
                        gas: 0,
                    }
                    .into_tx_state()
                    .success(),
                ],
                0,
            );

            let block2 = Block::create_block(
                2,
                &block1.blockhash,
                vec![Transaction {
                    msg: TransactionMessage::Transfer {
                        to: pk2.to_owned(),
                        amount: 5,
                    },
                    payer: pk1.to_owned(),
                    signature: Default::default(),
                    gas: 0,
                }
                .into_tx_state()
                .success()],
                0,
            );

            link.clone().try_insert(&block1).unwrap();
            link.clone().next_blocks.lock().unwrap()[0]
                .clone()
                .try_insert(&block2)
                .unwrap();

            let balance1 = link
                .clone()
                .try_find_block(&block2.blockhash, block2.index)
                .unwrap()
                .get_balance(&pk1);

            let balance2 = link
                .clone()
                .try_find_block(&block2.blockhash, block2.index)
                .unwrap()
                .get_balance(&pk2);

            assert_eq!(balance1, BalanceDelta::Pos(7));
            assert_eq!(balance2, BalanceDelta::Pos(62));
        }
    }
}
