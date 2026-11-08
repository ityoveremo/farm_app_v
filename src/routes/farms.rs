use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use crate::auth::CurrentUser;
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::repositories::farm_repository::FarmRepository;
use crate::services::farm_service::FarmService;

#[derive(Serialize)]
pub struct FarmOut {
    pub id: i64,
    pub name: Option<String>,
    pub town: String,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}

#[derive(Deserialize)]
pub struct NewFarmRequest {
    pub name: Option<String>,
    pub town: String,
    pub lon: f64,
    pub lat: f64,
}

pub async fn get_farms(State(pool): State<PgPool>) -> Result<Json<Vec<FarmOut>>, AppError> {
    let repo = FarmRepository::new(&pool);
    let farms = repo.find_all().await?;
    
    let farm_outs: Vec<FarmOut> = farms
        .into_iter()
        .map(|farm| FarmOut {
            id: farm.id,
            name: farm.name,
            town: farm.town,
            lon: farm.lon,
            lat: farm.lat,
        })
        .collect();
    
    Ok(Json(farm_outs))
}

pub async fn create_farm(
    current_user: CurrentUser,
    State(pool): State<PgPool>,
    Json(body): Json<NewFarmRequest>,
) -> Result<Json<i64>, AppError> {
    match current_user.role() {
        UserRole::Admin | UserRole::FieldAgent => {},
        _ => return Err(AppError::Forbidden),
    }
    
    let repo = FarmRepository::new(&pool);
    let service = FarmService::new(repo);
    let id = service.create_farm(body.name.as_deref(), &body.town, body.lon, body.lat).await?;
    Ok(Json(id))
}
