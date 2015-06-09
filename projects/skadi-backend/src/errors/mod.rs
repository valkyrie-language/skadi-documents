use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};
use serde::{Deserialize, Serialize};

mod convert;
mod display;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, SkadiError>;

/// A boxed error kind, wrapping an [SkadiErrorKind].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkadiError {
    EncodeError {
        format: String,
        message: String,
    },
    DecodeError {
        format: String,
        message: String,
    },
    IoError {
        message: String,
        path: String,
    },
    DatabaseError {
        message: String,
    },
    /// An unknown error.
    UnknownError,
}
