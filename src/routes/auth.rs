use axum::{extract::State, Json};
use sqlx::postgres::PgPool;

use crate::auth::AuthService;
use crate::error::AppError;
use crate::models::user::{RegisterRequest, RegisterResponse};
use crate::repositories::user_repository::UserRepository;

pub async fn register(
    State(pool): State<PgPool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let user_repo = UserRepository::new(&pool);
    let auth_service = AuthService::new(user_repo);
    let response = auth_service.register(request).await?;
    Ok(Json(response))
}
