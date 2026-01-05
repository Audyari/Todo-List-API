// Business logic for todo operations
use crate::models::todo::{CreateTodoRequest, UpdateTodoRequest};

// In a real application, this would interact with a database
// For now, we'll implement the logic that would be used in the handlers

// 1. Validasi Todo
pub fn validate_todo_request(request: &CreateTodoRequest) -> Result<(), String> {
    // 1. Validasi Title
    // 1. Logika Validasi Inti (is_empty, len)
    if request.title.trim().is_empty() {
        return Err("Title is required".to_string());
    }

    // 2. Validasi panjang Title
    // 2. Logika Validasi Inti (len)
    if request.title.len() > 255 {
        return Err("Title must be less than 255 characters".to_string());
    }

    // 3. Validasi Description
    // 3. Logika Validasi Inti (len)
    if let Some(description) = &request.description {
        if description.len() > 1000 {
            return Err("Description must be less than 1000 characters".to_string());
        }
    }

    Ok(())
}

// 2. Validasi Update
pub fn validate_update_request(request: &UpdateTodoRequest) -> Result<(), String> {
    if let Some(title) = &request.title {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 255 {
            return Err("Title must be less than 255 characters".to_string());
        }
    }

    if let Some(description) = &request.description {
        if description.len() > 1000 {
            return Err("Description must be less than 1000 characters".to_string());
        }
    }

    Ok(())
}

// Additional business logic functions would go here
// For example: filtering, sorting, pagination, etc.
