use crate::models::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::TaskService;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use bson::oid::ObjectId;
use serde_json::json;

// Get all tasks
pub async fn get_tasks(
    State(state): State<std::sync::Arc<AppState>>,
) -> AxumJson<serde_json::Value> {
    let task_service = TaskService::new(&state.db.database);

    match task_service.get_all_tasks().await {
        Ok(tasks) => AxumJson(json!({
            "success": true,
            "data": &tasks,
            "count": tasks.len()
        })),
        Err(_) => AxumJson(json!({
            "success": false,
            "message": "Failed to retrieve tasks"
        }))
    }
}

// Get a single task by ID
pub async fn get_task_by_id(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    match task_service.get_task_by_id(id).await {
        Ok(Some(task)) => Ok(AxumJson(json!({
            "success": true,
            "data": task
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Create a new task
pub async fn create_task(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string user ID to ObjectId
    let user_id = match ObjectId::parse_str(&payload.user_id) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    match task_service.create_task(payload, user_id).await {
        Ok(new_task) => Ok(AxumJson(json!({
            "success": true,
            "data": new_task,
            "message": "Task created successfully"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Update a task
pub async fn update_task(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert string user ID to ObjectId if provided
    let user_id = if let Some(user_id_str) = &payload.user_id {
        match ObjectId::parse_str(user_id_str) {
            Ok(oid) => Some(oid),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    } else {
        None
    };

    let task_service = TaskService::new(&state.db.database);

    match task_service.update_task(id, payload, user_id).await {
        Ok(Some(updated_task)) => Ok(AxumJson(json!({
            "success": true,
            "data": updated_task,
            "message": "Task updated successfully"
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete a task
pub async fn delete_task(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    match task_service.delete_task(id).await {
        Ok(true) => Ok(AxumJson(json!({
            "success": true,
            "message": "Task deleted successfully"
        }))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}