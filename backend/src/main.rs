use axum::{
    extract::State,
    routing::get,
    Router,
    response::Json,
};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;

// Import modules
mod models;
mod handlers;
mod routes;
mod services;
mod utils;

use routes::todo_routes;

// State for our application
pub struct AppState {
    pub todos: std::sync::Mutex<Vec<models::todo::Todo>>,
}

async fn hello_world(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello World! RESTful API with Rust",
        "status": "success",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn health_check(State(_state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "todo-list-api"
    }))
}

#[tokio::main]
async fn main() {
    // Initialize application state
    let app_state = Arc::new(AppState {
        todos: std::sync::Mutex::new(vec![]),
    });

    // Create router with routes
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .nest("/api/todos", todo_routes::create_todo_routes())
        .with_state(app_state);

    // Define address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("🚀 Server running on http://localhost:3000");
    println!("📝 Hello World endpoint: GET /");
    println!("❤️  Health check endpoint: GET /health");
    println!("📝 Todo API endpoints:");
    println!("   GET /api/todos - Get all todos");
    println!("   POST /api/todos - Create a new todo");
    println!("   GET /api/todos/:id - Get a specific todo");
    println!("   PUT /api/todos/:id - Update a specific todo");
    println!("   DELETE /api/todos/:id - Delete a specific todo");

    // Start server
    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    println!("Server started successfully!");
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {}", e);
    }
}