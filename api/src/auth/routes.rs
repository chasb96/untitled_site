use axum::{routing::post, Router};
use super::web::routes::{login, sign_up};

pub trait AuthRouter {
    fn register_auth_routes(self) -> Self;
}

impl AuthRouter for Router {
    fn register_auth_routes(self) -> Self {
        self.route("/sign_up", post(sign_up))
            .route("/login", post(login))
    }
}