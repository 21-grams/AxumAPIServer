use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Template error: {0}")]
    Template(String),
    
    #[error("Language error: {0}")]
    Language(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::Template(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::Language(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::Io(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        tracing::error!("Error: {}", error_message);
        
        (status, error_message).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;