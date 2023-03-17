#[derive(Debug, Clone)]
pub enum Error {
    Default,
    CannotConvertIntoString,
}

impl Default for Error {
    fn default() -> Self {
        Error::Default
    }
}

impl From<bincode::Error> for Error {
    fn from(_value: bincode::Error) -> Self {
        Self::Default
    }
}
