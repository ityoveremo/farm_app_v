use axum::{extract::{Multipart, Path, State}, Json};
use image::DynamicImage;
use image::ImageReader;
use image::ImageFormat;
use serde::Serialize;
use sqlx::postgres::PgPool;
use std::io::Cursor;

use crate::auth::CurrentUser;
use crate::error::AppError;
use crate::models::user::UserRole;
use crate::repositories::photo_repository::PhotoRepository;

#[derive(Serialize)]
pub struct PhotoOut {
    pub id: i64,
    pub farm_id: i64,
    pub filename: String,
    pub filepath: String,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}

pub async fn upload_photo(
    current_user: CurrentUser,
    State(pool): State<PgPool>,
    Path(farm_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<PhotoOut>, AppError> {
    match current_user.role() {
        UserRole::Admin | UserRole::FieldAgent => {},
        _ => return Err(AppError::Forbidden),
    }

    let repo = PhotoRepository::new(&pool);
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        let filename = field.file_name()
            .ok_or_else(|| AppError::BadRequest("File has no name".to_string()))?
            .to_string();
        
        let content_type = field.content_type()
            .map(|ct| ct.to_string());
        
        let bytes = field.bytes().await.map_err(|e| {
            AppError::BadRequest(format!("Failed to read file bytes: {}", e))
        })?;
        
        let stripped_bytes = strip_exif(&bytes).await?;
        
        let size_bytes = stripped_bytes.len() as i64;
        
        let filepath = format!("static/uploads/{}", filename);
        
        tokio::fs::write(&filepath, &stripped_bytes).await.map_err(|e| {
            AppError::Internal(format!("Failed to save file: {}", e))
        })?;
        
        let id = repo.insert(
            farm_id,
            &filename,
            &filepath,
            content_type.as_deref(),
            Some(size_bytes),
            None,
            None,
        ).await?;
        
        return Ok(Json(PhotoOut {
            id,
            farm_id,
            filename,
            filepath,
            mime_type: content_type,
            size_bytes: Some(size_bytes),
            lon: None,
            lat: None,
        }));
    }
    
    Err(AppError::BadRequest("No file uploaded".to_string()))
}

async fn strip_exif(bytes: &[u8]) -> Result<Vec<u8>, AppError> {
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| AppError::BadRequest(format!("Failed to read image: {}", e)))?;
    
    match reader.decode() {
        Ok(img) => {
            let mut stripped_bytes = Vec::new();
            let mut cursor = Cursor::new(&mut stripped_bytes);
            match img.write_to(&mut cursor, ImageFormat::Jpeg) {
                Ok(_) => Ok(stripped_bytes),
                Err(_) => Ok(bytes.to_vec()),
            }
        }
        Err(_) => Ok(bytes.to_vec()),
    }
}
