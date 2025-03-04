use serde::Deserialize;
use std::env;

#[derive(Debug,Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
    pub port: u16,
}

impl AppConfig {
    pub fn init() -> Result<AppConfig, config::ConfigError> {
        let mut cfg = config::Config::default();
        
        // Intentar cargar desde archivo .env
        let _ = dotenv::dotenv();
        
        // Establecer valores predeterminados
        cfg.set_default("port", 8000)?;
        cfg.set_default("jwt_expires_in", "60m")?;
        cfg.set_default("jwt_maxage", 60)?;

        // Cargar desde variables de entorno
        cfg.merge(config::Environment::default())?;
        
        //- Usar el método específico de la biblioteca config
        cfg.try_deserialize::<AppConfig>()

    }
}
