use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tracing::{debug, error, info};

use crate::error::AuthError;

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    info!("Iniciando proceso de hash de contraseña");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    debug!("Generando hash con Argon2");
    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => {
            debug!("Hash generado correctamente");
            hash.to_string()
        },
        Err(e) => {
            error!("Error al generar hash: {}", e);
            return Err(AuthError::PasswordHashError(e.to_string()));
        }
    };
    
    Ok(password_hash)
}

pub fn verify_password(provided_password: &str, stored_hash: &str) -> Result<bool, AuthError> {
    info!("Verificando contraseña");
    debug!("Hash almacenado: {}", stored_hash);
    
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(hash) => {
            debug!("Hash parseado correctamente");
            hash
        },
        Err(e) => {
            error!("Error al parsear hash: {}", e);
            return Err(AuthError::PasswordVerifyError(e.to_string()));
        }
    };
    
    debug!("Verificando contraseña con Argon2");
    let result = Argon2::default().verify_password(provided_password.as_bytes(), &parsed_hash);
    
    let is_valid = result.is_ok();
    if is_valid {
        info!("Verificación de contraseña exitosa");
    } else {
        info!("Verificación de contraseña fallida");
        if let Err(e) = result {
            debug!("Error de verificación: {}", e);
        }
    }
    
    Ok(is_valid)
}
