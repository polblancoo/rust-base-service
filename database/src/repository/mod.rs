use anyhow::Result;
use async_trait::async_trait;
use shared::user::{CreateUserSchema, FilteredUser, User};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, NaiveDateTime, Utc};
//pub mod user_repository;

/// Trait for user repository
#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, user: &CreateUserSchema, hashed_password: &str,telegram_user_id: Option<String>) -> Result<User>;
    async fn find_user_by_email(&self, email: &str) -> Result<User>;
    async fn find_user_by_id(&self, id: &Uuid) -> Result<User>;
        async fn find_by_telegram_user_id(&self, telegram_user_id: &str) -> Result<User>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(&self, user: &CreateUserSchema, hashed_password: &str, telegram_user_id: Option<String>) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password, name, role, telegram_user_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password, telegram_user_id, name, role, created_at, updated_at
            "#,
            user.email,
            hashed_password,
            user.name,
            "user",
            telegram_user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    async fn find_by_telegram_user_id(&self, telegram_user_id: &str) -> Result<User> {
        // Consulta SQL para buscar usuario por Telegram user ID
        let user = sqlx::query_as!(
            User,
            r#"SELECT * FROM users WHERE telegram_user_id = $1"#,
            telegram_user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
    async fn find_user_by_email(&self, email: &str) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    async fn find_user_by_id(&self, id: &Uuid) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        let created_at = user.created_at.map(|naive| DateTime::from_naive_utc_and_offset(naive.naive_utc(), Utc));
        let updated_at = user.updated_at.map(|naive| DateTime::from_naive_utc_and_offset(naive.naive_utc(), Utc));
    

    
        Ok(User {
            id: user.id,
            email: user.email,
            password: user.password,
            telegram_user_id: user.telegram_user_id,
            name: user.name,
            role: user.role,
            created_at,
            updated_at,
        })
    }
}