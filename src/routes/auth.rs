use axum::{extract::State, Json};
use sqlx::postgres::PgPool;
use std::env;

use crate::auth::AuthService;
use crate::error::AppError;
use crate::models::user::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::repositories::user_repository::UserRepository;

pub async fn register(
    State(pool): State<PgPool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let user_repo = UserRepository::new(&pool);
    let auth_service = AuthService::new(user_repo, jwt_secret);
    let response = auth_service.register(request).await?;
    Ok(Json(response))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    let user_repo = UserRepository::new(&pool);
    let auth_service = AuthService::new(user_repo, jwt_secret);
    let response = auth_service.login(request).await?;
    Ok(Json(response))
}
