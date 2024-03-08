use axum::{routing::{get, post}, Router};
use crate::{health, logging::LogLayer};
use crate::web::routes::create_user;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/create_user", post(create_user))
        .layer(LogLayer::new())
}