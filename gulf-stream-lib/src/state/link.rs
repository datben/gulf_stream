use super::{block::Block, blockhash::Blockhash};
use crate::err::*;
use std::{
    fmt::Display,
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
