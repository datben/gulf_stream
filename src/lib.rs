pub mod err;
pub mod state;

type Result<T> = std::result::Result<T, err::Error>;
