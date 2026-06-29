use axum::{
    routing::{get, post},
    Router, Json,
};
use tower_http::cors::{Any, CorsLayer};
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

async fn get_user_photo_compre() -> Json<serde_json::Value> {
    Json(json!({
        "code": 0,
        "msg": "success",
        "data": {
            "avatar_url": "https://api.dicebear.com/7.x/adventurer/svg?seed=Felix"
        }
    }))
}

async fn refresh_token() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "msg": "success", "data": { "token": "new_test_token" } }))
}

// BManager/Static Handlers
async fn get_articles() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "msg": "success", "data": [] }))
}

async fn get_article_list() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "msg": "success", "data": [] }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = load_config()?;

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<axum::http::HeaderValue>().unwrap(),
        ])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/user/login", post(login))
        .route("/api/v1/forum/articles", get(get_articles))
        // Add mocked endpoints the frontend expects locally:
        .route("/user/get_user_photo_compre", post(get_user_photo_compre))
        .route("/user/refresh_token", post(refresh_token))
        .route("/static/get_article_list", post(get_article_list))
        .layer(cors);

    let addr = "[::]:8080";
    let listener = TcpListener::bind(addr).await?;
    
    println!("API Gateway listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
