use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    // Puedes agregar más variantes según sea necesario
    #[error("Error al hashear la contraseña: {0}")]
    PasswordHashError(String),
    #[error("Error al verificar la contraseña: {0}")]
    PasswordVerifyError(String),
    #[error("Error al generar el token: {0}")]
    TokenGenerationError(String),
    #[error("Token inválido: {0}")]
    InvalidToken(String),
    #[error("Token expirado")]
    TokenExpired,
}
