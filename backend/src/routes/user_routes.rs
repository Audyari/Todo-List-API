use crate::handlers::user_handlers;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn create_user_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(user_handlers::get_users))
        .route("/", post(user_handlers::create_user))
        .route("/register", post(user_handlers::register_user))
        .route("/login", post(user_handlers::login_user))
        .route("/:id", get(user_handlers::get_user_by_id))
        .route("/:id", put(user_handlers::update_user))
        .route("/:id", delete(user_handlers::delete_user))
}