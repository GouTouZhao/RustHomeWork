use axum::{
    routing::{get, post},
    Router, Json,
};
use common::config::load_config;
use serde_json::json;
use tokio::net::TcpListener;

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok"
    }))
}

// User Handlers
async fn login() -> Json<serde_json::Value> {
    // In reality, this would connect to the User gRPC service
    Json(json!({ "code": 0, "msg": "success", "data": { "token": "test_token" } }))
}

// BManager Handlers
async fn get_articles() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "msg": "success", "data": [] }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = load_config()?;

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/user/login", post(login))
        .route("/api/v1/forum/articles", get(get_articles));

    let addr = "[::]:8080";
    let listener = TcpListener::bind(addr).await?;
    
    println!("API Gateway listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
