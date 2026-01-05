use crate::handlers::todo_handlers;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

// 4. State (Router<Arc<AppState>>)
pub fn create_todo_routes() -> Router<Arc<AppState>> {
    // 1. Pembuatan Instance Router (Router::new())
    Router::new()
        // 2. logika penentuan alamat yang paling penting dalam REST API.
        // 3. penyambung kabel dari HTTP request ke logika bisnis.
        .route("/", get(todo_handlers::get_todos))
        .route("/", post(todo_handlers::create_todo))
        .route("/:id", get(todo_handlers::get_todo_by_id))
        .route("/:id", put(todo_handlers::update_todo))
        .route("/:id", delete(todo_handlers::delete_todo))
}
