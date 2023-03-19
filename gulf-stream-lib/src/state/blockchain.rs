use super::{block::Block, link::Link};
use crate::err::*;
use std::sync::Arc;

#[derive(Default)]
pub struct Blockchain {
    pub genesis: Arc<Link>,
    pub latest_links: Vec<Arc<Link>>,
}

impl Blockchain {
    pub fn try_insert(&mut self, block: &Block) -> Result<()> {
        if self.is_valid(block) {
            if let Some((index_to_update, new_link)) =
                self.latest_links
                    .iter()
                    .enumerate()
                    .fold(None, |res, (index, link)| match res {
                        Some(_) => res,
                        None => {
                            if let Ok(new_link) = link.clone().try_insert(block) {
                                Some((index, new_link))
                            } else {
                                None
                            }
                        }
                    })
            {
                self.latest_links.remove(index_to_update);
                self.latest_links.insert(0, new_link);
                return Ok(());
            } else {
                if let Ok(previous_link) = self
                    .genesis
                    .clone()
                    .try_find_block(&block.previous_blockhash, block.index - 1)
                {
                    let new_link = previous_link.try_insert(block)?;
                    if self.latest_links.len() >= 10 {
                        self.latest_links.pop();
                    }
                    self.latest_links.insert(0, new_link);
                    return Ok(());
                } else {
                    return Err(GulfStreamError::DidNotFindPreviousBlock);
                }
            }
        } else {
            Err(GulfStreamError::BlockIsNotValid)
        }
    }

    fn is_valid(&self, _block: &Block) -> bool {
        true
    }
}
