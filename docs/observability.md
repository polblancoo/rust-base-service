# Observabilidad y Monitoreo

Este documento describe la estrategia de observabilidad y monitoreo para el proyecto.

## Componentes de Observabilidad

La observabilidad se compone de tres pilares principales:

1. **Logs**: Registros de eventos que ocurren en el sistema
2. **Métricas**: Medidas numéricas del comportamiento del sistema
3. **Trazas**: Seguimiento del flujo de una solicitud a través de diferentes componentes

## Estrategia de Logging

### Niveles de Log

- **ERROR**: Errores que requieren intervención inmediata
- **WARN**: Situaciones anómalas que no impiden el funcionamiento
- **INFO**: Información general sobre el funcionamiento del sistema
- **DEBUG**: Información detallada útil para depuración
- **TRACE**: Información muy detallada para diagnóstico profundo

### Estructura de Logs

Todos los logs deben seguir una estructura consistente:

```json
{
  "timestamp": "2023-03-07T16:02:29-03:00",
  "level": "INFO",
  "target": "api::handlers::auth",
  "message": "Usuario autenticado correctamente",
  "request_id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "latency_ms": 42,
  "status_code": 200
}
```

### Contexto en Logs

Siempre incluir contexto relevante en los logs:

- Identificador de solicitud (request_id)
- Identificador de usuario (cuando aplique)
- Latencia de operación
- Códigos de estado HTTP (para APIs)
- Información específica del dominio

## Implementación de Métricas

### Métricas Clave

1. **Métricas de Sistema**:
   - Uso de CPU
   - Uso de memoria
   - Uso de disco
   - Conexiones de red

2. **Métricas de Aplicación**:
   - Tasa de solicitudes
   - Latencia de respuesta (p50, p90, p99)
   - Tasa de errores
   - Tiempo de respuesta de la base de datos

3. **Métricas de Negocio**:
   - Número de usuarios registrados
   - Número de sesiones activas
   - Tasa de conversión
   - Uso de funcionalidades específicas

### Implementación con Prometheus

Para implementar métricas, se recomienda usar la biblioteca `metrics` con el exportador de Prometheus:

```rust
use metrics::{counter, gauge, histogram};

// Incrementar un contador
counter!("api.requests.total", 1, "endpoint" => "/users", "method" => "POST");

// Establecer un gauge
gauge!("api.active_connections", 42.0);

// Registrar una latencia
histogram!("api.request.duration", latency_ms, "endpoint" => "/users");
```

## Trazas Distribuidas

Para implementar trazas distribuidas, se recomienda usar OpenTelemetry con Jaeger:

```rust
use opentelemetry::trace::{Tracer, SpanKind};
use opentelemetry::Context;

let tracer = opentelemetry::global::tracer("my-service");
let span = tracer.start_with_context("process_request", Context::current());
let cx = Context::current_with_span(span);

// Realizar operaciones dentro del span
// ...

span.end();
```

## Integración con el Middleware de Logging

El middleware de logging ya implementado debe extenderse para capturar métricas:

```rust
pub async fn logging_middleware<B>(
    request_id: Uuid,
    method: &Method,
    uri: &Uri,
    request_headers: &HeaderMap,
    response: Response<B>,
    latency: Duration,
) -> Response<B> {
    // Logging existente
    info!(
        request_id = %request_id,
        method = %method,
        uri = %uri,
        status = %response.status().as_u16(),
        latency_ms = %latency.as_millis(),
        "Request completed"
    );
    
    // Agregar métricas
    counter!("http.requests.total", 1, 
        "method" => method.to_string(),
        "path" => uri.path().to_string(),
        "status" => response.status().as_u16().to_string()
    );
    
    histogram!("http.request.duration", latency.as_millis() as f64,
        "method" => method.to_string(),
        "path" => uri.path().to_string()
    );
    
    response
}
```

## Configuración de Exportadores

### Exportador de Logs

Configurar el exportador de logs para enviar a un sistema centralizado como Elasticsearch:

```rust
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_elasticsearch::ElasticsearchLayer;

pub fn init_tracing(elasticsearch_url: &str) {
    let elasticsearch_layer = ElasticsearchLayer::new(elasticsearch_url);
    
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(elasticsearch_layer)
        .init();
}
```

### Exportador de Métricas

Configurar el exportador de métricas para Prometheus:

```rust
use metrics_exporter_prometheus::PrometheusBuilder;

pub fn init_metrics() {
    let builder = PrometheusBuilder::new();
    builder.install().expect("Failed to install Prometheus exporter");
}
```

## Dashboard y Alertas

### Grafana Dashboard

Crear dashboards en Grafana para visualizar:

1. **Dashboard de Salud del Sistema**:
   - Uso de recursos
   - Tasa de solicitudes
   - Tasa de errores

2. **Dashboard de Rendimiento**:
   - Latencia de API
   - Tiempo de respuesta de base de datos
   - Tiempos de procesamiento

3. **Dashboard de Negocio**:
   - Métricas específicas del dominio
   - KPIs

### Alertas

Configurar alertas para:

1. **Alertas de Disponibilidad**:
   - Servicio caído
   - Tasa de errores alta

2. **Alertas de Rendimiento**:
   - Latencia por encima del umbral
   - Uso de recursos alto

3. **Alertas de Negocio**:
   - Caída en métricas clave de negocio

## Plan de Implementación

1. **Fase 1: Logging Mejorado**
   - Implementar estructura de logs consistente
   - Agregar contexto a todos los logs
   - Configurar exportación a sistema centralizado

2. **Fase 2: Métricas Básicas**
   - Implementar métricas de sistema
   - Implementar métricas de API
   - Configurar Prometheus y Grafana

3. **Fase 3: Trazas Distribuidas**
   - Implementar OpenTelemetry
   - Configurar Jaeger
   - Integrar con servicios existentes

4. **Fase 4: Alertas y Dashboards**
   - Crear dashboards en Grafana
   - Configurar alertas
   - Establecer procedimientos de respuesta
