use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::postgres::PgPool;

use crate::repositories::farm_repository::FarmRepository;

#[derive(Serialize)]
pub struct FarmOut {
    pub id: i64,
    pub name: Option<String>,
    pub town: String,
}

pub async fn get_farms(State(pool): State<PgPool>) -> Json<Vec<FarmOut>> {
    let repo = FarmRepository::new(&pool);
    let farms = repo.find_all().await.unwrap();
    
    let farm_outs: Vec<FarmOut> = farms
        .into_iter()
        .map(|farm| FarmOut {
            id: farm.id,
            name: farm.name,
            town: farm.town,
        })
        .collect();
    
    Json(farm_outs)
}
