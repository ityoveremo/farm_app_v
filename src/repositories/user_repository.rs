use sqlx::{postgres::PgPool, Error};

use crate::models::user::{User, UserRole};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, password_hash, role::text as "role", created_at, updated_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            password_hash: r.password_hash,
            role: match r.role.as_deref() {
                Some("admin") => UserRole::Admin,
                Some("field_agent") => UserRole::FieldAgent,
                _ => UserRole::Viewer,
            },
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, password_hash, role::text as "role", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            password_hash: r.password_hash,
            role: match r.role.as_deref() {
                Some("admin") => UserRole::Admin,
                Some("field_agent") => UserRole::FieldAgent,
                _ => UserRole::Viewer,
            },
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn create(&self, username: &str, password_hash: &str, role: UserRole) -> Result<i64, Error> {
        let role_str = match role {
            UserRole::Admin => "admin",
            UserRole::FieldAgent => "field_agent",
            UserRole::Viewer => "viewer",
        };
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, $3::user_role) RETURNING id",
        )
        .bind(username)
        .bind(password_hash)
        .bind(role_str)
        .fetch_one(self.pool)
        .await?;
        Ok(id)
    }
}
