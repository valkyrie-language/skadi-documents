use super::*;

impl From<SkadiErrorKind> for SkadiError {
    fn from(value: SkadiErrorKind) -> Self {
        Self {
            kind: Box::new(value),
        }
    }
}