use super::blockhash::Blockhash;

#[derive(Debug)]
#[repr(C)]
pub struct Block {
    pub index: u64,
    pub blockhash: Blockhash,
    pub previous_blockhash: Blockhash,
    pub nonce: u64,
}

impl Block {
    pub fn create_block(index: u64, previous_blockhash: Blockhash, nonce: u64) -> Self {
        Self {
            index,
            blockhash: Blockhash::from_data(index, &previous_blockhash, nonce),
            previous_blockhash: previous_blockhash.to_owned(),
            nonce,
        }
    }

    pub fn compute_blockhash(&self) -> Blockhash {
        Blockhash::from_data(self.index, &self.previous_blockhash, self.nonce)
    }
}
