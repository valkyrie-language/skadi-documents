use super::*;

impl From<std::io::Error> for SkadiError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        let location = std::panic::Location::caller();
        SkadiError::IoError { message: error.to_string(), path: location.to_string() }
    }
}
impl From<toml::de::Error> for SkadiError {
    fn from(error: toml::de::Error) -> Self {
        SkadiError::DecodeError { format: "toml".to_string(), message: error.to_string() }
    }
}

impl From<sqlx::Error> for SkadiError {
    fn from(error: sqlx::Error) -> Self {
        SkadiError::DatabaseError { message: error.to_string() }
    }
}
