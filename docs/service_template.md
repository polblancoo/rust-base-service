# Plantilla para Nuevos Servicios

Esta plantilla define la estructura recomendada para agregar nuevos servicios al proyecto.

## Estructura de Directorios

```
new_service/
├── Cargo.toml
└── src/
    ├── lib.rs          # Exporta los módulos públicos
    ├── service.rs      # Implementación del servicio
    ├── error.rs        # Errores específicos del servicio
    ├── models.rs       # Modelos de datos específicos del servicio
    └── repository.rs   # Acceso a datos específico del servicio (opcional)
```

## Cargo.toml

```toml
[package]
name = "new_service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
async-trait = { workspace = true }
common = { path = "../common" }
database = { path = "../database" }
shared = { path = "../shared" }
```

## lib.rs

```rust
//! Módulo que proporciona funcionalidad para [descripción del servicio].

pub mod error;
pub mod models;
pub mod service;

// Exportaciones públicas
pub use error::ServiceError;
pub use models::{ModelA, ModelB};
pub use service::{ServiceTrait, ServiceImpl};
```

## service.rs

```rust
//! Implementación del servicio para [descripción del servicio].

use anyhow::Result;
use async_trait::async_trait;
use tracing::{info, error, debug};
use std::sync::Arc;

use crate::error::ServiceError;
use crate::models::{ModelA, ModelB};

/// Trait que define las operaciones disponibles en el servicio.
#[async_trait]
pub trait ServiceTrait: Send + Sync {
    /// Descripción de la operación
    async fn operation_a(&self, param: &ModelA) -> Result<ModelB, ServiceError>;
    
    /// Descripción de la operación
    async fn operation_b(&self, id: &str) -> Result<ModelA, ServiceError>;
}

/// Implementación concreta del servicio.
pub struct ServiceImpl {
    // Dependencias del servicio
    repository: Arc<dyn RepositoryTrait>,
}

impl ServiceImpl {
    /// Crea una nueva instancia del servicio.
    pub fn new(repository: Arc<dyn RepositoryTrait>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ServiceTrait for ServiceImpl {
    async fn operation_a(&self, param: &ModelA) -> Result<ModelB, ServiceError> {
        info!("Iniciando operation_a con parámetro: {:?}", param);
        
        // Implementación de la operación
        let result = self.repository.some_operation(param).await
            .map_err(|e| {
                error!("Error en operation_a: {}", e);
                ServiceError::OperationFailed(e.to_string())
            })?;
            
        debug!("operation_a completada exitosamente");
        Ok(result)
    }
    
    async fn operation_b(&self, id: &str) -> Result<ModelA, ServiceError> {
        info!("Iniciando operation_b con id: {}", id);
        
        // Implementación de la operación
        let result = self.repository.find_by_id(id).await
            .map_err(|e| {
                error!("Error en operation_b: {}", e);
                ServiceError::NotFound(format!("Recurso con id {} no encontrado", id))
            })?;
            
        debug!("operation_b completada exitosamente");
        Ok(result)
    }
}

/// Trait para el repositorio que utiliza el servicio.
#[async_trait]
pub trait RepositoryTrait: Send + Sync {
    async fn some_operation(&self, param: &ModelA) -> Result<ModelB>;
    async fn find_by_id(&self, id: &str) -> Result<ModelA>;
}
```

## error.rs

```rust
//! Errores específicos para el servicio.

use thiserror::Error;

/// Errores que pueden ocurrir en el servicio.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Error cuando una operación falla.
    #[error("La operación falló: {0}")]
    OperationFailed(String),
    
    /// Error cuando un recurso no se encuentra.
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),
    
    /// Error cuando los datos de entrada son inválidos.
    #[error("Datos inválidos: {0}")]
    InvalidData(String),
    
    /// Error cuando ocurre un problema de autorización.
    #[error("No autorizado: {0}")]
    Unauthorized(String),
}

impl From<anyhow::Error> for ServiceError {
    fn from(err: anyhow::Error) -> Self {
        ServiceError::OperationFailed(err.to_string())
    }
}
```

## models.rs

```rust
//! Modelos de datos específicos para el servicio.

use serde::{Serialize, Deserialize};
use validator::Validate;

/// Modelo para la entrada de datos.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ModelA {
    /// Identificador único
    pub id: String,
    
    /// Nombre descriptivo
    #[validate(length(min = 1, message = "El nombre no puede estar vacío"))]
    pub name: String,
    
    /// Datos adicionales
    pub data: Option<String>,
}

/// Modelo para la salida de datos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelB {
    /// Identificador único
    pub id: String,
    
    /// Resultado de la operación
    pub result: String,
    
    /// Timestamp de la operación
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
```

## repository.rs (opcional)

```rust
//! Implementación del repositorio para el servicio.

use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{debug, error};

use crate::models::{ModelA, ModelB};
use crate::service::RepositoryTrait;

/// Implementación del repositorio usando PostgreSQL.
pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    /// Crea una nueva instancia del repositorio.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepositoryTrait for PgRepository {
    async fn some_operation(&self, param: &ModelA) -> Result<ModelB> {
        debug!("Ejecutando some_operation en la base de datos para: {:?}", param);
        
        // Implementación de la operación en la base de datos
        let row = sqlx::query!(
            r#"
            INSERT INTO some_table (name, data)
            VALUES ($1, $2)
            RETURNING id, created_at
            "#,
            param.name,
            param.data
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(ModelB {
            id: row.id.to_string(),
            result: "Operación completada".to_string(),
            timestamp: row.created_at,
        })
    }
    
    async fn find_by_id(&self, id: &str) -> Result<ModelA> {
        debug!("Buscando en la base de datos por id: {}", id);
        
        // Implementación de la búsqueda en la base de datos
        let row = sqlx::query!(
            r#"
            SELECT id, name, data
            FROM some_table
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(ModelA {
            id: row.id.to_string(),
            name: row.name,
            data: row.data,
        })
    }
}
```

## Integración con API

Para integrar el nuevo servicio con la API, sigue estos pasos:

1. Agregar el servicio al `AppState` en `api/src/state.rs`
2. Crear un nuevo módulo de handlers en `api/src/handlers/`
3. Agregar las rutas en `api/src/routes.rs`

### Ejemplo de Handler

```rust
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use common::error::AppError;
use std::sync::Arc;
use tracing::info;

use crate::AppState;
use new_service::{ModelA, ServiceTrait};

pub async fn create_resource(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ModelA>,
) -> Result<Json<ModelB>, AppError> {
    info!("Recibida solicitud para crear recurso: {:?}", payload);
    
    let result = state.new_service.operation_a(&payload)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(Json(result))
}

pub async fn get_resource(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ModelA>, AppError> {
    info!("Recibida solicitud para obtener recurso con id: {}", id);
    
    let result = state.new_service.operation_b(&id)
        .await
        .map_err(|e| match e {
            ServiceError::NotFound(_) => AppError::NotFound(format!("Recurso {} no encontrado", id)),
            _ => AppError::Internal(e.to_string()),
        })?;
    
    Ok(Json(result))
}
```
