use ed25519_dalek::{PublicKey, Signature};

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

impl TryInto<crate::state::transaction::Transaction> for Transaction {
    type Error = crate::err::GulfStreamError;

    fn try_into(self) -> Result<crate::state::transaction::Transaction, Self::Error> {
        Ok(crate::state::transaction::Transaction {
            payer: PublicKey::from_bytes(self.payer.as_ref())
                .map_err(|_| Self::Error::FailDeserialisationOfTransaction)?,
            msg: self.msg,
            signature: Signature::from_bytes(self.signature.as_ref())
                .map_err(|_| Self::Error::FailDeserialisationOfTransaction)?,
        })
    }
}

impl From<crate::state::transaction::Transaction> for Transaction {
    fn from(value: crate::state::transaction::Transaction) -> Self {
        Self {
            payer: value.payer.to_bytes().to_vec(),
            msg: value.msg,
            signature: value.signature.to_bytes().to_vec(),
        }
    }
}
