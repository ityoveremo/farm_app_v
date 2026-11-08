use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::crop::Crop;

pub struct CropRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> CropRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Crop>, Error> {
        let crops = sqlx::query_as!(
            Crop,
            r#"
            SELECT id, name, description, category FROM crops
            ORDER BY name
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(crops)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Crop>, Error> {
        let crop = sqlx::query_as!(
            Crop,
            r#"
            SELECT id, name, description, category FROM crops WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(crop)
    }
}
