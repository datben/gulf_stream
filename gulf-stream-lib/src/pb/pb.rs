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
