use axum::{Router, routing::get};
use crate::{health, logging::LogLayer};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .layer(LogLayer::new())
}