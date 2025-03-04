use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//use crate::error::AppError;
use crate::error::AppError;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,     // subject (user id)
    pub exp: usize,      // expiration time
    pub iat: usize,      // issued at
}

pub fn generate_jwt(
    user_id: &Uuid,
    jwt_secret: &str,
    expiration: &str,
) -> Result<String, AppError> {
    let now = Utc::now();
    let expires_in = parse_duration(expiration)
        .map_err(|_| AppError::TokenGenerationError("Invalid expiration format".into()))?;
    let exp = (now + expires_in).timestamp() as usize;
    let iat = now.timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::TokenGenerationError(e.to_string()))?;
    
    Ok(token)
}

pub fn verify_jwt(token: &str, jwt_secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        if e.to_string().contains("expired") {
            AppError::TokenExpired
        } else {
            AppError::InvalidToken(e.to_string())
        }
    })?;
    
    Ok(token_data.claims)
}

// Helper para convertir strings como "60m" a Duration
fn parse_duration(duration_str: &str) -> Result<Duration> {
    let last_char = duration_str.chars().last().unwrap_or('s');
    let value = duration_str[0..duration_str.len() - 1]
        .parse::<i64>()
        .unwrap_or(0);
    
    match last_char {
        's' => Ok(Duration::seconds(value)),
        'm' => Ok(Duration::minutes(value)),
        'h' => Ok(Duration::hours(value)),
        'd' => Ok(Duration::days(value)),
        _ => Err(anyhow::anyhow!("Invalid duration format")),
    }
}
