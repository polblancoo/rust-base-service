use anyhow::Result;
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use models::{User, CreateUserSchema};
use std::future::Future;

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password: String,
    name: String,
    role: String,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

pub trait UserRepository {
    fn find_user_by_id<'a>(&'a self, user_id: &'a Uuid) -> impl Future<Output = Result<User>> + Send + 'a; 
    fn find_user_by_email<'a>(&'a self, email: &'a str) -> impl Future<Output = Result<User>> + Send + 'a; 
    fn create_user<'a>(&'a self, user_data: &'a CreateUserSchema, hashed_password: &'a str, telegram_user_id: Option<String>) -> impl Future<Output = Result<User>> + Send + 'a; 
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn find_user_by_id<'a>(&'a self, user_id: &'a Uuid) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?;

            Ok(User {
                id: user.id,
                email: user.email,
                password: user.password,
                name: user.name,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
        }
    }

    fn find_user_by_email<'a>(&'a self, email: &'a str) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            let user = sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE email = $1")
                .bind(email)
                .fetch_one(&self.pool)
                .await?;

            Ok(User {
                id: user.id,
                email: user.email,
                password: user.password,
                name: user.name,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
        }
    }

    fn create_user<'a>(
        &'a self,
        user_data: &'a CreateUserSchema,
        hashed_password: &'a str,
        telegram_user_id: Option<String>,
    ) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            let user = sqlx::query_as::<_, UserRow>(
                "INSERT INTO users (email, password, name, role) VALUES ($1, $2, $3, $4) RETURNING *",
            )
                .bind(&user_data.email)
                .bind(hashed_password)
                .bind(&user_data.name)
                .bind("user")
                .fetch_one(&self.pool)
                .await?;

            Ok(User {
                id: user.id,
                email: user.email,
                password: user.password,
                name: user.name,
                role: user.role,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
        }
    }
}
