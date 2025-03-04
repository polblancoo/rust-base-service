// src/main.rs
use anyhow::Result;
use api::routes::create_router;
use auth::service::AuthService;
use common::config::AppConfig;
use shared::AppState;
use database::{migrations, pool};
use database::repository::PgUserRepository;
use std::sync::Arc;
use tracing::info;

// Definición de estado de la aplicación

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar el logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    // Cargar configuración
    let config = AppConfig::init()?;
    
    // Asegurarse de que la base de datos existe
    migrations::ensure_database_exists(&config.database_url).await?;
    
    // Inicializar el pool de conexiones
    let db_pool = pool::init_pool(&config.database_url).await?;
    
    // Ejecutar migraciones
    migrations::run_migrations(&db_pool).await?;
    
    // Inicializar repositorios
    let user_repo = PgUserRepository::new(db_pool.clone());
    
    // Inicializar servicios
    let auth_service = AuthService::new(
        user_repo,
        config.jwt_secret.clone(),
        config.jwt_expires_in.clone(),
    );
    
    // Crear el estado de la aplicación
    let app_state = Arc::new(AppState {
        config: config.clone(),
        auth_service,
    });
    
    // Crear el router
    let router = create_router(app_state);
    
    // Iniciar el servidor
    let addr = format!("0.0.0.0:{}", config.port);
    info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    //info!("Server running on {}", addr);
    axum::serve(listener, router).await?;
    
    Ok(())
}