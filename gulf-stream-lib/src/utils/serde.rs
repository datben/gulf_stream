use crate::err::Result;

pub trait BytesDeserialize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub trait BytesSerialize {
    fn serialize(&self) -> Vec<u8>;
}

impl BytesDeserialize for u64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let data = &buf[0..8];
        *buf = &buf[8..];
        return Ok(u64::from_le_bytes(data.try_into().unwrap()));
    }
}

impl BytesSerialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}
