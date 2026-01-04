use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::json;
use std::net::SocketAddr;

async fn hello_world() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Hello World! RESTful API with Rust",
        "status": "success",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "todo-list-api"
    }))
}

#[tokio::main]
async fn main() {
    // Create router
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    // Define address
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    
    println!("🚀 Server running on http://localhost:3000");
    println!("📝 Hello World endpoint: GET /");
    println!("❤️  Health check endpoint: GET /health");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}