use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, instrument};
use uuid::Uuid;

/// Middleware para registrar información detallada de las solicitudes HTTP
#[instrument(skip_all)]
pub async fn logging_middleware(request: Request, next: Next) -> Response {
    // Generar un ID único para la solicitud
    let request_id = Uuid::new_v4();
    
    // Extraer información de la solicitud
    let method = request.method().clone();
    let uri = request.uri().clone();
    let version = request.version();
    
    // Registrar el inicio de la solicitud
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        version = ?version,
        "Solicitud recibida"
    );
    
    // Medir el tiempo de respuesta
    let start = Instant::now();
    
    // Procesar la solicitud
    let response = next.run(request).await;
    
    // Calcular el tiempo transcurrido
    let latency = start.elapsed();
    
    // Registrar información de la respuesta
    info!(
        request_id = %request_id,
        status = response.status().as_u16(),
        latency = ?latency,
        "Respuesta enviada"
    );
    
    response
}
