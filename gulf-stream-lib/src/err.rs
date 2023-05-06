use std::{error::Error, fmt::Display, num::ParseIntError, sync::TryLockError};

use pg_embed::pg_errors::PgEmbedError;
use tonic::Status;

#[derive(Debug, Clone, PartialEq)]
pub enum GulfStreamError {
    Default,
    WrongParentBlockhash,
    LinkAlreadyFilled,
    BlockIsNotValid,
    WrongIndex,
    BlockNotFound,
    TryLockError,
    DidNotFindPreviousBlock,
    FailDeserialisationOfTransaction,
    TxIsNotValid,
    Generic(String),
    SerDeError(String),
}
pub type Result<T> = std::result::Result<T, GulfStreamError>;

impl Display for GulfStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for GulfStreamError {}

impl Default for GulfStreamError {
    fn default() -> Self {
        GulfStreamError::Default
    }
}

impl<T> From<TryLockError<T>> for GulfStreamError {
    fn from(_value: TryLockError<T>) -> Self {
        Self::TryLockError
    }
}

impl From<sqlx::Error> for GulfStreamError {
    fn from(value: sqlx::Error) -> Self {
        GulfStreamError::Generic(value.to_string())
    }
}

impl From<ParseIntError> for GulfStreamError {
    fn from(value: ParseIntError) -> Self {
        GulfStreamError::Generic(value.to_string())
    }
}

impl From<PgEmbedError> for GulfStreamError {
    fn from(value: PgEmbedError) -> Self {
        GulfStreamError::Generic(value.to_string())
    }
}

impl Into<String> for GulfStreamError {
    fn into(self) -> String {
        match self {
            GulfStreamError::Default => format!("GulfStreamError::Default"),
            GulfStreamError::WrongParentBlockhash => {
                format!("GulfStreamError::WrongParentBlockhash")
            }
            GulfStreamError::LinkAlreadyFilled => format!("GulfStreamError::LinkAlreadyFilled"),
            GulfStreamError::BlockIsNotValid => format!("GulfStreamError::BlockIsNotValid"),
            GulfStreamError::WrongIndex => format!("GulfStreamError::WrongIndex"),
            GulfStreamError::BlockNotFound => format!("GulfStreamError::BlockNotFound"),
            GulfStreamError::TryLockError => format!("GulfStreamError::TryLockError"),
            GulfStreamError::DidNotFindPreviousBlock => {
                format!("GulfStreamError::DidNotFindPreviousBlock")
            }
            GulfStreamError::FailDeserialisationOfTransaction => {
                format!("GulfStreamError::FailDeserialisationOfTransaction")
            }
            GulfStreamError::TxIsNotValid => {
                format!("GulfStreamError::TxIsNotValid")
            }
            GulfStreamError::SerDeError(s) => {
                format!("Failed to De/Serialize {}", s)
            }
            GulfStreamError::Generic(s) => {
                format!("Error : {}", s)
            }
        }
    }
}

impl Into<Status> for GulfStreamError {
    fn into(self) -> Status {
        Status::aborted(self)
    }
}

impl GulfStreamError {
    pub fn map<T: Into<String>>(value: T) -> Self {
        GulfStreamError::Generic(Into::into(value))
    }

    pub fn map_to_status<T: Into<String>>(value: T) -> Status {
        GulfStreamError::map(value).into()
    }
}
