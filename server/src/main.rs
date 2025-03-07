// src/main.rs
use std::sync::Arc;
use api::routes::create_router;
use auth::service::AuthService as AuthServiceImpl;
use common::config::AppConfig;
use api::AppState;
use database::pool;
use database::repository::PgUserRepository;

// Definición de estado de la aplicación

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Cargar la configuración
    let config = AppConfig::init()?;

    // Inicializar el pool de conexiones
    let db_pool = pool::init_pool(&config.database_url).await?;

    // Crear el repositorio de usuarios
    let user_repo = PgUserRepository::new(db_pool);

    // Crear el servicio de autenticación
    let auth_service = AuthServiceImpl::new(
        user_repo,
        config.jwt_secret.clone(),
        config.jwt_expires_in.clone(),
    );

    // Crear el estado de la aplicación
    let app_state = Arc::new(AppState {
        auth_service: Arc::new(auth_service),
        jwt_secret: config.jwt_secret.clone(),
    });

    // Crear el enrutador
    let router = create_router(app_state);

    // Iniciar el servidor
    let addr = format!("0.0.0.0:{}", config.port);
    println!("Servidor escuchando en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}