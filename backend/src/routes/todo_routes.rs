use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::todo_handlers;
use crate::AppState;
use std::sync::Arc;

pub fn create_todo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(todo_handlers::get_todos))
        .route("/", post(todo_handlers::create_todo))
        .route("/:id", get(todo_handlers::get_todo_by_id))
        .route("/:id", put(todo_handlers::update_todo))
        .route("/:id", delete(todo_handlers::delete_todo))
}