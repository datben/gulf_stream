#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Default,
    WrongParentBlockhash,
    LinkAlreadyFilled,
    NoMoreOlderBlocks,
    MissingIntermediateBlocks,
    BlockIsNotValid,
}
pub type Result<T> = std::result::Result<T, Error>;

impl Default for Error {
    fn default() -> Self {
        Error::Default
    }
}
