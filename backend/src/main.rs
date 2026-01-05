use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;

// Import modules
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use routes::{todo_routes, user_routes, task_routes};

use crate::utils::database::DatabaseConnection;

// 1. Manajemen State Bersama (AppState + Arc)
// Ini adalah memori bersama aplikasi.
pub struct AppState {
    pub db: DatabaseConnection,
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
    // Initialize database connection
    let db = match DatabaseConnection::new().await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    // 1. Manajemen State Bersama (AppState + Arc)
    // Ini adalah memori bersama aplikasi.
    let app_state = Arc::new(AppState { db });

    // 2. Konfigurasi Router & Dependency Injection (Router + with_state)
    // Ini adalah sistem saraf yang menghubungkan permintaan (request) ke logika (handler).
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .nest("/api/todos", todo_routes::create_todo_routes())
        .nest("/api/users", user_routes::create_user_routes())
        .nest("/api/tasks", task_routes::create_task_routes())
        .with_state(app_state);

    // 3. Define address
    // Ini adalah alamat di mana server akan berjalan.
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
    println!("👥 User API endpoints:");
    println!("   GET /api/users - Get all users");
    println!("   POST /api/users - Create a new user");
    println!("   GET /api/users/:id - Get a specific user");
    println!("   PUT /api/users/:id - Update a specific user");
    println!("   DELETE /api/users/:id - Delete a specific user");
    println!("✅ Task API endpoints:");
    println!("   GET /api/tasks - Get all tasks");
    println!("   POST /api/tasks - Create a new task");
    println!("   GET /api/tasks/:id - Get a specific task");
    println!("   PUT /api/tasks/:id - Update a specific task");
    println!("   DELETE /api/tasks/:id - Delete a specific task");

    // 4. Server Runtime (TcpListener + axum::serve)
    // Ini adalah server yang akan menerima permintaan dan mengirimkan respons.
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
