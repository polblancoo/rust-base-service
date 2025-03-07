use std::sync::Arc;
use crate::handlers::auth::AuthService;

// Definimos AppState sin genéricos para simplificar
pub struct AppState {
    pub auth_service: Arc<dyn AuthService>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(auth_service: Arc<dyn AuthService>, jwt_secret: String) -> Self {
        Self {
            auth_service,
            jwt_secret,
        }
    }
} 