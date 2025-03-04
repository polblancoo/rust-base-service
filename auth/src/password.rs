use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::AuthError;

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::PasswordHashError(e.to_string()))?
        .to_string();
    
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AuthError::PasswordVerifyError(e.to_string()))?;
    
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    
    Ok(is_valid)
}
