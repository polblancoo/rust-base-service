use anyhow::Result;
use common::jwt::generate_jwt;
use shared::user::{CreateUserSchema, FilteredUser, LoginUserSchema, User};
use repository::UserRepository;
use uuid::Uuid;
use serde::Serialize;
use async_trait::async_trait;
use tracing::{info, error, debug, warn};

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
        info!("Inicializando servicio de autenticación");
        Self {
            user_repository,
            jwt_secret,
            jwt_expires_in,
        }
    }

    pub async fn authenticate_by_email(&self, email: &str, password: &str) -> Result<User, AuthError> {
        info!("Intentando autenticar usuario con email: {}", email);
        
        let user = match self.user_repository.find_user_by_email(email).await {
            Ok(user) => {
                info!("Usuario encontrado en la base de datos: {}", email);
                user
            },
            Err(e) => {
                error!("Error al buscar usuario por email: {}, error: {}", email, e);
                return Err(AuthError::InvalidCredentials);
            }
        };

        debug!("Verificando contraseña para usuario: {}", email);
        let is_valid = match self.verify_password(&user.password, password) {
            Ok(valid) => {
                if valid {
                    info!("Contraseña válida para usuario: {}", email);
                } else {
                    warn!("Contraseña inválida para usuario: {}", email);
                }
                valid
            },
            Err(e) => {
                error!("Error al verificar contraseña: {}", e);
                return Err(AuthError::PasswordVerifyError(e.to_string()));
            }
        };
        
        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        info!("Autenticación exitosa para usuario: {}", email);
        Ok(user)
    }

    fn verify_password(&self, stored_password: &str, provided_password: &str) -> Result<bool> {
        debug!("Verificando contraseña almacenada: {}", stored_password);
        let is_valid = verify_password(provided_password, stored_password)?;
        Ok(is_valid)
    }

    pub async fn register_user(&self, user_data: &CreateUserSchema, telegram_user_id: Option<String>) -> Result<FilteredUser> {
        info!("Registrando nuevo usuario con email: {}", user_data.email);
        
        let hashed_password = match hash_password(&user_data.password) {
            Ok(hash) => {
                debug!("Contraseña hasheada correctamente");
                hash
            },
            Err(e) => {
                error!("Error al hashear contraseña: {}", e);
                return Err(anyhow::anyhow!("Error al hashear contraseña: {}", e));
            }
        };

        let user = match self.user_repository.create_user(user_data, &hashed_password, telegram_user_id).await {
            Ok(user) => {
                info!("Usuario creado correctamente: {}", user.email);
                user
            },
            Err(e) => {
                error!("Error al crear usuario: {}", e);
                return Err(anyhow::anyhow!("Error al crear usuario: {}", e));
            }
        };
        
        Ok(filter_user_response(user))
    }

    pub async fn login_user(&self, credentials: &LoginUserSchema) -> Result<(FilteredUser, String)> {
        info!("Iniciando proceso de login");
        
        let user = if let Some(email) = &credentials.email {
            info!("Autenticando por email: {}", email);
            self.authenticate_by_email(email, &credentials.password).await?
        } else {
            if let Some(telegram_id) = &credentials.telegram_user_id {
                warn!("Intento de autenticación por Telegram (no implementada): {}", telegram_id);
                return Err(anyhow::anyhow!("Autenticación por Telegram no implementada"));
            } else {
                error!("No se proporcionó email ni telegram_user_id");
                return Err(anyhow::anyhow!("Se requiere email o telegram_user_id"));
            }
        };

        info!("Generando token JWT para usuario: {}", user.email);
        let token = match generate_jwt(&user.id.to_string(), &self.jwt_secret, &self.jwt_expires_in) {
            Ok(token) => {
                debug!("Token JWT generado correctamente");
                token
            },
            Err(e) => {
                error!("Error al generar token JWT: {}", e);
                return Err(anyhow::anyhow!("Error al generar token JWT: {}", e));
            }
        };
        
        info!("Login exitoso para usuario: {}", user.email);
        Ok((filter_user_response(user), token))
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<FilteredUser> {
        info!("Buscando usuario por ID: {}", user_id);
        
        let user = match self.user_repository.find_user_by_id(user_id).await {
            Ok(user) => {
                info!("Usuario encontrado: {}", user.email);
                user
            },
            Err(e) => {
                error!("Error al buscar usuario por ID: {}, error: {}", user_id, e);
                return Err(anyhow::anyhow!("Error al buscar usuario: {}", e));
            }
        };
        
        Ok(filter_user_response(user))
    }
}

