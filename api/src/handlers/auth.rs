use axum::{
  extract::{Request, Json},
    http::StatusCode,
};
use std::sync::Arc;
use common::error::AppError;
use shared::user::{CreateUserSchema, FilteredUser, LoginUserSchema};
use crate::AppState;
use common::jwt::verify_jwt;

use serde_json::{json, Value};
use validator::Validate;
use axum::body::Body;
use axum::middleware::Next;
use axum::extract::{State,  Query};
use axum::response::Response;
use serde::Serialize;
use uuid::Uuid;

// Definimos un trait para AuthService
#[async_trait::async_trait]
pub trait AuthService: Send + Sync {
    async fn register_user(&self, user_data: &CreateUserSchema, telegram_id: Option<String>) -> Result<FilteredUser, String>;
    async fn authenticate_by_email(&self, email: &str, password: &str) -> Result<shared::user::User, String>;
    async fn authenticate_by_telegram(&self, telegram_id: &str) -> Result<shared::user::User, String>;
    async fn generate_token(&self, user: &shared::user::User) -> Result<String, String>;
    async fn get_user(&self, user_id: &Uuid) -> Result<FilteredUser, String>;
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
#[derive(serde::Deserialize, Debug)] // Añade Debug para facilitar el logging
pub struct RegisterParams {
    telegram_user_id: Option<String>,
}

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

// api/src/handlers/auth.rs
 
#[axum::debug_handler]
pub async fn register_handler(
 
  State(state): State<Arc<AppState>>,
    Query(params): Query<RegisterParams>,
  Json(payload): Json<CreateUserSchema>,
   
) -> Result<Json<Value>, AppError> {
  // Validar entrada
  payload.validate().map_err(|e| AppError::Validation(e.to_string()))?;

  let telegram_id = params.telegram_user_id;

  // Registrar usuario
  let user = state
      .auth_service
      .register_user(&payload, telegram_id)
      .await
      .map_err(|e| AppError::Auth(e.to_string()))?;
  
  Ok(Json(json!({
      "status": "success",
      "message": "User registered successfully",
      "user": user
  })))
}

pub async fn login_handler(
  State(app_state): State<Arc<AppState>>,
  Json(body): Json<LoginUserSchema>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
  let user = if let Some(email) = body.email {
      // Autenticación con correo electrónico
      match app_state.auth_service.authenticate_by_email(&email, &body.password).await {
          Ok(user) => user,
          Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid email or password"}))))
      }
  } else if let Some(telegram_user_id) = body.telegram_user_id {
      // Autenticación con Telegram
      match app_state.auth_service.authenticate_by_telegram(&telegram_user_id).await {
          Ok(user) => user,
          Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid Telegram user ID"}))))
      }
  } else {
      return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Email or Telegram user ID is required"}))));
  };

  // Generar token JWT
  let token = match app_state.auth_service.generate_token(&user).await {
      Ok(token) => token,
      Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to generate token"}))))
  };

  Ok(Json(LoginResponse { token }))
}
