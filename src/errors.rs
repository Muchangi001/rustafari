// This module defines custom error types for the application.
// It includes error handling for user-related operations and connection issues.
// It also implements the `IntoResponse` trait for converting errors into HTTP responses.
// This allows the application to return appropriate HTTP status codes and messages
// when errors occur, making it easier to handle errors in a consistent way.
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