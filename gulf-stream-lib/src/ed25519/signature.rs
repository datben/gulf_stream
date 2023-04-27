use crate::{
    err::{GulfStreamError, Result},
    state::transaction::{Transaction, TransactionMessage},
    utils::serde::{BytesDeserialize, BytesSerialize},
};
use ed25519_dalek::{Digest, Keypair, Sha512};
use hex_literal::hex;

#[derive(Debug, Clone, PartialEq)]
pub struct Signature(pub ed25519_dalek::Signature);

impl Signature {
    pub fn sign_payload(
        signer: &Keypair,
        blockheight: u64,
        gas: u64,
        msg: TransactionMessage,
    ) -> Transaction {
        let mut prehashed: Sha512 = Sha512::new();
        prehashed.update(blockheight.serialize());
        prehashed.update(gas.serialize());
        prehashed.update(msg.serialize());
        let signature = signer.sign_prehashed(prehashed, None).unwrap();
        Transaction {
            blockheight,
            payer: signer.public.into(),
            msg,
            signature: signature.into(),
            gas,
        }
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self(
            ed25519_dalek::Signature::from_bytes(&hex!("2A80234CCDFA3FC3C8DC7B24394DAB4CF00A63E7F646B49540256192A635FCEE3A7ED14898A1AAC09950BC4F1EAA1569EEA23C33537EA68DF0F41990FF384F08"))
            .unwrap(),
        )
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
