use serde::Deserialize;

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
        // Intentar cargar desde archivo .env
        let _ = dotenv::dotenv();
        
        // Crear una configuración con valores predeterminados y fuentes
        let config = config::Config::builder()
            .set_default("port", 8000)?
            .set_default("jwt_expires_in", "60m")?
            .set_default("jwt_maxage", 60)?
            .add_source(config::Environment::default())
            .build()?;
        
        // Deserializar la configuración
        config.try_deserialize::<AppConfig>()
    }
}
