use super::block::Block;
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
    pub fn try_insert(self: Arc<Link>, block: &Block) -> Result<()> {
        if block.index == self.block.index + 1 {
            if block.previous_blockhash.eq(&self.block.blockhash) {
                self.insert(block.clone());
                return Ok(());
            } else {
                return Err(Error::WrongParentBlockhash);
            }
        } else if block.index <= self.block.index {
            return if let Some(block_parent) = &self.block_parent {
                block_parent.clone().try_insert(block)
            } else {
                Err(Error::NoMoreOlderBlocks)
            };
        } else {
            return self
                .next_blocks
                .try_lock()
                .unwrap()
                .iter()
                .fold(Err(Error::Default), |res, link| {
                    return match res {
                        Ok(_) => Ok(()),
                        Err(_) => link.clone().try_insert(block),
                    };
                })
                .map_err(|_| Error::MissingIntermediateBlocks);
        }
    }

    fn insert(self: Arc<Link>, block: Block) {
        self.next_blocks.try_lock().unwrap().push(
            Self {
                block_parent: self.clone().into(),
                block,
                next_blocks: vec![].into(),
            }
            .into(),
        )
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
pub mod test {
    use super::*;

    pub mod link {
        use super::*;

        #[test]
        pub fn create_link() -> Result<()> {
            let genesis = Arc::new(Link::default());

            let next_block_1_0 = Block::create_block(1, &genesis.block.blockhash, 0);
            let next_block_1_1 = Block::create_block(1, &genesis.block.blockhash, 1);
            let next_block_2_0 = Block::create_block(2, &next_block_1_0.blockhash, 0);
            let next_block_2_1 = Block::create_block(2, &next_block_1_1.blockhash, 0);
            let next_block_4_0 = Block::create_block(4, &next_block_1_0.blockhash, 0);

            genesis.clone().try_insert(&next_block_1_0)?;
            genesis.clone().try_insert(&next_block_1_1)?;
            genesis.clone().try_insert(&next_block_2_0)?;
            genesis
                .next_blocks
                .try_lock()
                .unwrap()
                .get(1)
                .unwrap()
                .clone()
                .try_insert(&next_block_2_1)?;

            assert_eq!(
                genesis
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .block,
                next_block_1_0
            );

            assert_eq!(
                genesis
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .block,
                next_block_1_1
            );

            assert_eq!(
                genesis
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .block,
                next_block_2_0
            );

            assert_eq!(
                genesis
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .next_blocks
                    .try_lock()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .block,
                next_block_2_1
            );

            assert_eq!(
                genesis.clone().try_insert(&next_block_4_0),
                Err(Error::MissingIntermediateBlocks)
            );

            Ok(())
        }
    }
}
