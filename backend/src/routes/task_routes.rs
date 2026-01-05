use crate::handlers::task_handlers;
use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn create_task_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(task_handlers::get_tasks))
        .route("/", post(task_handlers::create_task))
        .route("/:id", get(task_handlers::get_task_by_id))
        .route("/:id", put(task_handlers::update_task))
        .route("/:id", delete(task_handlers::delete_task))
}