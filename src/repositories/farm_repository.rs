use sqlx::postgres::PgPool;
use sqlx::Error;

pub struct Farm {
    pub id: i64,
    pub name: Option<String>,
    pub town: String,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub owner_id: Option<i64>,
}

pub struct FarmRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> FarmRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, name: &str, town: &str, lon: f64, lat: f64, owner_id: i64) -> Result<i64, Error> {
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

        Ok(row)
    }

    pub async fn find_all(&self) -> Result<Vec<Farm>, Error> {
        let farms = sqlx::query_as!(
            Farm,
            r#"
            SELECT id, name, town, ST_X(location) as lon, ST_Y(location) as lat, owner_id FROM farms
            "#
        )
        .fetch_all(self.pool)
        .await?;

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
