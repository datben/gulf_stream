tonic::include_proto!("pb");

impl Into<crate::state::block::Block> for Block {
    fn into(self) -> crate::state::block::Block {
        crate::state::block::Block {
            index: self.index,
            blockhash: self.blockhash.into(),
            previous_blockhash: self.previous_blockhash.into(),
            nonce: self.nonce,
        }
    }
}

impl From<crate::state::block::Block> for Block {
    fn from(value: crate::state::block::Block) -> Self {
        Block {
            index: value.index,
            blockhash: value.blockhash.into(),
            previous_blockhash: value.previous_blockhash.into(),
            nonce: value.nonce,
        }
    }
}
