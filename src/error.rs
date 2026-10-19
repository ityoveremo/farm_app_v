use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Database(SqlxError),
}

impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        AppError::Database(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
        }
    }
}
