use axum::{
  body::Body,
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use common::error::AppError;
use crate::AppState;
use std::sync::Arc;

//use shared::AppState;
use common::jwt::verify_jwt;

pub async fn auth_middleware(
  State(state): State<Arc<AppState>>,
  request: Request<Body>,
  next: Next,
) -> Result<Response, AppError> {
  let authorization = request
      .headers()
      .get("Authorization")
      .ok_or_else(|| AppError::Auth("Missing authorization header".into()))?;
  
  let auth_header = authorization
      .to_str()
      .map_err(|_| AppError::Auth("Invalid authorization header".into()))?;
  
  if !auth_header.starts_with("Bearer ") {
      return Err(AppError::Auth("Invalid token format".into()));
  }
  
  let token = auth_header.trim_start_matches("Bearer ").trim();
  
  let _claims = verify_jwt(token, &state.jwt_secret)
      .map_err(|e| AppError::Auth(e.to_string()))?;
  
  // Continuar con la siguiente middleware/handler con el token validado
  Ok(next.run(request).await)
}
