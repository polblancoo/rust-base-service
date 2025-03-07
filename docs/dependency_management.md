# Gestión de Dependencias

Este documento proporciona directrices para la gestión de dependencias en el proyecto.

## Principios Generales

1. **Utilizar el Workspace**: Siempre que sea posible, las dependencias comunes deben definirse en el `Cargo.toml` del workspace y referenciarse en los crates individuales con `workspace = true`.

2. **Versiones Específicas**: Especificar versiones exactas para todas las dependencias para garantizar la reproducibilidad de las compilaciones.

3. **Minimizar Dependencias**: Evaluar cuidadosamente la necesidad de cada dependencia antes de agregarla al proyecto.

4. **Auditoría Regular**: Realizar auditorías periódicas de seguridad y actualizaciones de dependencias.

## Dependencias Principales

| Dependencia | Versión | Propósito | Alternativas Consideradas |
|-------------|---------|-----------|--------------------------|
| tokio | 1.34 | Runtime asíncrono | async-std |
| axum | 0.7 | Framework web | actix-web, rocket |
| tower | 0.4 | Middleware para servicios HTTP | - |
| tower-http | 0.5 | Extensiones HTTP para Tower | - |
| tracing | 0.1 | Logging y tracing | log |
| tracing-subscriber | 0.3 | Suscriptores para tracing | - |
| serde | 1.0 | Serialización/deserialización | - |
| sqlx | 0.7 | Acceso a base de datos | diesel, sea-orm |
| jsonwebtoken | 9.1 | Manejo de JWT | - |
| argon2 | 0.5 | Hashing de contraseñas | bcrypt |
| anyhow | 1.0 | Manejo de errores genéricos | - |
| thiserror | 1.0 | Definición de errores tipados | - |
| validator | 0.16 | Validación de datos | - |

## Estructura de Dependencias

```
rust-base-service (workspace)
├── api
│   ├── axum
│   ├── tower
│   └── tower-http
├── auth
│   ├── argon2
│   └── jsonwebtoken
├── database
│   └── sqlx
├── common
│   ├── serde
│   ├── anyhow
│   └── thiserror
└── server
    ├── tokio
    └── tracing
```

## Directrices para Agregar Nuevas Dependencias

1. **Evaluación**:
   - ¿Es realmente necesaria esta dependencia?
   - ¿Podemos implementar la funcionalidad nosotros mismos sin mucho esfuerzo?
   - ¿La dependencia está bien mantenida y tiene una comunidad activa?

2. **Seguridad**:
   - Verificar si la dependencia tiene vulnerabilidades conocidas
   - Considerar el historial de seguridad del mantenedor

3. **Rendimiento**:
   - Evaluar el impacto en el tiempo de compilación
   - Considerar el tamaño del binario resultante
   - Evaluar el rendimiento en tiempo de ejecución

4. **Licencia**:
   - Asegurarse de que la licencia sea compatible con nuestro proyecto

## Proceso de Actualización

1. **Programación Regular**:
   - Programar actualizaciones de dependencias trimestralmente
   - Mantener un registro de cambios importantes en cada actualización

2. **Pruebas**:
   - Ejecutar pruebas automatizadas después de cada actualización
   - Realizar pruebas manuales para funcionalidades críticas

3. **Implementación Gradual**:
   - Actualizar primero en entornos de desarrollo
   - Luego en entornos de prueba
   - Finalmente en producción

## Dependencias Problemáticas

Mantener una lista de dependencias que han causado problemas en el pasado:

| Dependencia | Problema | Solución/Alternativa |
|-------------|----------|----------------------|
| *Ejemplo:* bcrypt | Problemas de rendimiento en compilación | Migrado a argon2 |

## Herramientas de Gestión

- **cargo-audit**: Para verificar vulnerabilidades de seguridad
- **cargo-outdated**: Para identificar dependencias desactualizadas
- **cargo-deny**: Para aplicar políticas de licencias y dependencias