// Implementación del trait api::handlers::auth::AuthService para AuthService<T>
#[async_trait]
impl<T: UserRepository + Send + Sync + 'static> api::handlers::auth::AuthService for AuthService<T> {
    async fn register_user(&self, user_data: &shared::user::CreateUserSchema, telegram_id: Option<String>) -> Result<shared::user::FilteredUser, String> {
        info!("Delegando registro de usuario a la implementación interna");
        
        // Convertir de shared::user::CreateUserSchema a models::CreateUserSchema
        let models_user_data = CreateUserSchema {
            email: user_data.email.clone(),
            name: user_data.name.clone(),
            password: user_data.password.clone(),
            role: user_data.role.clone(),
        };

        // Ejecutamos el future y manejamos el resultado
        let filtered_user = match self.register_user(&models_user_data, telegram_id).await {
            Ok(user) => {
                info!("Usuario registrado correctamente: {}", user.email);
                user
            },
            Err(e) => {
                error!("Error en registro de usuario: {}", e);
                return Err(e.to_string());
            }
        };
        
        // Convertimos el resultado a shared::user::FilteredUser
        Ok(shared::user::FilteredUser {
            id: filtered_user.id,
            email: filtered_user.email,
            name: filtered_user.name,
            role: filtered_user.role,
            created_at: filtered_user.created_at,
            updated_at: filtered_user.updated_at,
        })
    }

    async fn authenticate_by_email(&self, email: &str, password: &str) -> Result<shared::user::User, String> {
        info!("Delegando autenticación por email a la implementación interna: {}", email);
        
        // Ejecutamos el future y manejamos el resultado
        let user = match self.authenticate_by_email(email, password).await {
            Ok(user) => {
                info!("Autenticación exitosa para: {}", email);
                user
            },
            Err(e) => {
                error!("Error en autenticación por email: {}, error: {}", email, e);
                return Err(e.to_string());
            }
        };
        
        // Convertimos el modelo::User a shared::user::User
        Ok(shared::user::User {
            id: user.id,
            email: user.email,
            password: user.password,
            name: user.name,
            role: user.role,
            telegram_user_id: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    async fn authenticate_by_telegram(&self, telegram_id: &str) -> Result<shared::user::User, String> {
        warn!("Intento de autenticación por Telegram (no implementada): {}", telegram_id);
        Err("Autenticación por Telegram no implementada".to_string())
    }

    async fn generate_token(&self, user: &shared::user::User) -> Result<String, String> {
        info!("Generando token JWT para usuario: {}", user.email);
        
        match generate_jwt(&user.id.to_string(), &self.jwt_secret, &self.jwt_expires_in) {
            Ok(token) => {
                debug!("Token JWT generado correctamente");
                Ok(token)
            },
            Err(e) => {
                error!("Error al generar token JWT: {}", e);
                Err(e.to_string())
            }
        }
    }

    async fn get_user(&self, user_id: &Uuid) -> Result<shared::user::FilteredUser, String> {
        info!("Delegando búsqueda de usuario por ID a la implementación interna: {}", user_id);
        
        // Ejecutamos el future y manejamos el resultado
        let filtered_user = match self.get_user(user_id).await {
            Ok(user) => {
                info!("Usuario encontrado por ID: {}", user_id);
                user
            },
            Err(e) => {
                error!("Error al buscar usuario por ID: {}, error: {}", user_id, e);
                return Err(e.to_string());
            }
        };
        
        // Convertimos el resultado a shared::user::FilteredUser
        Ok(shared::user::FilteredUser {
            id: filtered_user.id,
            email: filtered_user.email,
            name: filtered_user.name,
            role: filtered_user.role,
            created_at: filtered_user.created_at,
            updated_at: filtered_user.updated_at,
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