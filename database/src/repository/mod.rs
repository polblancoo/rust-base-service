use anyhow::Result;
use models::{CreateUserSchema, User};
use repository::UserRepository;
use sqlx::PgPool;
use uuid::Uuid;
use std::future::Future;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    fn create_user<'a>(&'a self, user_data: &'a CreateUserSchema, hashed_password: &'a str, telegram_user_id: Option<String>) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            // Utilizamos query! en lugar de query_as! para tener más control sobre los campos
            let row = sqlx::query!(
                r#"
                INSERT INTO users (email, password, name, role, telegram_user_id)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, email, password, name, role, created_at, updated_at
                "#,
                user_data.email,
                hashed_password,
                user_data.name,
                "user",
                telegram_user_id
            )
            .fetch_one(&self.pool)
            .await?;
            
            // Convertimos manualmente el resultado a la estructura User
            Ok(User {
                id: row.id,
                email: row.email,
                password: row.password,
                name: row.name.expect("El nombre no puede ser nulo"),
                role: row.role,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
        }
    }
    
    fn find_user_by_id<'a>(&'a self, user_id: &'a Uuid) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            let row = sqlx::query!(
                r#"SELECT id, email, password, name, role, created_at, updated_at FROM users WHERE id = $1"#,
                user_id
            )
            .fetch_one(&self.pool)
            .await?;
            
            Ok(User {
                id: row.id,
                email: row.email,
                password: row.password,
                name: row.name.expect("El nombre no puede ser nulo"),
                role: row.role,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
        }
    }

    fn find_user_by_email<'a>(&'a self, email: &'a str) -> impl Future<Output = Result<User>> + Send + 'a {
        async move {
            let row = sqlx::query!(
                r#"SELECT id, email, password, name, role, created_at, updated_at FROM users WHERE email = $1"#,
                email
            )
            .fetch_one(&self.pool)
            .await?;
            
            Ok(User {
                id: row.id,
                email: row.email,
                password: row.password,
                name: row.name.expect("El nombre no puede ser nulo"),
                role: row.role,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
        }
    }
}