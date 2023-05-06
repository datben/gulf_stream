use crate::{
    err::{GulfStreamError, Result},
    state::transaction::{Transaction, TransactionMessage},
    utils::serde::{BytesDeserialize, BytesSerialize},
};
use ed25519_dalek::{Keypair, Signer};

#[derive(Debug, Clone, PartialEq)]
pub struct Signature(pub ed25519_dalek::Signature);

impl Signature {
    pub fn into_string(&self) -> String {
        Into::<String>::into(self)
    }

    pub fn try_from_str(s: &str) -> Result<Self> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| GulfStreamError::SerDeError("Signature".into()))?;
        Self::deserialize(&mut &bytes[..])
    }

    pub fn sign_payload(
        signer: &Keypair,
        blockheight: u64,
        gas: u64,
        msg: TransactionMessage,
    ) -> Transaction {
        let mut message = vec![];
        message.extend(blockheight.serialize());
        message.extend(gas.serialize());
        message.extend(msg.serialize());
        let encoded = bs58::encode(message).into_vec();
        let signature = signer.try_sign(encoded.as_slice()).unwrap();
        Transaction {
            blockheight,
            payer: signer.public.into(),
            msg,
            signature: signature.into(),
            gas,
        }
    }
}

impl Into<String> for &Signature {
    fn into(self) -> String {
        bs58::encode(self.0.to_bytes()).into_string()
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self::try_from_str("4K9HxzmBv5ALfq9nMZS7jewM5XbBqFEgkKxpW3j7gExPmmACAaFY7kdFfUvt1W7oPkPHPtGgWWH6XjT1g17cT2wZ")
            .unwrap()
    }
}

impl BytesSerialize for Signature {
    fn serialize(&self) -> Vec<u8> {
        self.0.to_bytes().to_vec()
    }
}

impl BytesDeserialize for Signature {
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        let data = &buf[..ed25519_dalek::SIGNATURE_LENGTH];
        *buf = &buf[ed25519_dalek::SIGNATURE_LENGTH..];
        Ok(Self(ed25519_dalek::Signature::from_bytes(data).map_err(
            |_| GulfStreamError::SerDeError("Signature".into()),
        )?))
    }
}

impl Into<ed25519_dalek::Signature> for Signature {
    fn into(self) -> ed25519_dalek::Signature {
        self.0
    }
}

impl From<ed25519_dalek::Signature> for Signature {
    fn from(value: ed25519_dalek::Signature) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn se_de_signature() {
        let signature = Signature::default();
        let se = signature.serialize();
        let de: Signature = Signature::deserialize(&mut se.as_slice()).unwrap();
        assert_eq!(de, signature);
    }
}
