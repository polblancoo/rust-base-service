# Rust Base Service

Un servicio base en Rust que proporciona autenticación y gestión de usuarios con soporte para autenticación por email y Telegram.

## Características

- Autenticación de usuarios mediante email/contraseña
- Autenticación de usuarios mediante Telegram ID
- Generación y validación de tokens JWT
- Endpoints protegidos con middleware de autenticación
- Base de datos PostgreSQL con migraciones automáticas
- Estructura modular con crates separados para diferentes funcionalidades

## Estructura del Proyecto

El proyecto está organizado en varios crates:

- **api**: Contiene los handlers y rutas HTTP
- **auth**: Implementa la lógica de autenticación y autorización
- **common**: Utilidades comunes como JWT, errores, etc.
- **database**: Gestión de la conexión a la base de datos y migraciones
- **repository**: Implementación de los repositorios para acceso a datos
- **server**: Punto de entrada de la aplicación
- **shared**: Modelos y estructuras compartidas entre crates

## Endpoints

El servicio expone los siguientes endpoints:

- `POST /api/auth/register`: Registro de usuarios
- `POST /api/auth/login`: Inicio de sesión (email o Telegram)
- `GET /api/users/me`: Información del usuario autenticado

Para más detalles sobre los endpoints y ejemplos de uso, consulta [API_DOCUMENTATION.md](./API_DOCUMENTATION.md).

## Requisitos

- Rust 1.70 o superior
- PostgreSQL 13 o superior
- Docker (opcional, para desarrollo)

## Configuración

1. Copia el archivo `.envCopy` a `.env` y configura las variables de entorno:

```bash
cp .envCopy .env
```

2. Edita el archivo `.env` con tus configuraciones específicas.

## Ejecución

### Desarrollo

```bash
cargo run
```

### Producción

```bash
cargo build --release
./target/release/server
```

## Migraciones de Base de Datos

Las migraciones se ejecutan automáticamente al iniciar la aplicación. Los archivos de migración se encuentran en `database/migrations/`.

## Pruebas

```bash
cargo test
```

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.
