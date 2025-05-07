use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    UserNotFound(String),
    UserAlreadyExists(String),
    ConnectionFailed(String),
    InternalError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AppError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::ConnectionFailed(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UserNotFound(username) => write!(f, "User not found: {}", username),
            AppError::UserAlreadyExists(username) => write!(f, "User already exists: {}", username),
            AppError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::UserAlreadyExists(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::ConnectionFailed(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        (status, message).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
