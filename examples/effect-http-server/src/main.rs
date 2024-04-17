use axum::{Router, extract::Json, routing::get};
use serde::{Deserialize, Serialize};
use wae_effect::{AlgebraicEffect, EffectError, Effectful};

#[derive(Debug, Clone, Deserialize)]
struct AppConfig {
    app_name: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
}

#[derive(Debug, Clone, Serialize)]
struct HealthResponse {
    status: String,
    app_name: String,
    version: String,
}

#[derive(Debug, Clone, Serialize)]
struct UserResponse {
    user: User,
    requested_by: String,
}

async fn health(ae: Effectful) -> Result<Json<HealthResponse>, EffectError> {
    let config: AppConfig = ae.get("config")?;
    Ok(Json(HealthResponse { status: "ok".to_string(), app_name: config.app_name, version: config.version }))
}

async fn get_user(ae: Effectful) -> Result<Json<UserResponse>, EffectError> {
    let user: User = ae.get("auth")?;
    let config: AppConfig = ae.get("config")?;

    Ok(Json(UserResponse { user, requested_by: config.app_name }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig { app_name: "WAE Effect HTTP Server".to_string(), version: "0.1.0".to_string() };

    let demo_user =
        User { id: "user-001".to_string(), username: "demo_user".to_string(), email: "demo@example.com".to_string() };

    let deps = AlgebraicEffect::new().with("config", config).with("auth", demo_user).build();

    let app = Router::new().route("/health", get(health)).route("/api/user", get(get_user)).with_state(deps);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://localhost:3000");
    println!("Endpoints:");
    println!("  GET  /health    - 健康检查");
    println!("  GET  /api/user  - 获取当前用户");

    axum::serve(listener, app).await?;

    Ok(())
}
