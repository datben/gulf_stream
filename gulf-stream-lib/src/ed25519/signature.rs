use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Signature(pub ed25519_dalek::Signature);

impl Default for Signature {
    fn default() -> Self {
        Self(
            ed25519_dalek::Signature::from_bytes(&[
                228, 156, 243, 186, 38, 251, 212, 144, 41, 101, 11, 216, 197, 222, 175, 108, 195,
                0, 14, 254, 241, 84, 60, 110, 10, 39, 109, 174, 150, 49, 117, 5, 90, 91, 14, 117,
                113, 67, 241, 82, 9, 59, 153, 3, 232, 63, 239, 194, 189, 196, 34, 175, 50, 82, 246,
                91, 78, 192, 60, 209, 115, 28, 159, 137,
            ])
            .unwrap(),
        )
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_tuple_struct(
            "Signature",
            ed25519_dalek::Signature::BYTE_SIZE,
            SignatureVisitor,
        )
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Signature", 1)?;
        state.serialize_field("0", &self.0.to_bytes().to_vec())?;
        state.end()
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

struct SignatureVisitor;

impl<'de> Visitor<'de> for SignatureVisitor {
    type Value = Signature;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Signature")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Signature(
            ed25519_dalek::Signature::from_bytes(value)
                .map_err(|_| serde::de::Error::custom("Signature deserialization failed"))?,
        ))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut collector = vec![];
        let mut next: Option<u8> = seq.next_element()?;
        while next.is_some() {
            collector.push(next.unwrap());
            next = seq.next_element()?;
        }
        self.visit_bytes(collector.as_slice())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn se_de_publickey() {
        let signature = Signature::default();
        let se = bincode::serialize(&signature).unwrap();
        let de: Signature = bincode::deserialize(se.as_slice()).unwrap();
        assert_eq!(de, signature);
    }
}
