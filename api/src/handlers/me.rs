use axum::{
  extract::{Json, State},
  http::HeaderMap,
};
use common::error::AppError;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use common::jwt::verify_jwt;
use crate::AppState;

pub async fn me_handler(
  State(state): State<Arc<AppState>>,
  headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
  let authorization = headers
      .get("Authorization")
      .ok_or_else(|| AppError::Auth("Missing authorization header".into()))?;
  
  let auth_header = authorization
      .to_str()
      .map_err(|_| AppError::Auth("Invalid authorization header".into()))?;
  
  if !auth_header.starts_with("Bearer ") {
      return Err(AppError::Auth("Invalid token format".into()));
  }
  
  let token = auth_header.trim_start_matches("Bearer ").trim();
  
  let claims = verify_jwt(token, &state.jwt_secret)
      .map_err(|e| AppError::Auth(e.to_string()))?;
  
  let user_id = Uuid::parse_str(&claims.sub)
      .map_err(|_| AppError::Auth("Invalid user ID in token".into()))?;
  
  let user = state
      .auth_service
      .get_user(&user_id)
      .await
      .map_err(|e| AppError::Auth(e.to_string()))?;
  
  Ok(Json(json!({
      "status": "success",
      "user": user
  })))
}
