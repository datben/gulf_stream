use ed25519_dalek::Signature;

use crate::state::publickey::PublicKey;

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

impl From<crate::state::block::Block> for Block {
    fn from(value: crate::state::block::Block) -> Self {
        Block {
            index: value.index,
            transactions: value.transactions.into_iter().map(Into::into).collect(),
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
            payer: PublicKey(
                ed25519_dalek::PublicKey::from_bytes(self.payer.as_ref())
                    .map_err(|_| Self::Error::FailDeserialisationOfTransaction)?,
            ),
            msg: self.msg,
            signature: Signature::from_bytes(self.signature.as_ref())
                .map_err(|_| Self::Error::FailDeserialisationOfTransaction)?,
        })
    }
}

impl From<crate::state::transaction::Transaction> for Transaction {
    fn from(value: crate::state::transaction::Transaction) -> Self {
        Self {
            payer: value.payer.0.to_bytes().to_vec(),
            msg: value.msg,
            signature: value.signature.to_bytes().to_vec(),
        }
    }
}
