use argon2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::models::user::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserRole};
use crate::repositories::user_repository::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

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

pub fn create_token(user_id: i64, username: &str, role: UserRole, secret: &str) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        role: match role {
            UserRole::Admin => "admin",
            UserRole::FieldAgent => "field_agent",
            UserRole::Viewer => "viewer",
        }.to_string(),
        exp: expiration,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;
    
    Ok(token)
}

pub struct AuthService<'a> {
    user_repo: UserRepository<'a>,
    jwt_secret: String,
}

impl<'a> AuthService<'a> {
    pub fn new(user_repo: UserRepository<'a>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
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

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self.user_repo.find_by_username(&request.username)
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid username or password".to_string()))?;

        let valid = verify_password(&request.password, &user.password_hash)?;
        if !valid {
            return Err(AppError::BadRequest("Invalid username or password".to_string()));
        }

        let role = user.role.clone();
        let token = create_token(user.id, &user.username, role, &self.jwt_secret)?;

        Ok(LoginResponse {
            token,
            user_id: user.id,
            username: user.username,
            role: user.role,
        })
    }
}
