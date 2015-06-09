use super::*;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

impl Error for SkadiError {}

impl Display for SkadiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SkadiError::EncodeError { format, message } => {
                write!(f, "EncodeError({}): {}", format, message)
            }
            SkadiError::DecodeError { format, message } => {
                write!(f, "DecodeError({}): {}", format, message)
            }
            SkadiError::IoError { message, path } => {
                write!(f, "IoError: {} at {}", message, path)
            }
            SkadiError::DatabaseError { message } => {
                write!(f, "DatabaseError: {}", message)
            }
            SkadiError::UnknownError => {
                write!(f, "UnknownError")
            }
        }
    }
}

impl IntoResponse for SkadiError {
    fn into_response(self) -> Response {
        let json = serde_json::to_string(&self).unwrap_or_default();
        Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from(json)).unwrap()
    }
}
