
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn init_pool(database_url: &str) -> Result<DbPool> {
    info!("Initializing database connection pool");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}
