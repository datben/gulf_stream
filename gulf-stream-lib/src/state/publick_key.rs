use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct PublicKey(pub ed25519_dalek::PublicKey);

pub struct PublicKeyVisitor;

impl<'de> Visitor<'de> for PublicKeyVisitor {
    type Value = PublicKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a PublicKey")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(PublicKey(
            ed25519_dalek::PublicKey::from_bytes(value)
                .map_err(|_| serde::de::Error::custom("Publickey deserialization failed"))?,
        ))
    }
}

impl<'de> Deserialize<'de> for PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PublicKeyVisitor)
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PublicKey", 1)?;
        state.serialize_field("0", self.0.as_bytes())?;
        state.end()
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
