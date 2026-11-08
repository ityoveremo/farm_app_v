use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::crop::Crop;
use crate::repositories::crop_repository::CropRepository;

pub async fn get_crops(State(pool): State<PgPool>) -> Result<Json<Vec<Crop>>, AppError> {
    let repo = CropRepository::new(&pool);
    let crops = repo.find_all().await?;
    Ok(Json(crops))
}
