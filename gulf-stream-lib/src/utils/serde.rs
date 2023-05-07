use crate::err::GulfStreamError;

pub trait BytesDeserialize {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, GulfStreamError>
    where
        Self: Sized;
}

pub trait BytesSerialize {
    fn serialize(&self) -> Vec<u8>;
}

impl BytesDeserialize for u64 {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, GulfStreamError>
    where
        Self: Sized,
    {
        let data = &buf[0..8];
        *buf = &buf[8..];
        return Ok(u64::from_le_bytes(
            data.try_into()
                .map_err(|_| GulfStreamError::SerDeError("u64".into()))?,
        ));
    }
}

impl BytesSerialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl BytesDeserialize for bool {
    fn deserialize(buf: &mut &[u8]) -> Result<Self, GulfStreamError>
    where
        Self: Sized,
    {
        let data = buf[0];
        *buf = &buf[1..];
        if data == 1 {
            Ok(true)
        } else if data == 0 {
            Ok(false)
        } else {
            Err(crate::err::GulfStreamError::SerDeError("bool".into()).into())
        }
    }
}

impl BytesSerialize for bool {
    fn serialize(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }
}
