use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};
use jwt::generate_jwt;
use bcrypt::{hash, verify, DEFAULT_COST};

pub mod user;
use user::{
    User, 
    CreateUserSchema, 
    LoginUserSchema, 
    FilteredUser
};

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Password verify error")]
    PasswordVerifyError(String),
    #[error("Validation error")]
    ValidationError(String),
}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(_err: jsonwebtoken::errors::Error) -> AuthError {
        AuthError::InternalServerError
    }
}

pub trait UserRepository: Send + Sync {
    fn find_user_by_id<'a>(&'a self, user_id: &Uuid) -> impl std::future::Future<Output = Result<User, AuthError>> + Send;
    fn find_user_by_email<'a>(&'a self, email: &str) -> impl std::future::Future<Output = Result<User, AuthError>> + Send;
    fn create_user<'a>(&'a self, user_data: &'a CreateUserSchema, hashed_password: &str, telegram_user_id: Option<String>) -> impl std::future::Future<Output = Result<User, AuthError>> + Send;
    fn find_by_telegram_user_id<'a>(&'a self, telegram_id: &str) -> impl std::future::Future<Output = Result<User, AuthError>> + Send;
}

pub struct AuthServiceImpl<T: UserRepository> {
    repository: T,
    jwt_secret: String,
    jwt_expires_in: String,
}

impl<T: UserRepository> AuthServiceImpl<T> {
    pub fn new(repository: T, jwt_secret: String, jwt_expires_in: String) -> Self {
        Self { repository, jwt_secret, jwt_expires_in }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

pub trait AuthService: Send + Sync {
    fn authenticate_by_email<'a>(&'a self, email: &'a str, password: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a>;
    fn authenticate_by_telegram<'a>(&'a self, telegram_id: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a>;
}

pub trait AuthServiceAsync: Send + Sync {
    fn generate_token<'a>(&'a self, user: &'a User) -> Box<dyn std::future::Future<Output = Result<String, AuthError>> + Send + 'a>;
    fn get_user<'a>(&'a self, user_id: &'a Uuid) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a>;
    fn register_user<'a>(&'a self, user_data: &'a CreateUserSchema, telegram_user_id: Option<String>) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a>;
}

impl<T: UserRepository + Send + Sync> AuthService for AuthServiceImpl<T> {
    fn authenticate_by_email<'a>(&'a self, email: &'a str, password: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user = self.repository.find_user_by_email(email).await?;
            if verify_password(password, &user.password)? {
                Ok(user)
            } else {
                Err(AuthError::InvalidCredentials)
            }
        })
    }

    fn authenticate_by_telegram<'a>(&'a self, telegram_id: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a> {
        Box::new(async move {
            self.repository.find_by_telegram_user_id(telegram_id).await
        })
    }
}

impl<T: UserRepository + Send + Sync> AuthServiceAsync for AuthServiceImpl<T> {
    fn generate_token<'a>(&'a self, user: &'a User) -> Box<dyn std::future::Future<Output = Result<String, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user_id_str = user.id.to_string();
            let token = generate_jwt(&user_id_str, &self.jwt_secret, &self.jwt_expires_in)?;
            Ok(token)
        })
    }

    fn get_user<'a>(&'a self, user_id: &'a Uuid) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user = self.repository.find_user_by_id(user_id).await?;
            Ok(user.to_filtered_user())
        })
    }

    fn register_user<'a>(&'a self, user_data: &'a CreateUserSchema, telegram_user_id: Option<String>) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a> {
        Box::new(async move {
            if user_data.email.is_empty() {
                return Err(AuthError::InvalidCredentials);
            }
            
            let hashed_password = hash_password(&user_data.password)?;
            let user = self.repository.create_user(user_data, &hashed_password, telegram_user_id).await?;
            Ok(user.to_filtered_user())
        })
    }
}

pub trait AuthServiceImplTrait: AuthService + AuthServiceAsync {
    fn authenticate_by_email<'a>(&'a self, email: &'a str, password: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a>;
    fn authenticate_by_telegram<'a>(&'a self, telegram_id: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a>;
    fn generate_token<'a>(&'a self, user: &'a User) -> Box<dyn std::future::Future<Output = Result<String, AuthError>> + Send + 'a>;
    fn get_user<'a>(&'a self, user_id: &'a Uuid) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a>;
    fn register_user<'a>(&'a self, user_data: &'a CreateUserSchema, telegram_user_id: Option<String>) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a>;
}

impl<T: UserRepository + Send + Sync> AuthServiceImplTrait for AuthServiceImpl<T> {
    fn authenticate_by_email<'a>(&'a self, email: &'a str, password: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user = self.repository.find_user_by_email(email).await?;
            if verify_password(password, &user.password)? {
                Ok(user)
            } else {
                Err(AuthError::InvalidCredentials)
            }
        })
    }

    fn authenticate_by_telegram<'a>(&'a self, telegram_id: &'a str) -> Box<dyn std::future::Future<Output = Result<User, AuthError>> + Send + 'a> {
        Box::new(async move {
            self.repository.find_by_telegram_user_id(telegram_id).await
        })
    }

    fn generate_token<'a>(&'a self, user: &'a User) -> Box<dyn std::future::Future<Output = Result<String, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user_id_str = user.id.to_string();
            let token = generate_jwt(&user_id_str, &self.jwt_secret, &self.jwt_expires_in)?;
            Ok(token)
        })
    }

    fn get_user<'a>(&'a self, user_id: &'a Uuid) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a> {
        Box::new(async move {
            let user = self.repository.find_user_by_id(user_id).await?;
            Ok(user.to_filtered_user())
        })
    }

    fn register_user<'a>(&'a self, user_data: &'a CreateUserSchema, telegram_user_id: Option<String>) -> Box<dyn std::future::Future<Output = Result<FilteredUser, AuthError>> + Send + 'a> {
        Box::new(async move {
            if user_data.email.is_empty() {
                return Err(AuthError::InvalidCredentials);
            }
            
            let hashed_password = hash_password(&user_data.password)?;
            let user = self.repository.create_user(user_data, &hashed_password, telegram_user_id).await?;
            Ok(user.to_filtered_user())
        })
    }
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    verify(password, hash).map_err(|e| AuthError::PasswordVerifyError(e.to_string()))
}

fn hash_password(password: &str) -> Result<String, AuthError> {
    hash(password, DEFAULT_COST).map_err(|e| AuthError::PasswordVerifyError(e.to_string()))
}