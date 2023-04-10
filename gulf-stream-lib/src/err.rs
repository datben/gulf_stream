use std::sync::TryLockError;

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
    FailedToDeserialized,
}
pub type Result<T> = std::result::Result<T, GulfStreamError>;

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
            GulfStreamError::FailedToDeserialized => {
                format!("GulfStreamError::FailedToDeserialized")
            }
        }
    }
}

impl Into<Status> for GulfStreamError {
    fn into(self) -> Status {
        Status::aborted(self)
    }
}
