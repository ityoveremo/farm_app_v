use argon2::{
    password_hash::{PasswordHash, PasswordVerifier, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequestParts, State},
    http::Request,
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, Header, DecodingKey, EncodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;

use crate::error::AppError;
use crate::models::user::{User, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse, UserRole};
use crate::repositories::user_repository::UserRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user: User,
}

impl CurrentUser {
    pub fn user_id(&self) -> i64 {
        self.user.id
    }
    
    pub fn username(&self) -> &str {
        &self.user.username
    }
    
    pub fn role(&self) -> &UserRole {
        &self.user.role
    }
}

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());
    
    let (mut parts, body) = request.into_parts();
    
    let headers = parts.headers.clone();
    let auth_header = headers.get("Authorization")
        .ok_or(AppError::Unauthorized)?;
    
    let auth_str = auth_header.to_str()
        .map_err(|_| AppError::Unauthorized)?;
    
    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }
    
    let token = &auth_str[7..];
    
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| AppError::Unauthorized)?;
    
    let user_repo = UserRepository::new(&pool);
    let user = user_repo.find_by_id(decoded.claims.sub)
        .await
        .map_err(AppError::from)?
        .ok_or(AppError::Unauthorized)?;
    
    let current_user = CurrentUser { user };
    
    parts.extensions.insert(current_user);
    
    let request = Request::from_parts(parts, body);
    Ok(next.run(request).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<CurrentUser>()
            .cloned()
            .ok_or(AppError::Unauthorized)
    }
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
