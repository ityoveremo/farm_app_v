use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::photo::FarmPhoto;

pub struct PhotoRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PhotoRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(
        &self,
        farm_id: i64,
        filename: &str,
        filepath: &str,
        mime_type: Option<&str>,
        size_bytes: Option<i64>,
        lon: Option<f64>,
        lat: Option<f64>,
    ) -> Result<i64, Error> {
        let row = sqlx::query_scalar!(
            r#"
            INSERT INTO farm_photos (farm_id, filename, filepath, mime_type, size_bytes, lon, lat)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            farm_id,
            filename,
            filepath,
            mime_type,
            size_bytes,
            lon,
            lat
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_by_farm(&self, farm_id: i64) -> Result<Vec<FarmPhoto>, Error> {
        let photos = sqlx::query_as!(
            FarmPhoto,
            r#"
            SELECT id, farm_id, filename, filepath, mime_type, size_bytes, lon, lat
            FROM farm_photos
            WHERE farm_id = $1
            ORDER BY uploaded_at DESC
            "#,
            farm_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(photos)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<FarmPhoto>, Error> {
        let photo = sqlx::query_as!(
            FarmPhoto,
            r#"
            SELECT id, farm_id, filename, filepath, mime_type, size_bytes, lon, lat
            FROM farm_photos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(photo)
    }
}
