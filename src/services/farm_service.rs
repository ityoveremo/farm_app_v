use crate::error::AppError;
use crate::repositories::farm_repository::FarmRepository;

pub struct FarmService<'a> {
    repo: FarmRepository<'a>,
}

impl<'a> FarmService<'a> {
    pub fn new(repo: FarmRepository<'a>) -> Self {
        Self { repo }
    }

    pub async fn create_farm(
        &self,
        name: Option<&str>,
        town: &str,
        lon: f64,
        lat: f64,
    ) -> Result<i64, AppError> {
        if lon < -180.0 || lon > 180.0 || lat < -90.0 || lat > 90.0 {
            return Err(AppError::BadRequest(
                "Invalid coordinates: longitude must be between -180 and 180, latitude must be between -90 and 90".to_string(),
            ));
        }

        let name_str = name.unwrap_or("");
        let id = self.repo.insert(name_str, town, lon, lat).await?;
        Ok(id)
    }
}
