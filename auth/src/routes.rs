use axum::{routing::{get, post}, Router};
use crate::{health, logging::LogLayer, web::routes::authenticate};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/authenticate", post(authenticate))
        .layer(LogLayer::new())
}