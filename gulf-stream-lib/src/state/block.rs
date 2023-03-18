use super::blockhash::Blockhash;

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Block {
    pub index: u64,
    pub blockhash: Blockhash,
    pub previous_blockhash: Blockhash,
    pub nonce: u64,
}

impl Block {
    pub fn create_block(index: u64, previous_blockhash: &Blockhash, nonce: u64) -> Self {
        Self {
            index,
            blockhash: Blockhash::from_data(index, previous_blockhash, nonce),
            previous_blockhash: previous_blockhash.to_owned(),
            nonce,
        }
    }

    pub fn compute_blockhash(&self) -> Blockhash {
        Blockhash::from_data(self.index, &self.previous_blockhash, self.nonce)
    }

    pub fn genesis() -> Self {
        let previous_blockhash = Blockhash("genesis".as_bytes().to_vec());
        Self {
            index: 0,
            blockhash: Blockhash::from_data(0, &previous_blockhash, 0),
            previous_blockhash,
            nonce: 0,
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::genesis()
    }
}
