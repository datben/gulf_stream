use ed25519_dalek::Signature;

tonic::include_proto!("pb");

impl TryInto<crate::state::block::Block> for Block {
    type Error = crate::err::GulfStreamError;

    fn try_into(self) -> Result<crate::state::block::Block, Self::Error> {
        Ok(crate::state::block::Block {
            index: self.index,
            transactions: self
                .transactions
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<crate::state::transaction::Transaction>, Self::Error>>()?,
            blockhash: self.blockhash.into(),
            previous_blockhash: self.previous_blockhash.into(),
            nonce: self.nonce,
        })
    }
}

impl TryFrom<crate::state::block::Block> for Block {
    type Error = crate::err::GulfStreamError;

    fn try_from(value: crate::state::block::Block) -> Result<Self, Self::Error> {
        Ok(Block {
            index: value.index,
            transactions: value
                .transactions
                .into_iter()
                .map(TryInto::<Transaction>::try_into)
                .collect::<Result<Vec<Transaction>, Self::Error>>()?,
            blockhash: value.blockhash.into(),
            previous_blockhash: value.previous_blockhash.into(),
            nonce: value.nonce,
        })
    }
}

impl TryInto<crate::state::transaction::Transaction> for Transaction {
    type Error = crate::err::GulfStreamError;

    fn try_into(self) -> Result<crate::state::transaction::Transaction, Self::Error> {
        Ok(crate::state::transaction::Transaction {
            payer: bincode::deserialize(self.payer.as_ref())?,
            msg: bincode::deserialize(self.msg.as_ref())?,
            signature: bincode::deserialize(self.signature.as_ref())?,
        })
    }
}

impl TryFrom<crate::state::transaction::Transaction> for Transaction {
    type Error = crate::err::GulfStreamError;

    fn try_from(value: crate::state::transaction::Transaction) -> Result<Self, Self::Error> {
        Ok(Self {
            payer: value.payer.0.to_bytes().to_vec(),
            msg: bincode::serialize(&value.msg)?,
            signature: value.signature.0.to_bytes().to_vec(),
        })
    }
}
