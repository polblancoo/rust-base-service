// src/main.rs
use std::sync::Arc;
use api::routes::create_router;
use auth::service::AuthService as AuthServiceImpl;
use common::config::AppConfig;
use api::AppState;
use database::pool;
use database::repository::PgUserRepository;
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tower_http::trace::{self, TraceLayer};

// Definición de estado de la aplicación

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar el logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "server=debug,api=debug,auth=debug,tower_http=debug,axum::rejection=trace".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Iniciando el servidor...");

    // Cargar la configuración
    let config = AppConfig::init()?;
    info!("Configuración cargada correctamente");

    // Inicializar el pool de conexiones
    let db_pool = pool::init_pool(&config.database_url).await?;
    info!("Conexión a la base de datos establecida");

    // Crear el repositorio de usuarios
    let user_repo = PgUserRepository::new(db_pool);

    // Crear el servicio de autenticación
    let auth_service = AuthServiceImpl::new(
        user_repo,
        config.jwt_secret.clone(),
        config.jwt_expires_in.clone(),
    );
    info!("Servicio de autenticación inicializado");

    // Crear el estado de la aplicación
    let app_state = Arc::new(AppState {
        auth_service: Arc::new(auth_service),
        jwt_secret: config.jwt_secret.clone(),
    });

    // Crear el enrutador con capa de logging
    let router = create_router(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_failure(trace::DefaultOnFailure::new().level(Level::ERROR)),
        );
    
    info!("Router configurado con capas de logging");

    // Iniciar el servidor
    let addr = format!("0.0.0.0:{}", config.port);
    info!("Servidor escuchando en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}