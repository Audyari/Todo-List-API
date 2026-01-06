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
        Ok(tasks) => {
            // Convert tasks to a format that handles optional fields properly
            let tasks_json: Vec<serde_json::Value> = tasks
                .iter()
                .map(|task| {
                    json!({
                        "id": task.id,
                        "title": task.title,
                        "description": task.description,
                        "completed": task.completed,
                        "userId": task.user_id,
                        "createdAt": task.created_at,
                        "updatedAt": task.updated_at
                    })
                })
                .collect();

            AxumJson(json!({
                "success": true,
                "data": tasks_json,
                "count": tasks.len()
            }))
        }
        Err(e) => {
            eprintln!("Error getting tasks: {}", e);
            AxumJson(json!({
                "success": false,
                "message": "Failed to retrieve tasks",
                "error": e.to_string()
            }))
        }
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
        Ok(Some(task)) => {
            let task_json = json!({
                "id": task.id,
                "title": task.title,
                "description": task.description,
                "completed": task.completed,
                "userId": task.user_id,
                "createdAt": task.created_at,
                "updatedAt": task.updated_at
            });
            Ok(AxumJson(json!({
                "success": true,
                "data": task_json
            })))
        }
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
        Ok(new_task) => {
            let task_json = json!({
                "id": new_task.id,
                "title": new_task.title,
                "description": new_task.description,
                "completed": new_task.completed,
                "userId": new_task.user_id,
                "createdAt": new_task.created_at,
                "updatedAt": new_task.updated_at
            });
            Ok(AxumJson(json!({
                "success": true,
                "data": task_json,
                "message": "Task created successfully"
            })))
        }
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
        Ok(Some(updated_task)) => {
            let task_json = json!({
                "id": updated_task.id,
                "title": updated_task.title,
                "description": updated_task.description,
                "completed": updated_task.completed,
                "userId": updated_task.user_id,
                "createdAt": updated_task.created_at,
                "updatedAt": updated_task.updated_at
            });
            Ok(AxumJson(json!({
                "success": true,
                "data": task_json,
                "message": "Task updated successfully"
            })))
        }
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
