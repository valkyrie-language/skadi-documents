use super::*;

impl From<SkadiErrorKind> for SkadiError {
    fn from(error: SkadiErrorKind) -> Self {
        Self { kind: Box::new(error) }
    }
}

impl From<std::io::Error> for SkadiError {
    fn from(error: std::io::Error) -> Self {
        SkadiErrorKind::UnknownError.into()
    }
}
impl From<toml::de::Error> for SkadiError {
    fn from(error: toml::de::Error) -> Self {
        SkadiErrorKind::UnknownError.into()
    }
}

impl From<sqlx::Error> for SkadiError {
    fn from(error: sqlx::Error) -> Self {
        SkadiErrorKind::UnknownError.into()
    }
}