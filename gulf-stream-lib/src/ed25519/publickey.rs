use crate::{
    err::*,
    utils::serde::{BytesDeserialize, BytesSerialize},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PublicKey(pub ed25519_dalek::PublicKey);

impl PublicKey {
    #[cfg(test)]
    pub fn random() -> Self {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        return Self(keypair.public);
    }
}

impl BytesDeserialize for PublicKey {
    fn deserialize(buf: &mut &[u8]) -> Result<Self> {
        let data = &buf[..32];
        *buf = &buf[32..];
        Ok(Self(
            ed25519_dalek::PublicKey::from_bytes(data).map_err(|_| GulfStreamError::default())?,
        ))
    }
}

impl BytesSerialize for PublicKey {
    fn serialize(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl Into<ed25519_dalek::PublicKey> for PublicKey {
    fn into(self) -> ed25519_dalek::PublicKey {
        self.0
    }
}

impl From<ed25519_dalek::PublicKey> for PublicKey {
    fn from(value: ed25519_dalek::PublicKey) -> Self {
        Self(value)
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn se_de_publickey() {
        let pk = PublicKey::default();
        let se = pk.serialize();
        let de: PublicKey = PublicKey::deserialize(&mut se.as_slice()).unwrap();
        assert_eq!(de, pk);
    }
}
