# API Documentation

Este documento describe los endpoints disponibles en el servicio base de Rust y cómo utilizarlos con `curl`.

## Autenticación

Todos los endpoints protegidos requieren un token JWT que debe ser incluido en el encabezado `Authorization` con el formato `Bearer <token>`.

## Endpoints

### Registro de Usuario

Registra un nuevo usuario en el sistema.

- **URL**: `/api/auth/register`
- **Método**: `POST`
- **Parámetros de consulta**:
  - `telegram_user_id` (opcional): ID de usuario de Telegram

- **Cuerpo de la solicitud**:
  ```json
  {
    "email": "usuario@ejemplo.com",
    "password": "contraseña123",
    "name": "Nombre Usuario",
    "role": "user"  // Opcional, por defecto es "user"
  }
  ```

- **Respuesta exitosa**:
  ```json
  {
    "status": "success",
    "message": "User registered successfully",
    "user": {
      "id": "uuid-generado",
      "email": "usuario@ejemplo.com",
      "name": "Nombre Usuario",
      "role": "user",
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

- **Ejemplo con curl**:
  ```bash
  curl -X POST http://localhost:8000/api/auth/register \
    -H "Content-Type: application/json" \
    -d '{
      "email": "usuario@ejemplo.com",
      "password": "contraseña123",
      "name": "Nombre Usuario"
    }'
  ```

- **Ejemplo con curl y parámetro de Telegram**:
  ```bash
  curl -X POST "http://localhost:8000/api/auth/register?telegram_user_id=123456789" \
    -H "Content-Type: application/json" \
    -d '{
      "email": "usuario@ejemplo.com",
      "password": "contraseña123",
      "name": "Nombre Usuario"
    }'
  ```

### Inicio de Sesión

Autentica a un usuario y devuelve un token JWT.

- **URL**: `/api/auth/login`
- **Método**: `POST`
- **Cuerpo de la solicitud**:
  ```json
  {
    "email": "usuario@ejemplo.com",
    "password": "contraseña123"
  }
  ```
  o
  ```json
  {
    "telegram_user_id": "123456789",
    "password": "contraseña123"
  }
  ```

- **Respuesta exitosa**:
  ```json
  {
    "token": "jwt-token"
  }
  ```

- **Ejemplo con curl (email)**:
  ```bash
  curl -X POST http://localhost:8000/api/auth/login \
    -H "Content-Type: application/json" \
    -d '{
      "email": "usuario@ejemplo.com",
      "password": "contraseña123"
    }'
  ```

- **Ejemplo con curl (Telegram)**:
  ```bash
  curl -X POST http://localhost:8000/api/auth/login \
    -H "Content-Type: application/json" \
    -d '{
      "telegram_user_id": "123456789",
      "password": "contraseña123"
    }'
  ```

### Información del Usuario Actual

Obtiene la información del usuario autenticado.

- **URL**: `/api/users/me`
- **Método**: `GET`
- **Encabezados**:
  - `Authorization`: `Bearer <token>`

- **Respuesta exitosa**:
  ```json
  {
    "status": "success",
    "user": {
      "id": "uuid-del-usuario",
      "email": "usuario@ejemplo.com",
      "name": "Nombre Usuario",
      "role": "user",
      "created_at": "2023-01-01T00:00:00Z",
      "updated_at": "2023-01-01T00:00:00Z"
    }
  }
  ```

- **Ejemplo con curl**:
  ```bash
  curl -X GET http://localhost:8000/api/users/me \
    -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  ```

## Códigos de Estado

- `200 OK`: La solicitud se ha completado con éxito.
- `400 Bad Request`: La solicitud contiene datos inválidos.
- `401 Unauthorized`: Autenticación fallida o token inválido.
- `403 Forbidden`: El usuario no tiene permisos para acceder al recurso.
- `500 Internal Server Error`: Error interno del servidor.

## Flujo de Trabajo Típico

1. Registrar un usuario con `/api/auth/register`
2. Iniciar sesión con `/api/auth/login` para obtener un token
3. Usar el token para acceder a endpoints protegidos como `/api/users/me`
