use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::models::farm::FarmStatus;

pub struct Farm {
    pub id: i64,
    pub name: Option<String>,
    pub town: String,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub owner_id: Option<i64>,
    pub boundary: Option<String>,
    pub area_hectares: Option<f64>,
    pub status: FarmStatus,
    pub approved_by: Option<i64>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct FarmRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> FarmRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, name: &str, town: &str, lon: f64, lat: f64, owner_id: i64, boundary_wkt: Option<&str>) -> Result<i64, Error> {
        let row: i64 = if let Some(boundary) = boundary_wkt {
            let row = sqlx::query_scalar!(
                r#"
                INSERT INTO farms (name, town, location, owner_id, boundary, area_hectares)
                VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326), $5, 
                        ST_GeomFromText($6, 4326), 
                        ST_Area(ST_GeomFromText($6, 4326)::geography) / 10000.0)
                RETURNING id
                "#,
                name,
                town,
                lon,
                lat,
                owner_id,
                boundary
            )
            .fetch_one(self.pool)
            .await?;
            row
        } else {
            let row = sqlx::query_scalar!(
                r#"
                INSERT INTO farms (name, town, location, owner_id)
                VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326), $5)
                RETURNING id
                "#,
                name,
                town,
                lon,
                lat,
                owner_id
            )
            .fetch_one(self.pool)
            .await?;
            row
        };

        Ok(row)
    }

    pub async fn find_all(&self) -> Result<Vec<Farm>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, town, ST_X(location) as lon, ST_Y(location) as lat, owner_id, 
                   ST_AsText(boundary) as boundary, area_hectares, status as "status: FarmStatus", 
                   approved_by, approved_at FROM farms
            "#
        )
        .fetch_all(self.pool)
        .await?;

        let farms = rows.into_iter().map(|row| Farm {
            id: row.id,
            name: row.name,
            town: row.town,
            lon: row.lon,
            lat: row.lat,
            owner_id: row.owner_id,
            boundary: row.boundary,
            area_hectares: row.area_hectares,
            status: row.status,
            approved_by: row.approved_by,
            approved_at: row.approved_at.map(|dt| DateTime::<Utc>::from(dt)),
        }).collect();

        Ok(farms)
    }

    pub async fn find_pending(&self) -> Result<Vec<Farm>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, town, ST_X(location) as lon, ST_Y(location) as lat, owner_id, 
                   ST_AsText(boundary) as boundary, area_hectares, status as "status: FarmStatus", 
                   approved_by, approved_at FROM farms
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(self.pool)
        .await?;

        let farms = rows.into_iter().map(|row| Farm {
            id: row.id,
            name: row.name,
            town: row.town,
            lon: row.lon,
            lat: row.lat,
            owner_id: row.owner_id,
            boundary: row.boundary,
            area_hectares: row.area_hectares,
            status: row.status,
            approved_by: row.approved_by,
            approved_at: row.approved_at.map(|dt| DateTime::<Utc>::from(dt)),
        }).collect();

        Ok(farms)
    }

    pub async fn insert_crop_assignments(&self, farm_id: i64, crop_ids: &[i64]) -> Result<(), Error> {
        if crop_ids.is_empty() {
            return Ok(());
        }
        
        let query = format!(
            "INSERT INTO farm_crops (farm_id, crop_id) SELECT $1, unnest($2::BIGINT[])"
        );
        
        sqlx::query(&query)
            .bind(farm_id)
            .bind(crop_ids)
            .execute(self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn find_crops_by_farm(&self, farm_id: i64) -> Result<Vec<i64>, Error> {
        let crop_ids = sqlx::query_scalar!(
            r#"
            SELECT crop_id FROM farm_crops WHERE farm_id = $1
            "#,
            farm_id
        )
        .fetch_all(self.pool)
        .await?;
        
        Ok(crop_ids)
    }
}
