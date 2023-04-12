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
                .collect::<Result<Vec<crate::state::block::TransactionState>, Self::Error>>()?,
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
            blockheight: self.blockheight,
            payer: PublicKey::deserialize(&mut self.payer.as_ref())?,
            msg: TransactionMessage::deserialize(&mut self.msg.as_ref())?,
            signature: Signature::deserialize(&mut self.signature.as_ref())?,
            gas: self.gas,
        })
    }
}

impl From<crate::state::transaction::Transaction> for Transaction {
    fn from(value: crate::state::transaction::Transaction) -> Self {
        Self {
            blockheight: value.blockheight,
            payer: value.payer.serialize(),
            msg: value.msg.serialize(),
            signature: value.signature.serialize(),
            gas: value.gas,
        }
    }
}

impl TryInto<crate::state::block::TransactionState> for TransactionState {
    type Error = crate::err::GulfStreamError;

    fn try_into(self) -> Result<crate::state::block::TransactionState, Self::Error> {
        match self.state {
            0 => self
                .tx
                .map(|tx| {
                    tx.try_into()
                        .map(|tx| crate::state::block::TransactionState::Success(tx))
                        .ok()
                })
                .ok_or(Self::Error::default())?
                .ok_or(Self::Error::default()),
            1 => self
                .tx
                .map(|tx| {
                    tx.try_into()
                        .map(|tx| crate::state::block::TransactionState::Fail(tx))
                        .ok()
                })
                .ok_or(Self::Error::default())?
                .ok_or(Self::Error::default()),
            2 => self
                .tx
                .map(|tx| {
                    tx.try_into()
                        .map(|tx| crate::state::block::TransactionState::Pending(tx))
                        .ok()
                })
                .ok_or(Self::Error::default())?
                .ok_or(Self::Error::default()),
            _ => Err(Self::Error::default()),
        }
    }
}

impl From<crate::state::block::TransactionState> for TransactionState {
    fn from(value: crate::state::block::TransactionState) -> Self {
        match value {
            crate::state::block::TransactionState::Success(tx) => TransactionState {
                state: 0,
                tx: Some(tx.into()),
            },
            crate::state::block::TransactionState::Fail(tx) => TransactionState {
                state: 1,
                tx: Some(tx.into()),
            },
            crate::state::block::TransactionState::Pending(tx) => TransactionState {
                state: 2,
                tx: Some(tx.into()),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use crate::err;

    use super::*;

    #[test]
    fn pb_into_tx() {
        let raw_tx = Transaction {
            blockheight: 5,
            payer: vec![
                110, 244, 56, 156, 170, 232, 45, 208, 70, 45, 1, 194, 190, 0, 250, 95, 236, 230,
                83, 70, 255, 253, 51, 219, 174, 30, 197, 82, 243, 235, 57, 228,
            ],
            msg: TransactionMessage::Transfer {
                to: PublicKey::random(),
                amount: 77,
            }
            .serialize(),
            signature: vec![
                158, 110, 193, 161, 204, 34, 23, 164, 123, 203, 32, 198, 108, 22, 79, 192, 36, 18,
                136, 124, 181, 99, 169, 163, 180, 80, 254, 220, 173, 142, 18, 237, 54, 3, 16, 169,
                21, 103, 55, 115, 199, 117, 130, 100, 97, 24, 39, 153, 125, 7, 132, 139, 67, 104,
                143, 156, 136, 167, 112, 144, 125, 173, 240, 8,
            ],
            gas: 65,
        };

        let tx: Result<crate::state::transaction::Transaction, err::GulfStreamError> =
            raw_tx.try_into();

        assert!(tx.is_ok())
    }
}
