use anyhow::Result;
use database::repository::PgUserRepository;
use shared::User;
use shared::{AppState, AuthService as AuthServiceTrait};
use shared::user::{CreateUserSchema, FilteredUser};
use common::jwt::generate_jwt;
use common::password::hash_password;
use uuid::Uuid;

pub struct AuthService {
    user_repo: PgUserRepository,
    jwt_secret: String,
    jwt_expires_in: String,
}

impl AuthService {
    pub fn new(user_repo: PgUserRepository, jwt_secret: String, jwt_expires_in: String) -> Self {
        Self { 
            user_repo,
            jwt_secret,
            jwt_expires_in,
        }
    }
}

impl AuthServiceTrait for AuthService {
    fn authenticate_by_email(&self, email: &str, password: &str) -> Result<User> {
        // Implementación síncrona que llama al método asíncrono
        // En un entorno real, necesitarías manejar esto de manera diferente
        // Esta es una simplificación para este ejemplo
        Err(anyhow::anyhow!("Not implemented in synchronous context"))
    }

    fn authenticate_by_telegram(&self, telegram_id: &str) -> Result<User> {
        // Implementación síncrona que llama al método asíncrono
        // En un entorno real, necesitarías manejar esto de manera diferente
        // Esta es una simplificación para este ejemplo
        Err(anyhow::anyhow!("Not implemented in synchronous context"))
    }

    fn generate_token(&self, user: &User) -> Result<String> {
        // Generar token JWT
        let token = generate_jwt(user, &self.jwt_secret, &self.jwt_expires_in)?;
        Ok(token)
    }

    fn get_user(&self, user_id: &Uuid) -> Result<FilteredUser> {
        // Esta es una implementación síncrona que no puede llamar a métodos asíncronos
        Err(anyhow::anyhow!("Not implemented in synchronous context"))
    }

    fn register_user(&self, user_data: &CreateUserSchema, telegram_user_id: Option<String>) -> Result<FilteredUser> {
        // Esta es una implementación síncrona que no puede llamar a métodos asíncronos
        Err(anyhow::anyhow!("Not implemented in synchronous context"))
    }
}

// Métodos asíncronos para usar en los handlers
impl AuthService {
    pub async fn authenticate_by_email_async(&self, email: &str, password: &str) -> Result<User> {
        // Buscar usuario por email
        let user = self.user_repo.find_user_by_email(email).await?;

        // Verificar contraseña
        if !self.verify_password(&user.password, password)? {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        Ok(user)
    }

    pub async fn authenticate_by_telegram_async(&self, telegram_user_id: &str) -> Result<User> {
        // Buscar usuario por Telegram user ID
        let user = self.user_repo.find_by_telegram_user_id(telegram_user_id).await?;
        Ok(user)
    }

    pub async fn register_user_async(&self, user_data: &CreateUserSchema, telegram_user_id: Option<String>) -> Result<FilteredUser> {
        // Hash de la contraseña
        let hashed_password = hash_password(&user_data.password)?;
        
        // Crear usuario
        let user = self.user_repo.create_user(user_data, &hashed_password, telegram_user_id).await?;
        
        // Filtrar información sensible
        Ok(filter_user_response(user))
    }

    pub async fn get_user_async(&self, user_id: &Uuid) -> Result<FilteredUser> {
        // Buscar usuario por ID
        let user = self.user_repo.find_by_id(user_id).await?;
        
        // Filtrar información sensible
        Ok(filter_user_response(user))
    }

    fn verify_password(&self, stored_password: &str, provided_password: &str) -> Result<bool> {
        // Implementa la verificación de contraseña (por ejemplo, usando bcrypt)
        Ok(stored_password == provided_password) // Cambia esto por una implementación segura
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