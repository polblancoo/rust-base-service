use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub telegram_user_id: Option<String>,
    pub name: Option<String>,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserSchema {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(email(message = "Invalid email format"))]
    pub email:  Option<String>,
      #[serde(default)]
      pub telegram_user_id: Option<String>,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
