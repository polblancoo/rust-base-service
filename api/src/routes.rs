use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::{
    handlers::{
        auth::{login_handler, register_handler},
        me::me_handler,
    },
    middleware::auth::auth_middleware,

};
use shared::AppState;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler));
    
    let protected_routes = Router::new()
        .route("/me", get(me_handler))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware));
    
    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/users", protected_routes)
        .with_state(app_state)
}