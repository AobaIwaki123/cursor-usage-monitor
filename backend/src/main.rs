mod handlers;
mod models;
mod services;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(handlers::health::health_check))
        .route("/api/upload", post(handlers::upload::upload_csv))
        .route("/api/upload/append", post(handlers::upload::append_csv))
        .route("/api/stats/comprehensive", get(handlers::stats::comprehensive_stats))
        .layer(CorsLayer::permissive());

    // Run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}