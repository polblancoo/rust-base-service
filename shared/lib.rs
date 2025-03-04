use std::sync::Arc;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub mod user;

// Definimos nuestra propia versión de User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    // Otros campos necesarios
}

// Definimos nuestra propia versión de Config
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    // Otros campos necesarios
}

pub struct AppState {
    pub auth_service: Arc<dyn AuthService>,
    pub config: Arc<Config>,
}

pub trait AuthService: Send + Sync {
    fn authenticate_by_email(&self, email: &str, password: &str) -> Result<User>;
    fn authenticate_by_telegram(&self, telegram_id: &str) -> Result<User>;
    fn generate_token(&self, user: &User) -> Result<String>;
    fn get_user(&self, user_id: &Uuid) -> Result<user::FilteredUser>;
    fn register_user(&self, user_data: &user::CreateUserSchema, telegram_user_id: Option<String>) -> Result<user::FilteredUser>;
}