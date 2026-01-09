
use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::CorsLayer;

// Import modules
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

use routes::{task_routes, user_routes};

use crate::utils::database::DatabaseConnection;

// Configuration constants
const SERVER_HOST: &str = "0.0.0.0";
const SERVER_PORT: u16 = 3000;
const CORS_MAX_AGE_SECONDS: u64 = 86400; // 24 hours

// CORS Origins
const CORS_ORIGINS: &[&str] = &[
    "http://localhost:5173", // Vite default port
    "http://localhost:3000", // Backend port (for testing)
    "http://127.0.0.1:5173",
    "http://127.0.0.1:3000",
];

// 1. Manajemen State Bersama (AppState + Arc)
// Ini adalah memori bersama aplikasi.
pub struct AppState {
    pub db: DatabaseConnection,
}

// 3. Penanganan Masalah (Label Kesalahan)
#[derive(Debug)]
enum ServerError {
    DatabaseConnection(String),
    BindAddress(String),
    Server(String),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::DatabaseConnection(msg) => write!(f, "Database connection error: {}", msg),
            ServerError::BindAddress(msg) => write!(f, "Bind address error: {}", msg),
            ServerError::Server(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl std::error::Error for ServerError {}

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

//2. Sistem Navigasi (Saraf Penghubung)
fn create_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check))
        .nest("/api/users", user_routes::create_user_routes())
        .nest("/api/tasks", task_routes::create_task_routes())
        .with_state(state)
        // Add CORS layer to allow requests from frontend
        .layer(
            CorsLayer::new()
                .allow_origin(
                    CORS_ORIGINS
                        .iter()
                        .map(|origin| origin.parse().unwrap())
                        .collect::<Vec<_>>(),
                )
                .allow_methods(tower_http::cors::AllowMethods::any())
                .allow_headers(tower_http::cors::AllowHeaders::any())
                .max_age(Duration::from_secs(CORS_MAX_AGE_SECONDS)),
        )
}

// Print API documentation to console
fn print_api_docs() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    TODO-LIST API SERVER                      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║                                                              ║");
    println!(
        "║  🚀 Server running on http://{}:{}                    ║",
        SERVER_HOST, SERVER_PORT
    );
    println!("║                                                              ║");
    println!("║  📝 Endpoints:                                             ║");
    println!("║     GET    /              - Hello World                    ║");
    println!("║     GET    /health        - Health check                   ║");
    println!("║                                                              ║");
    println!("║  👥 User API:                                              ║");
    println!("║     GET    /api/users              - Get all users         ║");
    println!("║     POST   /api/users              - Create a new user     ║");
    println!("║     POST   /api/users/register     - Register a new user   ║");
    println!("║     POST   /api/users/login        - Login a user          ║");
    println!("║     GET    /api/users/:id          - Get a specific user   ║");
    println!("║     PUT    /api/users/:id          - Update a specific user║");
    println!("║     DELETE /api/users/:id          - Delete a specific user║");
    println!("║                                                              ║");
    println!("║  ✅ Task API:                                              ║");
    println!("║     GET    /api/tasks              - Get all tasks         ║");
    println!("║     POST   /api/tasks              - Create a new task     ║");
    println!("║     GET    /api/tasks/:id          - Get a specific task   ║");
    println!("║     PUT    /api/tasks/:id          - Update a specific task║");
    println!("║     DELETE /api/tasks/:id          - Delete a specific task║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("Server failed to start: {}", e);
        std::process::exit(1);
    }
}

async fn run_server() -> Result<(), ServerError> {
    // Initialize database connection
    let db = DatabaseConnection::new()
        .await
        .map_err(|e| ServerError::DatabaseConnection(e.to_string()))?;

    let app_state = Arc::new(AppState { db });

    // Create the application with state
    let app = create_app(app_state);

    // Define server address
    let addr = SocketAddr::from((
        SERVER_HOST
            .parse::<std::net::Ipv4Addr>()
            .expect("Invalid host"),
        SERVER_PORT,
    ));

    // Print API documentation
    print_api_docs();

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| ServerError::BindAddress(format!("{}: {}", addr, e)))?;

    println!("Server started successfully!");
    axum::serve(listener, app)
        .await
        .map_err(|e| ServerError::Server(e.to_string()))?;

    Ok(())
}
