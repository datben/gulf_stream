use std::{num::ParseIntError, sync::TryLockError};

use pg_embed::pg_errors::PgEmbedError;
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum GulfStreamError {
    #[error("Default Error")]
    Default,
    #[error("Wrong Parent Blockhash")]
    WrongParentBlockhash,
    #[error("Link already filled")]
    LinkAlreadyFilled,
    #[error("Block did not meet requirement")]
    BlockIsNotValid,
    #[error("Did not expect this index")]
    WrongIndex,
    #[error("Unknown block")]
    BlockNotFound,
    #[error("Lock error")]
    TryLockError,
    #[error("Failed to find previous blockhash")]
    DidNotFindPreviousBlock,
    #[error("This transaction is not valid")]
    TxIsNotValid,
    #[error("Something went wrong : {0}")]
    Generic(String),
    #[error("SerDeError : {0}")]
    SerDeError(String),
}

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
        format!("{self}")
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
