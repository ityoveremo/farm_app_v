use axum::{routing::get, Router};
use std::net::SocketAddr;

mod error;
mod repositories;
mod services;

async fn health_check() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
