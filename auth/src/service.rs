use anyhow::Result;
use common::jwt::generate_jwt;
use shared::user::{CreateUserSchema, FilteredUser, LoginUserSchema, User};
use database::repository::UserRepository;
use uuid::Uuid;
use axum::extract::State;
use axum::Json;
use axum::http::StatusCode;

use sqlx::types::Json as SqlxJson;
use std::sync::Arc;
use serde::Serialize;
use serde_json::json;
use shared::AppState;

use crate::{
    error::AuthError,
    password::{hash_password, verify_password},
};

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct AuthService<T: UserRepository> {
    user_repository: T,
    jwt_secret: String,
    jwt_expires_in: String,
}

impl<T: UserRepository> AuthService<T> {
    pub fn new(user_repository: T, jwt_secret: String, jwt_expires_in: String) -> Self {
        Self {
            user_repository,
            jwt_secret,
            jwt_expires_in,
        }
    }

    pub async fn login_handler(
        State(app_state): State<Arc<AppState>>,
        Json(body): Json<LoginUserSchema>,
    ) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)> {
        let user = if let Some(email) = body.email {
            // Autenticación con correo electrónico
            match app_state.auth_service.authenticate_by_email(&email, &body.password) {
                Ok(user) => user,
                Err(err) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": err.to_string()}))))
            }
        } else if let Some(telegram_user_id) = body.telegram_user_id {
            // Autenticación con Telegram
            match app_state.auth_service.authenticate_by_telegram(&telegram_user_id) {
                Ok(user) => user,
                Err(err) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": err.to_string()}))))
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Email or Telegram user ID is required"}))));
        };

        // Generar token JWT
        let token = match app_state.auth_service.generate_token(&user) {
            Ok(token) => token,
            Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err.to_string()}))))
        };

        Ok(Json(LoginResponse { token }))
    }

    pub async fn authenticate_by_telegram(&self, telegram_user_id: &str) -> Result<User, AuthError> {
        // Buscar usuario por Telegram user ID
        let user = self.user_repository
            .find_by_telegram_user_id(telegram_user_id)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok(user)
    }

    pub async fn authenticate_by_email(&self, email: &str, password: &str) -> Result<User, AuthError> {
        // Buscar usuario por email
        let user = self.user_repository
            .find_user_by_email(email)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        // Verificar contraseña
        if !self.verify_password(&user.password, password)
            .map_err(|e| AuthError::PasswordVerifyError(e.to_string()))? 
        {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(user)
    }

    fn verify_password(&self, stored_password: &str, provided_password: &str) -> Result<bool> {
        // Implementa la verificación de contraseña (por ejemplo, usando bcrypt)
        Ok(stored_password == provided_password) // Cambia esto por una implementación segura
    }

    pub async fn register_user(&self, user_data: &CreateUserSchema, telegram_user_id: Option<String>) -> Result<FilteredUser> {
        let hashed_password = hash_password(&user_data.password)?;
        
        let user = self
            .user_repository
            .create_user(user_data, &hashed_password, telegram_user_id)
            .await?;
        
        Ok(filter_user_response(user))
    }

    pub async fn login_user(&self, credentials: &LoginUserSchema) -> Result<(FilteredUser, String)> {
        let user = self
            .user_repository
            .find_user_by_email(&credentials.email.as_ref().unwrap_or(&"".to_string()))
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;
        
        let is_valid = verify_password(&credentials.password, &user.password)?;
        
        if !is_valid {
            return Err(AuthError::InvalidCredentials.into());
        }
        
        let token = generate_jwt(&user.id, &self.jwt_secret, &self.jwt_expires_in)?;
        
        Ok((filter_user_response(user), token))
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<FilteredUser> {
        let user = self
            .user_repository
            .find_user_by_id(user_id)
            .await?;
        
        Ok(filter_user_response(user))
    }
}

// Función para filtrar información sensible del usuario
fn filter_user_response(user: User) -> FilteredUser {
    FilteredUser {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
        created_at: user.created_at.expect("created_at is missing"),
        updated_at: user.updated_at.expect("updated_at is missing"),
    }
}