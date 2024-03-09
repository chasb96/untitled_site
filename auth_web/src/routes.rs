use axum::{routing::{get, post}, Router};
use crate::{health, logging::LogLayer, web::routes::{authenticate, sign_up}};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/sign_up", post(sign_up))
        .route("/authenticate", post(authenticate))
        .layer(LogLayer::new())
}