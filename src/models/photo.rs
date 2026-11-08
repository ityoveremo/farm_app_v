use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FarmPhoto {
    pub id: i64,
    pub farm_id: i64,
    pub filename: String,
    pub filepath: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}
