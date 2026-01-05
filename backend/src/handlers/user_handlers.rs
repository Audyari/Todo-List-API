use crate::models::user::{CreateUserRequest, UpdateUserRequest};
use crate::services::UserService;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use bson::oid::ObjectId;
use serde_json::json;

// Get all users
pub async fn get_users(
    State(state): State<std::sync::Arc<AppState>>,
) -> AxumJson<serde_json::Value> {
    let user_service = UserService::new(&state.db.database);

    match user_service.get_all_users().await {
        Ok(users) => AxumJson(json!({
            "success": true,
            "data": &users,
            "count": users.len()
        })),
        Err(_) => AxumJson(json!({
            "success": false,
            "message": "Failed to retrieve users"
        }))
    }
}

// Get a single user by ID
pub async fn get_user_by_id(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let user_service = UserService::new(&state.db.database);

    match user_service.get_user_by_id(id).await {
        Ok(Some(user)) => Ok(AxumJson(json!({
            "success": true,
            "data": user
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Create a new user
pub async fn create_user(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    let user_service = UserService::new(&state.db.database);

    match user_service.create_user(payload).await {
        Ok(new_user) => Ok(AxumJson(json!({
            "success": true,
            "data": new_user,
            "message": "User created successfully"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Update a user
pub async fn update_user(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let user_service = UserService::new(&state.db.database);

    match user_service.update_user(id, payload).await {
        Ok(Some(updated_user)) => Ok(AxumJson(json!({
            "success": true,
            "data": updated_user,
            "message": "User updated successfully"
        }))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Delete a user
pub async fn delete_user(
    Path(id_str): Path<String>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Convert string ID to ObjectId
    let id = match ObjectId::parse_str(&id_str) {
        Ok(oid) => oid,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let user_service = UserService::new(&state.db.database);

    match user_service.delete_user(id).await {
        Ok(true) => Ok(AxumJson(json!({
            "success": true,
            "message": "User deleted successfully"
        }))),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}