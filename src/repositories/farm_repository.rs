use sqlx::postgres::PgPool;
use sqlx::Error;

pub struct Farm {
    pub id: i64,
    pub name: Option<String>,
    pub town: String,
}

pub struct FarmRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> FarmRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, name: &str, town: &str, lon: f64, lat: f64) -> Result<i64, Error> {
        let row = sqlx::query_scalar!(
            r#"
            INSERT INTO farms (name, town, location)
            VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326))
            RETURNING id
            "#,
            name,
            town,
            lon,
            lat
        )
        .fetch_one(self.pool)
        .await?;

        Ok(row)
    }

    pub async fn find_all(&self) -> Result<Vec<Farm>, Error> {
        let farms = sqlx::query_as!(
            Farm,
            r#"
            SELECT id, name, town FROM farms
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(farms)
    }
}
