use super::{block::Block, link::Link};
use crate::err::*;
use std::{sync::Arc, vec};

pub struct Blockchain {
    genesis: Arc<Link>,
    latest_links: Vec<Arc<Link>>,
    heighest_link: Arc<Link>,
}

impl Blockchain {
    pub const LASTEST_LINK_LENGTH: usize = 10;

    pub fn try_insert(&mut self, block: &Block) -> Result<()> {
        if self.is_valid(block) {
            if let Some(new_link) = self.latest_links.iter().fold(None, |res, link| match res {
                Some(_) => res,
                None => {
                    if let Ok(new_link) = link.clone().try_insert(block) {
                        Some(new_link)
                    } else {
                        None
                    }
                }
            }) {
                self.update_latest(new_link.clone());
                self.update_heighest(new_link);
                return Ok(());
            } else {
                if let Ok(previous_link) = self
                    .genesis
                    .clone()
                    .try_find_block(&block.previous_blockhash, block.index - 1)
                {
                    let new_link = previous_link.try_insert(block)?;
                    self.update_latest(new_link.clone());
                    self.update_heighest(new_link);

                    return Ok(());
                } else {
                    return Err(GulfStreamError::DidNotFindPreviousBlock);
                }
            }
        } else {
            Err(GulfStreamError::BlockIsNotValid)
        }
    }

    fn update_latest(&mut self, new_link: Arc<Link>) {
        if self.latest_links.len() >= Self::LASTEST_LINK_LENGTH {
            self.latest_links.pop();
        }
        self.latest_links.insert(0, new_link);
    }

    fn update_heighest(&mut self, new_link: Arc<Link>) {
        if self.heighest_link.block.index <= new_link.block.index {
            self.heighest_link = new_link
        }
    }

    fn is_valid(&self, _block: &Block) -> bool {
        true
    }

    pub fn get_latest(&self) -> Arc<Link> {
        self.latest_links[0].clone()
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        let genesis: Arc<Link> = Default::default();
        Self {
            latest_links: vec![genesis.clone()],
            heighest_link: genesis.clone(),
            genesis,
        }
    }
}
