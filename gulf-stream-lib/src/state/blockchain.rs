use super::{block::Block, link::Link};
use crate::err::*;
use std::sync::Arc;

#[derive(Default)]
pub struct Blockchain {
    pub genesis: Arc<Link>,
    pub latest_link: Arc<Link>,
}

impl Blockchain {
    pub fn try_insert(&self, block: &Block) -> Result<()> {
        if self.is_valid(block) {
            self.latest_link.clone().try_insert(block)
        } else {
            Err(Error::BlockIsNotValid)
        }
    }

    fn is_valid(&self, _block: &Block) -> bool {
        true
    }
}
