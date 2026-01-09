use crate::models::task::{CreateTaskRequest, UpdateTaskRequest};
use crate::services::TaskService;
use crate::utils::auth::AuthUser;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use bson::oid::ObjectId;
use serde_json::json;

// Get all tasks (for authenticated user)
#[axum::debug_handler]
pub async fn get_tasks(
    State(state): State<std::sync::Arc<AppState>>,
    AuthUser { user_id }: AuthUser,
) -> AxumJson<serde_json::Value> {
    let task_service = TaskService::new(&state.db.database);

    // Convert user_id string to ObjectId for filtering
    let user_object_id = match ObjectId::parse_str(&user_id) {
        Ok(oid) => oid,
        Err(_) => {
            return AxumJson(json!({
                "success": false,
                "message": "Invalid user ID format",
                "error": "Invalid user ID"
            }));
        }
    };

    match task_service.get_tasks_by_user(user_object_id).await {
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
#[axum::debug_handler]
pub async fn get_task_by_id(
    State(state): State<std::sync::Arc<AppState>>,
    AuthUser { user_id }: AuthUser,
    Path(id_str): Path<String>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert user_id string to ObjectId for comparison
    let user_object_id = match ObjectId::parse_str(&user_id) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    // Check if the task exists and belongs to the user
    match task_service.get_task_by_id(id).await {
        Ok(Some(task)) => {
            if task.user_id != user_object_id {
                return Err(StatusCode::FORBIDDEN);
            }

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
#[axum::debug_handler]
pub async fn create_task(
    State(state): State<std::sync::Arc<AppState>>,
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert user_id string from token to ObjectId
    let user_object_id = match ObjectId::parse_str(&user_id) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    // Create task with authenticated user's ID, ignoring any user_id in the payload
    let create_request = CreateTaskRequest {
        title: payload.title,
        description: payload.description,
        user_id: user_id, // Use the authenticated user's ID
    };

    match task_service
        .create_task(create_request, user_object_id)
        .await
    {
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
#[axum::debug_handler]
pub async fn update_task(
    State(state): State<std::sync::Arc<AppState>>,
    AuthUser { user_id }: AuthUser,
    Path(id_str): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert user_id string to ObjectId for comparison
    let user_object_id = match ObjectId::parse_str(&user_id) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    // First, check if the task exists and belongs to the user
    if let Some(existing_task) = task_service.get_task_by_id(id).await.ok().flatten() {
        if existing_task.user_id != user_object_id {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    match task_service
        .update_task(id, payload, Some(user_object_id))
        .await
    {
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
#[axum::debug_handler]
pub async fn delete_task(
    State(state): State<std::sync::Arc<AppState>>,
    AuthUser { user_id }: AuthUser,
    Path(id_str): Path<String>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Convert user_id string to ObjectId for comparison
    let user_object_id = match ObjectId::parse_str(&user_id) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let task_service = TaskService::new(&state.db.database);

    // First, check if the task exists and belongs to the user
    if let Some(existing_task) = task_service.get_task_by_id(id).await.ok().flatten() {
        if existing_task.user_id != user_object_id {
            return Err(StatusCode::FORBIDDEN);
        }
    } else {
        return Err(StatusCode::NOT_FOUND);
    }

    match task_service.delete_task(id).await {
        Ok(true) => Ok(AxumJson(json!({
            "success": true,
            "message": "Task deleted successfully"
        }))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
