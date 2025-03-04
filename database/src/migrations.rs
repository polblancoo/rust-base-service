use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, postgres::Postgres, Executor, PgPool};
use tracing::info;

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    info!("Running database migrations");
    
    // Crear tabla de usuarios si no existe
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            name VARCHAR(255),
            role VARCHAR(50) NOT NULL DEFAULT 'user',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .await?;
    
    info!("Migrations completed successfully");
    
    Ok(())
}

pub async fn ensure_database_exists(database_url: &str) -> Result<()> {
    let db_url_parts: Vec<&str> = database_url.split('/').collect();
    
    if db_url_parts.len() < 4 {
        return Err(anyhow::anyhow!("Invalid database URL format"));
    }
    
    let db_name = db_url_parts.last().unwrap();
    let server_url = database_url.replace(&format!("/{}", db_name), "");
    
    if !Postgres::database_exists(&server_url).await? {
        info!("Creating database {}", db_name);
        Postgres::create_database(&database_url).await?;
    }
    
    Ok(())
}