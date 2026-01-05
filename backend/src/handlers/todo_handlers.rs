use crate::models::todo::{CreateTodoRequest, Todo, UpdateTodoRequest};
use crate::services::todo_service;
use crate::AppState;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as AxumJson,
};
use serde_json::json;

// Get all todos
pub async fn get_todos(
    State(state): State<std::sync::Arc<AppState>>,
) -> AxumJson<serde_json::Value> {
    let todos = state.todos.lock().unwrap();
    AxumJson(json!({
        "success": true,
        "data": &*todos,
        "count": todos.len()
    }))
}

// Get a single todo by ID
pub async fn get_todo_by_id(
    // 2. Cara Axum meng   ambil data dari Request.
    Path(id): Path<i32>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    let todos = state.todos.lock().unwrap();
    let todo = todos.iter().find(|t| t.id == Some(id)).cloned();

    match todo {
        Some(todo) => Ok(AxumJson(json!({
            "success": true,
            "data": todo
        }))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// fungsi membuat todo baru
pub async fn create_todo(
    State(state): State<std::sync::Arc<AppState>>,
    // 2. Cara Axum mengambil data dari Request.
    Json(payload): Json<CreateTodoRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Validate the request
    if let Err(_validation_error) = todo_service::validate_todo_request(&payload) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 1. Penguncian State Aman (state.todos.lock().unwrap())
    let mut todos = state.todos.lock().unwrap();

    let new_todo = Todo {
        id: Some(todos.len() as i32 + 1),
        title: payload.title,
        description: payload.description,
        completed: false,
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    todos.push(new_todo.clone());

    Ok(AxumJson(json!({
        "success": true,
        "data": new_todo,
        "message": "Todo created successfully"
    })))
}

// fungsi update todo
pub async fn update_todo(
    //2. Cara Axum mengambil data dari Request.
    Path(id): Path<i32>,
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // Validate the request if there are fields to update
    if let Err(_validation_error) = todo_service::validate_update_request(&payload) {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 1. Penguncian State Aman (state.todos.lock().unwrap())
    let mut todos = state.todos.lock().unwrap();
    let todo_index = todos.iter().position(|t| t.id == Some(id));

    match todo_index {
        Some(index) => {
            let todo = &mut todos[index];

            if let Some(title) = payload.title {
                todo.title = title;
            }

            if let Some(description) = payload.description {
                todo.description = Some(description);
            }

            if let Some(completed) = payload.completed {
                todo.completed = completed;
            }

            todo.updated_at = Some(chrono::Utc::now());

            let updated_todo = todo.clone();

            Ok(AxumJson(json!({
                "success": true,
                "data": updated_todo,
                "message": "Todo updated successfully"
            })))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// fungsi delete todo
pub async fn delete_todo(
    // 2. Cara Axum mengambil data dari Request.
    Path(id): Path<i32>,
    State(state): State<std::sync::Arc<AppState>>,
) -> Result<AxumJson<serde_json::Value>, StatusCode> {
    // 1. Penguncian State Aman (state.todos.lock().unwrap())
    let mut todos = state.todos.lock().unwrap();
    let initial_len = todos.len();

    todos.retain(|t| t.id != Some(id));

    if todos.len() == initial_len {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(AxumJson(json!({
            "success": true,
            "message": "Todo deleted successfully"
        })))
    }
}
