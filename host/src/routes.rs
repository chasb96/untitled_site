use axum::{Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};
use crate::health;

pub fn routes() -> Router {
    Router::new()
        .route_service("/", ServeFile::new("./frontend/index.html"))
        .route_service("/frontend_bg.wasm", ServeFile::new("./frontend/frontend_bg.wasm"))
        .route_service("/frontend.js", ServeFile::new("./frontend/frontend.js"))
        .nest_service("/assets/js", ServeDir::new("./frontend/assets/js"))
        .nest_service("/assets/css", ServeDir::new("./frontend/assets/css"))
        .route("/health", get(health::health))
}