use crate::{
    ed25519::{publickey::PublicKey, signature::Signature},
    state::transaction::TransactionMessage,
    utils::serde::BytesDeserialize,
    utils::serde::BytesSerialize,
};

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
            payer: PublicKey::deserialize(&mut self.payer.as_ref())?,
            msg: TransactionMessage::deserialize(&mut self.msg.as_ref())?,
            signature: Signature::deserialize(&mut self.signature.as_ref())?,
        })
    }
}

impl From<crate::state::transaction::Transaction> for Transaction {
    fn from(value: crate::state::transaction::Transaction) -> Self {
        Self {
            payer: value.payer.0.to_bytes().to_vec(),
            msg: TransactionMessage::serialize(&value.msg),
            signature: value.signature.0.to_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let raw_tx = Transaction {
            payer: vec![
                110, 244, 56, 156, 170, 232, 45, 208, 70, 45, 1, 194, 190, 0, 250, 95, 236, 230,
                83, 70, 255, 253, 51, 219, 174, 30, 197, 82, 243, 235, 57, 228,
            ],
            msg: vec![
                1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 77, 0, 0, 0, 0, 0, 0, 0,
            ],
            signature: vec![
                158, 110, 193, 161, 204, 34, 23, 164, 123, 203, 32, 198, 108, 22, 79, 192, 36, 18,
                136, 124, 181, 99, 169, 163, 180, 80, 254, 220, 173, 142, 18, 237, 54, 3, 16, 169,
                21, 103, 55, 115, 199, 117, 130, 100, 97, 24, 39, 153, 125, 7, 132, 139, 67, 104,
                143, 156, 136, 167, 112, 144, 125, 173, 240, 8,
            ],
        };

        let tx: crate::state::transaction::Transaction = raw_tx.try_into().unwrap();

        println!("{:?}", tx)
    }
}
