use axum::{middleware, routing::{get, post}, Router};
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod auth;
mod error;
mod models;
mod repositories;
mod routes;
mod services;

async fn health_check() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://farmapp:devpassword@localhost:5432/farmapp".to_string());
    
    let pool = PgPool::connect(&database_url).await.unwrap();
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/register", post(routes::auth::register))
        .route("/login", post(routes::auth::login))
        .route("/farms", get(routes::farms::get_farms))
        .route("/farms", post(routes::farms::create_farm).layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware)))
        .route("/crops", get(routes::crops::get_crops))
        .route("/farms/:id/photos", post(routes::photos::upload_photo).layer(middleware::from_fn_with_state(pool.clone(), auth::auth_middleware)))
        .with_state(pool)
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
