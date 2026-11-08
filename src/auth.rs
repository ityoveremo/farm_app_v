use argon2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};

use crate::error::AppError;
use crate::models::user::{RegisterRequest, RegisterResponse, UserRole};
use crate::repositories::user_repository::UserRepository;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(AppError::from)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub struct AuthService<'a> {
    user_repo: UserRepository<'a>,
}

impl<'a> AuthService<'a> {
    pub fn new(user_repo: UserRepository<'a>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, AppError> {
        if request.username.is_empty() || request.password.is_empty() {
            return Err(AppError::BadRequest("Username and password are required".to_string()));
        }

        if let Some(_) = self.user_repo.find_by_username(&request.username).await? {
            return Err(AppError::BadRequest("Username already exists".to_string()));
        }

        let password_hash = hash_password(&request.password)?;
        let id = self.user_repo.create(&request.username, &password_hash, UserRole::Viewer).await?;

        Ok(RegisterResponse {
            id,
            username: request.username,
            role: UserRole::Viewer,
        })
    }
}
