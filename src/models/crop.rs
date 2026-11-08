use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Crop {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
}
