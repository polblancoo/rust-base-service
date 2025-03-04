use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Authentication error: {0}")]
  Auth(String),
  #[error("Failed to generate token: {0}")]
    TokenGenerationError(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("Token expired")]
    TokenExpired,
  #[error("Database error: {0}")]
  Database(String),
  
  #[error("Validation error: {0}")]
  Validation(String),
  
  #[error("Not found: {0}")]
  NotFound(String),
  
  #[error("Internal server error: {0}")]
  Internal(String),
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
      let (status, error_message) = match self {
          AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg),
          AppError::TokenGenerationError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
          AppError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
          AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
          AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
          AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
          AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
          AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
      };

      let body = Json(json!({
          "error": error_message,
      }));

      (status, body).into_response()
  }
}
