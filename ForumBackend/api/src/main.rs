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

async fn get_captcha() -> Json<serde_json::Value> {
    Json(json!({
        "errCode": 0,
        "msg": "success",
        "data": {
            "id": "mock_captcha_id",
            "b64s": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII="
        }
    }))
}

async fn user_login() -> Json<serde_json::Value> {
    Json(json!({
        "errCode": 0,
        "msg": "success",
        "data": {
            "accessToken": "mock_access_token",
            "refreshToken": "mock_refresh_token",
            "email": "test@example.com",
            "userId": "1234567890",
            "nickname": "TestUser",
            "protoUrl": "https://api.dicebear.com/7.x/adventurer/svg?seed=Felix"
        }
    }))
}

async fn generic_mock() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "errCode": 0, "msg": "success", "data": {} }))
}

async fn generic_array_mock() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "errCode": 0, "msg": "success", "data": [] }))
}

async fn mock_article_details() -> Json<serde_json::Value> {
    Json(json!({ "code": 0, "errCode": 0, "msg": "success", "data": {
        "title": "Mock Article",
        "content": "This is a mock article content",
        "tags": "mock,tag",
        "category": "mock_category",
        "cover_url": "https://api.dicebear.com/7.x/adventurer/svg?seed=Felix"
    }}))
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
        .route("/user/get_captcha", post(get_captcha))
        .route("/user/user_login", post(user_login))
        // --- Missing User Endpoints ---
        .route("/user/send_email_code", post(generic_mock))
        .route("/user/user_register", post(generic_mock))
        .route("/user/post_update_password", post(generic_mock))
        .route("/user/post_update_profile_photo", post(generic_mock))
        .route("/user/update_user_info", post(generic_mock))
        .route("/user/get_image_url", post(generic_mock))
        // --- Missing Static Endpoints ---
        .route("/static/get_article_details", post(mock_article_details))
        .route("/static/add_view_count", post(generic_mock))
        .route("/static/get_comment_list", post(generic_array_mock))
        .route("/static/get_comment_son_list", post(generic_array_mock))
        // --- Missing BManager Endpoints ---
        .route("/bmanager/get_oss_key", post(generic_mock))
        .route("/bmanager/update_forum", post(generic_mock))
        .route("/bmanager/push_forum_new", post(generic_mock))
        .route("/bmanager/delete_forum", post(generic_mock))
        .route("/bmanager/push_forum_comment_new", post(generic_mock))
        .route("/bmanager/delete_comment", post(generic_mock))
        .route("/bmanager/get_image_url", post(generic_mock))
        // --- Missing Admin Endpoints ---
        .route("/admin/post_blog_change", post(generic_mock))
        .route("/admin/push_blog_new", post(generic_mock))
        .route("/admin/delete_blog", post(generic_mock))
        .layer(cors);

    let addr = "[::]:8080";
    let listener = TcpListener::bind(addr).await?;
    
    println!("API Gateway listening on {}", addr);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
