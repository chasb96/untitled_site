use axum::{routing::get, Router};
use super::web::routes::{get_by_id, get_by_username};

pub trait UsersRouter {
    fn register_user_routes(self) -> Self;
}

impl UsersRouter for Router {
    fn register_user_routes(self) -> Self {
        self.route("/users/:id", get(get_by_id))
            .route("/users/@:username", get(get_by_username))
    }
}