use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;
mod display;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, SkadiError>;

/// A boxed error kind, wrapping an [SkadiErrorKind].
#[derive(Clone)]
pub struct SkadiError {
    kind: Box<SkadiErrorKind>,
}

/// The kind of [SkadiError].
#[derive(Debug, Copy, Clone)]
pub enum SkadiErrorKind {
    /// An unknown error.
    UnknownError,
}
