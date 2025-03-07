use anyhow::Result;
use jwt::generate_jwt;
use models::{CreateUserSchema, FilteredUser, LoginUserSchema, User};
use repository::UserRepository;
use uuid::Uuid;
use serde::Serialize;

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

    pub async fn authenticate_by_email(&self, email: &str, password: &str) -> Result<User, AuthError> {
        let user = self.user_repository
            .find_user_by_email(email)
            .await
            .map_err(|_| AuthError::InvalidCredentials)?;

        let is_valid = self.verify_password(&user.password, password)
            .map_err(|e| AuthError::PasswordVerifyError(e.to_string()))?;
        
        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        Ok(user)
    }

    fn verify_password(&self, stored_password: &str, provided_password: &str) -> Result<bool> {
        let is_valid = verify_password(stored_password, provided_password)?;
        Ok(is_valid)
    }

    pub async fn register_user(&self, user_data: &CreateUserSchema, telegram_user_id: Option<String>) -> Result<FilteredUser> {
        let hashed_password = hash_password(&user_data.password)?;

        let user = self.user_repository.create_user(user_data, &hashed_password, telegram_user_id).await?;
        
        Ok(filter_user_response(user))
    }

    pub async fn login_user(&self, credentials: &LoginUserSchema) -> Result<(FilteredUser, String)> {
        let user = if let Some(email) = &credentials.email {
            self.authenticate_by_email(email, &credentials.password).await?
        } else {
            if let Some(_telegram_id) = &credentials.telegram_user_id {
                return Err(anyhow::anyhow!("Autenticación por Telegram no implementada"));
            } else {
                return Err(anyhow::anyhow!("Se requiere email o telegram_user_id"));
            }
        };

        let token = generate_jwt(&user.id.to_string(), &self.jwt_secret, &self.jwt_expires_in)?;
        
        Ok((filter_user_response(user), token))
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<FilteredUser> {
        let user = self.user_repository
            .find_user_by_id(user_id)
            .await?;
        
        Ok(filter_user_response(user))
    }
}

impl<T: UserRepository + Send + Sync + 'static> api::handlers::auth::AuthService for AuthService<T> {
    fn register_user(&self, user_data: &shared::user::CreateUserSchema, telegram_id: Option<String>) -> Result<shared::user::FilteredUser, String> {
        let models_user_data = CreateUserSchema {
            email: user_data.email.clone(),
            name: user_data.name.clone().unwrap_or_default(),
            password: user_data.password.clone(),
        };

        let result = tokio::runtime::Handle::current().block_on(async {
            self.register_user(&models_user_data, telegram_id).await
        });
        
        result.map_err(|e| e.to_string()).map(|filtered_user| {
            shared::user::FilteredUser {
                id: filtered_user.id,
                email: filtered_user.email,
                name: Some(filtered_user.name),
                role: filtered_user.role,
                created_at: filtered_user.created_at,
                updated_at: filtered_user.updated_at,
            }
        })
    }

    fn authenticate_by_email(&self, email: &str, password: &str) -> Result<shared::user::User, String> {
        let result = tokio::runtime::Handle::current().block_on(async {
            self.authenticate_by_email(email, password).await
        });
        
        result.map_err(|e| e.to_string()).map(|user| {
            shared::user::User {
                id: user.id,
                email: user.email,
                password: user.password,
                name: Some(user.name),
                role: user.role,
                telegram_user_id: None,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }
        })
    }

    fn authenticate_by_telegram(&self, _telegram_id: &str) -> Result<shared::user::User, String> {
        Err("Autenticación por Telegram no implementada".to_string())
    }

    fn generate_token(&self, user: &shared::user::User) -> Result<String, String> {
        generate_jwt(&user.id.to_string(), &self.jwt_secret, &self.jwt_expires_in)
            .map_err(|e| e.to_string())
    }

    fn get_user(&self, user_id: &Uuid) -> Result<shared::user::FilteredUser, String> {
        let result = tokio::runtime::Handle::current().block_on(async {
            self.get_user(user_id).await
        });
        
        result.map_err(|e| e.to_string()).map(|filtered_user| {
            shared::user::FilteredUser {
                id: filtered_user.id,
                email: filtered_user.email,
                name: Some(filtered_user.name),
                role: filtered_user.role,
                created_at: filtered_user.created_at,
                updated_at: filtered_user.updated_at,
            }
        })
    }
}

fn filter_user_response(user: User) -> FilteredUser {
    FilteredUser {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
        created_at: user.created_at.unwrap_or_default(),
        updated_at: user.updated_at.unwrap_or_default(),
    }
}